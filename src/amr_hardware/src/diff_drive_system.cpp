#include "amr_hardware/diff_drive_system.hpp"

#include <fcntl.h>
#include <linux/can.h>
#include <linux/can/raw.h>
#include <net/if.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <unistd.h>

#include <algorithm>
#include <cerrno>
#include <cmath>
#include <cstring>
#include <limits>
#include <stdexcept>
#include <thread>

#include "hardware_interface/types/hardware_interface_type_values.hpp"
#include "pluginlib/class_list_macros.hpp"
#include "rclcpp/rclcpp.hpp"

namespace
{
constexpr double kTwoPi = 2.0 * M_PI;
constexpr uint16_t kControlWord = 0x6040;
constexpr uint16_t kModeOfOperation = 0x6060;
constexpr uint16_t kTargetVelocity = 0x60FF;
constexpr uint16_t kTpdo0Communication = 0x1800;
constexpr uint16_t kTpdo0Mapping = 0x1A00;
constexpr uint16_t kStoreParameters = 0x2010;

double clamp(double value, double lower, double upper)
{
  return std::max(lower, std::min(upper, value));
}

int16_t clamp_to_i16(int value)
{
  return static_cast<int16_t>(
    std::max(
      static_cast<int>(std::numeric_limits<int16_t>::min()),
      std::min(static_cast<int>(std::numeric_limits<int16_t>::max()), value)));
}

int16_t int16_from_le(uint8_t low, uint8_t high)
{
  return static_cast<int16_t>(static_cast<uint16_t>(low) | (static_cast<uint16_t>(high) << 8));
}

void put_i16_le(std::vector<uint8_t> & data, std::size_t offset, int16_t value)
{
  const auto raw = static_cast<uint16_t>(value);
  data[offset] = static_cast<uint8_t>(raw & 0xFF);
  data[offset + 1] = static_cast<uint8_t>((raw >> 8) & 0xFF);
}
}  // namespace

namespace amr_hardware
{

SocketCanBus::SocketCanBus(const std::string & interface_name)
{
  fd_ = ::socket(PF_CAN, SOCK_RAW | SOCK_NONBLOCK, CAN_RAW);
  if (fd_ < 0) {
    throw std::runtime_error("failed to open CAN socket: " + std::string(std::strerror(errno)));
  }

  ifreq ifr {};
  std::strncpy(ifr.ifr_name, interface_name.c_str(), IFNAMSIZ - 1);
  if (::ioctl(fd_, SIOCGIFINDEX, &ifr) < 0) {
    close();
    throw std::runtime_error(
      "failed to find CAN interface " + interface_name + ": " + std::strerror(errno));
  }

  sockaddr_can address {};
  address.can_family = AF_CAN;
  address.can_ifindex = ifr.ifr_ifindex;
  if (::bind(fd_, reinterpret_cast<sockaddr *>(&address), sizeof(address)) < 0) {
    close();
    throw std::runtime_error(
      "failed to bind CAN interface " + interface_name + ": " + std::strerror(errno));
  }
}

SocketCanBus::~SocketCanBus()
{
  close();
}

void SocketCanBus::send(uint32_t can_id, const std::vector<uint8_t> & data)
{
  if (data.size() > CAN_MAX_DLEN) {
    throw std::runtime_error("classic CAN payload cannot exceed 8 bytes");
  }

  can_frame frame {};
  frame.can_id = can_id;
  frame.can_dlc = static_cast<__u8>(data.size());
  std::copy(data.begin(), data.end(), frame.data);

  const auto written = ::write(fd_, &frame, sizeof(frame));
  if (written != static_cast<ssize_t>(sizeof(frame))) {
    throw std::runtime_error("CAN write failed: " + std::string(std::strerror(errno)));
  }
}

std::optional<std::pair<uint32_t, std::vector<uint8_t>>> SocketCanBus::receive()
{
  can_frame frame {};
  const auto count = ::read(fd_, &frame, sizeof(frame));
  if (count < 0) {
    if (errno == EAGAIN || errno == EWOULDBLOCK) {
      return std::nullopt;
    }
    throw std::runtime_error("CAN read failed: " + std::string(std::strerror(errno)));
  }

  if (count != static_cast<ssize_t>(sizeof(frame))) {
    return std::nullopt;
  }

  const uint32_t can_id = frame.can_id & CAN_EFF_MASK;
  return std::pair<uint32_t, std::vector<uint8_t>> {
    can_id, std::vector<uint8_t>(frame.data, frame.data + frame.can_dlc)};
}

void SocketCanBus::close()
{
  if (fd_ >= 0) {
    ::close(fd_);
    fd_ = -1;
  }
}

hardware_interface::CallbackReturn DiffDriveSystem::on_init(
  const hardware_interface::HardwareComponentInterfaceParams & params)
{
  if (hardware_interface::SystemInterface::on_init(params) !=
    hardware_interface::CallbackReturn::SUCCESS)
  {
    return hardware_interface::CallbackReturn::ERROR;
  }

  if (!parse_parameters() || !validate_joint_interfaces()) {
    return hardware_interface::CallbackReturn::ERROR;
  }

  return hardware_interface::CallbackReturn::SUCCESS;
}

std::vector<hardware_interface::StateInterface> DiffDriveSystem::export_state_interfaces()
{
  std::vector<hardware_interface::StateInterface> state_interfaces;
  state_interfaces.emplace_back(
    left_wheel_name_, hardware_interface::HW_IF_POSITION, &hw_positions_[0]);
  state_interfaces.emplace_back(
    left_wheel_name_, hardware_interface::HW_IF_VELOCITY, &hw_velocities_[0]);
  state_interfaces.emplace_back(
    right_wheel_name_, hardware_interface::HW_IF_POSITION, &hw_positions_[1]);
  state_interfaces.emplace_back(
    right_wheel_name_, hardware_interface::HW_IF_VELOCITY, &hw_velocities_[1]);
  return state_interfaces;
}

std::vector<hardware_interface::CommandInterface> DiffDriveSystem::export_command_interfaces()
{
  std::vector<hardware_interface::CommandInterface> command_interfaces;
  command_interfaces.emplace_back(
    left_wheel_name_, hardware_interface::HW_IF_VELOCITY, &hw_commands_[0]);
  command_interfaces.emplace_back(
    right_wheel_name_, hardware_interface::HW_IF_VELOCITY, &hw_commands_[1]);
  return command_interfaces;
}

hardware_interface::CallbackReturn DiffDriveSystem::on_configure(
  const rclcpp_lifecycle::State &)
{
  hw_commands_[0] = 0.0;
  hw_commands_[1] = 0.0;
  hw_positions_[0] = 0.0;
  hw_positions_[1] = 0.0;
  hw_velocities_[0] = 0.0;
  hw_velocities_[1] = 0.0;

  if (!use_can_) {
    RCLCPP_WARN(logger_, "use_can is false; hardware interface will run in open-loop mode");
    return hardware_interface::CallbackReturn::SUCCESS;
  }

  try {
    can_bus_ = std::make_unique<SocketCanBus>(can_interface_);
  } catch (const std::exception & error) {
    RCLCPP_ERROR(logger_, "failed to configure CAN interface: %s", error.what());
    return hardware_interface::CallbackReturn::ERROR;
  }

  RCLCPP_INFO(logger_, "opened SocketCAN interface %s", can_interface_.c_str());
  return hardware_interface::CallbackReturn::SUCCESS;
}

hardware_interface::CallbackReturn DiffDriveSystem::on_activate(
  const rclcpp_lifecycle::State &)
{
  hw_commands_[0] = 0.0;
  hw_commands_[1] = 0.0;
  hw_velocities_[0] = 0.0;
  hw_velocities_[1] = 0.0;

  try {
    initialize_drive();
    send_wheel_command(0.0, 0.0);
  } catch (const std::exception & error) {
    RCLCPP_ERROR(logger_, "failed to activate CANopen drive: %s", error.what());
    return hardware_interface::CallbackReturn::ERROR;
  }

  return hardware_interface::CallbackReturn::SUCCESS;
}

hardware_interface::CallbackReturn DiffDriveSystem::on_deactivate(
  const rclcpp_lifecycle::State &)
{
  try {
    send_wheel_command(0.0, 0.0);
  } catch (const std::exception & error) {
    RCLCPP_WARN(logger_, "failed to send deactivation stop command: %s", error.what());
  }
  try {
    sdo_write_2(kControlWord, 0x00, 0x0000);
  } catch (const std::exception & error) {
    RCLCPP_WARN(logger_, "failed to disable drive: %s", error.what());
  }
  can_bus_.reset();
  return hardware_interface::CallbackReturn::SUCCESS;
}

hardware_interface::return_type DiffDriveSystem::read(
  const rclcpp::Time & time, const rclcpp::Duration & period)
{
  const double dt = period.seconds();
  if (dt <= 0.0) {
    return hardware_interface::return_type::OK;
  }

  (void)time;
  auto measured = read_feedback_radps();
  if (measured) {
    hw_velocities_[0] = measured->first;
    hw_velocities_[1] = measured->second;
  } else if (!can_bus_) {
    hw_velocities_[0] = hw_commands_[0];
    hw_velocities_[1] = hw_commands_[1];
  }

  hw_positions_[0] += hw_velocities_[0] * dt;
  hw_positions_[1] += hw_velocities_[1] * dt;
  return hardware_interface::return_type::OK;
}

hardware_interface::return_type DiffDriveSystem::write(
  const rclcpp::Time &, const rclcpp::Duration &)
{
  const double left = clamp(hw_commands_[0], -max_wheel_velocity_radps_, max_wheel_velocity_radps_);
  const double right =
    clamp(hw_commands_[1], -max_wheel_velocity_radps_, max_wheel_velocity_radps_);

  try {
    send_wheel_command(left, right);
  } catch (const std::exception & error) {
    RCLCPP_ERROR(logger_, "motor command failed: %s", error.what());
    return hardware_interface::return_type::ERROR;
  }
  return hardware_interface::return_type::OK;
}

bool DiffDriveSystem::parse_parameters()
{
  if (info_.joints.size() != 2) {
    RCLCPP_ERROR(logger_, "expected exactly 2 wheel joints, got %zu", info_.joints.size());
    return false;
  }

  left_wheel_name_ = param_as_string("left_wheel_name", info_.joints[0].name);
  right_wheel_name_ = param_as_string("right_wheel_name", info_.joints[1].name);
  can_interface_ = param_as_string("can_interface", can_interface_);
  use_can_ = param_as_bool("use_can", use_can_);
  configure_tpdo_ = param_as_bool("configure_tpdo", configure_tpdo_);
  node_id_ = param_as_int("node_id", node_id_);
  left_command_sign_ = param_as_int("left_command_sign", left_command_sign_);
  right_command_sign_ = param_as_int("right_command_sign", right_command_sign_);
  max_wheel_velocity_radps_ =
    param_as_double("max_wheel_velocity_radps", max_wheel_velocity_radps_);
  feedback_sign_left_ = param_as_double("feedback_sign_left", feedback_sign_left_);
  feedback_sign_right_ = param_as_double("feedback_sign_right", feedback_sign_right_);

  if (node_id_ < 1 || node_id_ > 127) {
    RCLCPP_ERROR(logger_, "node_id must be in the CANopen range 1-127");
    return false;
  }
  if (max_wheel_velocity_radps_ <= 0.0) {
    RCLCPP_ERROR(logger_, "max_wheel_velocity_radps must be positive");
    return false;
  }
  return true;
}

void DiffDriveSystem::initialize_drive()
{
  if (!can_bus_) {
    return;
  }

  if (configure_tpdo_) {
    send_nmt(0x80);  // Enter pre-operational before PDO mapping.
    std::this_thread::sleep_for(std::chrono::milliseconds(50));
    configure_speed_feedback_tpdo();
  }

  send_nmt(0x01);  // Start remote node.
  std::this_thread::sleep_for(std::chrono::milliseconds(100));

  sdo_write_2(kControlWord, 0x00, 0x0080);  // Clear fault.
  std::this_thread::sleep_for(std::chrono::milliseconds(100));
  sdo_write_1(kModeOfOperation, 0x00, 0x03);  // Profile velocity mode.
  std::this_thread::sleep_for(std::chrono::milliseconds(100));
  sdo_write_2(kControlWord, 0x00, 0x0006);
  std::this_thread::sleep_for(std::chrono::milliseconds(100));
  sdo_write_2(kControlWord, 0x00, 0x0007);
  std::this_thread::sleep_for(std::chrono::milliseconds(100));
  sdo_write_2(kControlWord, 0x00, 0x000F);
  std::this_thread::sleep_for(std::chrono::milliseconds(100));
}

void DiffDriveSystem::configure_speed_feedback_tpdo()
{
  sdo_write_1(kTpdo0Mapping, 0x00, 0x00);
  std::this_thread::sleep_for(std::chrono::milliseconds(20));
  sdo_write_4(kTpdo0Mapping, 0x01, 0x606C0320);
  std::this_thread::sleep_for(std::chrono::milliseconds(20));
  sdo_write_1(kTpdo0Communication, 0x02, 0xFE);
  std::this_thread::sleep_for(std::chrono::milliseconds(20));
  sdo_write_1(kTpdo0Mapping, 0x00, 0x01);
  std::this_thread::sleep_for(std::chrono::milliseconds(20));
  sdo_write_2(kStoreParameters, 0x00, 0x0001);
  std::this_thread::sleep_for(std::chrono::milliseconds(50));
}

void DiffDriveSystem::send_nmt(uint8_t command)
{
  if (!can_bus_) {
    return;
  }
  can_bus_->send(0x000, {command, static_cast<uint8_t>(node_id_)});
}

void DiffDriveSystem::sdo_write_1(uint16_t index, uint8_t subindex, uint8_t value)
{
  if (!can_bus_) {
    return;
  }
  can_bus_->send(
    0x600 + static_cast<uint32_t>(node_id_),
    {0x2F, static_cast<uint8_t>(index & 0xFF), static_cast<uint8_t>((index >> 8) & 0xFF),
      subindex, value, 0x00, 0x00, 0x00});
}

void DiffDriveSystem::sdo_write_2(uint16_t index, uint8_t subindex, uint16_t value)
{
  if (!can_bus_) {
    return;
  }
  can_bus_->send(
    0x600 + static_cast<uint32_t>(node_id_),
    {0x2B, static_cast<uint8_t>(index & 0xFF), static_cast<uint8_t>((index >> 8) & 0xFF),
      subindex, static_cast<uint8_t>(value & 0xFF), static_cast<uint8_t>((value >> 8) & 0xFF),
      0x00, 0x00});
}

void DiffDriveSystem::sdo_write_4(uint16_t index, uint8_t subindex, uint32_t value)
{
  if (!can_bus_) {
    return;
  }
  can_bus_->send(
    0x600 + static_cast<uint32_t>(node_id_),
    {0x23, static_cast<uint8_t>(index & 0xFF), static_cast<uint8_t>((index >> 8) & 0xFF),
      subindex, static_cast<uint8_t>(value & 0xFF), static_cast<uint8_t>((value >> 8) & 0xFF),
      static_cast<uint8_t>((value >> 16) & 0xFF), static_cast<uint8_t>((value >> 24) & 0xFF)});
}

bool DiffDriveSystem::validate_joint_interfaces() const
{
  for (const auto & joint : info_.joints) {
    if (joint.command_interfaces.size() != 1 ||
      joint.command_interfaces[0].name != hardware_interface::HW_IF_VELOCITY)
    {
      RCLCPP_ERROR(
        logger_, "joint %s must have exactly one velocity command interface",
        joint.name.c_str());
      return false;
    }

    bool has_position = false;
    bool has_velocity = false;
    for (const auto & state_interface : joint.state_interfaces) {
      has_position = has_position || state_interface.name == hardware_interface::HW_IF_POSITION;
      has_velocity = has_velocity || state_interface.name == hardware_interface::HW_IF_VELOCITY;
    }
    if (!has_position || !has_velocity) {
      RCLCPP_ERROR(
        logger_, "joint %s must have position and velocity state interfaces", joint.name.c_str());
      return false;
    }
  }
  return true;
}

void DiffDriveSystem::send_wheel_command(double left_radps, double right_radps)
{
  if (!can_bus_) {
    return;
  }

  const auto left_rpm = clamp_to_i16(
    static_cast<int>(std::lround(left_radps * 60.0 / kTwoPi)) * left_command_sign_);
  const auto right_rpm = clamp_to_i16(
    static_cast<int>(std::lround(right_radps * 60.0 / kTwoPi)) * right_command_sign_);

  std::vector<uint8_t> data {0x23, 0xFF, 0x60, 0x03, 0x00, 0x00, 0x00, 0x00};
  put_i16_le(data, 4, left_rpm);
  put_i16_le(data, 6, right_rpm);
  can_bus_->send(0x600 + static_cast<uint32_t>(node_id_), data);
}

std::optional<std::pair<double, double>> DiffDriveSystem::read_feedback_radps()
{
  if (!can_bus_) {
    return std::nullopt;
  }

  const uint32_t tpdo0_id = 0x180 + static_cast<uint32_t>(node_id_);
  std::optional<std::pair<double, double>> latest;
  while (true) {
    auto received = can_bus_->receive();
    if (!received) {
      break;
    }

    const auto & data = received->second;
    if (received->first != tpdo0_id || data.size() < 4) {
      continue;
    }

    const double left_rpm = static_cast<double>(int16_from_le(data[0], data[1])) * 0.1;
    const double right_rpm = static_cast<double>(int16_from_le(data[2], data[3])) * 0.1;
    latest = std::pair<double, double> {
      left_rpm * feedback_sign_left_ * kTwoPi / 60.0,
      right_rpm * feedback_sign_right_ * kTwoPi / 60.0};
  }
  return latest;
}

double DiffDriveSystem::param_as_double(const std::string & name, double default_value) const
{
  const auto iterator = info_.hardware_parameters.find(name);
  if (iterator == info_.hardware_parameters.end()) {
    return default_value;
  }
  return std::stod(iterator->second);
}

int DiffDriveSystem::param_as_int(const std::string & name, int default_value) const
{
  const auto iterator = info_.hardware_parameters.find(name);
  if (iterator == info_.hardware_parameters.end()) {
    return default_value;
  }
  return std::stoi(iterator->second);
}

bool DiffDriveSystem::param_as_bool(const std::string & name, bool default_value) const
{
  const auto iterator = info_.hardware_parameters.find(name);
  if (iterator == info_.hardware_parameters.end()) {
    return default_value;
  }
  return iterator->second == "true" || iterator->second == "1";
}

std::string DiffDriveSystem::param_as_string(
  const std::string & name, const std::string & default_value) const
{
  const auto iterator = info_.hardware_parameters.find(name);
  if (iterator == info_.hardware_parameters.end()) {
    return default_value;
  }
  return iterator->second;
}

}  // namespace amr_hardware

PLUGINLIB_EXPORT_CLASS(amr_hardware::DiffDriveSystem, hardware_interface::SystemInterface)
