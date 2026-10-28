#ifndef AMR_HARDWARE__DIFF_DRIVE_SYSTEM_HPP_
#define AMR_HARDWARE__DIFF_DRIVE_SYSTEM_HPP_

#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include "hardware_interface/handle.hpp"
#include "hardware_interface/hardware_info.hpp"
#include "hardware_interface/system_interface.hpp"
#include "hardware_interface/types/hardware_component_interface_params.hpp"
#include "hardware_interface/types/hardware_interface_return_values.hpp"
#include "rclcpp/logger.hpp"
#include "rclcpp_lifecycle/state.hpp"

namespace amr_hardware
{

class SocketCanBus
{
public:
  explicit SocketCanBus(const std::string & interface_name);
  ~SocketCanBus();

  SocketCanBus(const SocketCanBus &) = delete;
  SocketCanBus & operator=(const SocketCanBus &) = delete;

  void send(uint32_t can_id, const std::vector<uint8_t> & data);
  std::optional<std::pair<uint32_t, std::vector<uint8_t>>> receive();

private:
  void close();

  int fd_ {-1};
};

class DiffDriveSystem : public hardware_interface::SystemInterface
{
public:
  hardware_interface::CallbackReturn on_init(
    const hardware_interface::HardwareComponentInterfaceParams & params) override;

  std::vector<hardware_interface::StateInterface> export_state_interfaces() override;
  std::vector<hardware_interface::CommandInterface> export_command_interfaces() override;

  hardware_interface::CallbackReturn on_configure(
    const rclcpp_lifecycle::State & previous_state) override;
  hardware_interface::CallbackReturn on_activate(
    const rclcpp_lifecycle::State & previous_state) override;
  hardware_interface::CallbackReturn on_deactivate(
    const rclcpp_lifecycle::State & previous_state) override;

  hardware_interface::return_type read(
    const rclcpp::Time & time, const rclcpp::Duration & period) override;
  hardware_interface::return_type write(
    const rclcpp::Time & time, const rclcpp::Duration & period) override;

private:
  bool parse_parameters();
  bool validate_joint_interfaces() const;
  void initialize_drive();
  void configure_speed_feedback_tpdo();
  void send_nmt(uint8_t command);
  void sdo_write_1(uint16_t index, uint8_t subindex, uint8_t value);
  void sdo_write_2(uint16_t index, uint8_t subindex, uint16_t value);
  void sdo_write_4(uint16_t index, uint8_t subindex, uint32_t value);
  void send_wheel_command(double left_radps, double right_radps);
  std::optional<std::pair<double, double>> read_feedback_radps();

  double param_as_double(const std::string & name, double default_value) const;
  int param_as_int(const std::string & name, int default_value) const;
  bool param_as_bool(const std::string & name, bool default_value) const;
  std::string param_as_string(const std::string & name, const std::string & default_value) const;

  rclcpp::Logger logger_ {rclcpp::get_logger("amr_hardware.DiffDriveSystem")};

  std::string left_wheel_name_;
  std::string right_wheel_name_;
  std::string can_interface_ {"can0"};

  bool use_can_ {true};
  bool configure_tpdo_ {true};
  int node_id_ {1};
  int left_command_sign_ {1};
  int right_command_sign_ {-1};
  double max_wheel_velocity_radps_ {14.55};
  double feedback_sign_left_ {1.0};
  double feedback_sign_right_ {-1.0};

  double hw_commands_[2] {0.0, 0.0};
  double hw_positions_[2] {0.0, 0.0};
  double hw_velocities_[2] {0.0, 0.0};

  std::unique_ptr<SocketCanBus> can_bus_;
};

}  // namespace amr_hardware

#endif  // AMR_HARDWARE__DIFF_DRIVE_SYSTEM_HPP_
