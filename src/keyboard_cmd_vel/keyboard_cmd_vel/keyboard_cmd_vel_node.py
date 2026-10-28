#!/usr/bin/env python3

import time
from select import select

import rclpy
from rclpy.node import Node
from geometry_msgs.msg import TwistStamped

from evdev import InputDevice, categorize, ecodes, list_devices


MOTION_KEY_TIMEOUT = 0.75
#INPUT_DEVICE_PATH = "/dev/input/by-id/usb-MOSART_Semi._2.4G_Keyboard_Mouse-event-kbd"
INPUT_DEVICE_PATH = "/dev/input/by-id/usb-Compx_2.4G_Receiver-event-kbd"



KEY_RELEASED = 0
KEY_PRESSED = 1
KEY_REPEATED = 2

UP_KEYS = {ecodes.KEY_UP, ecodes.KEY_KP8}
DOWN_KEYS = {ecodes.KEY_DOWN, ecodes.KEY_KP2}
LEFT_KEYS = {ecodes.KEY_LEFT, ecodes.KEY_KP4}
RIGHT_KEYS = {ecodes.KEY_RIGHT, ecodes.KEY_KP6}
STOP_KEYS = {ecodes.KEY_SPACE, ecodes.KEY_KP5, ecodes.KEY_ENTER}
EXIT_KEYS = {ecodes.KEY_ESC}

CONTROL_KEYS = (
    UP_KEYS
    | DOWN_KEYS
    | LEFT_KEYS
    | RIGHT_KEYS
    | STOP_KEYS
    | EXIT_KEYS
)


def open_input_device():
    candidates = [INPUT_DEVICE_PATH]
    candidates.extend(path for path in list_devices() if path != INPUT_DEVICE_PATH)

    permission_errors = []

    for path in candidates:
        try:
            device = InputDevice(path)
        except FileNotFoundError:
            continue
        except PermissionError:
            permission_errors.append(path)
            continue
        except OSError:
            continue

        supported_keys = set(device.capabilities().get(ecodes.EV_KEY, []))

        if supported_keys & CONTROL_KEYS:
            if path != INPUT_DEVICE_PATH:
                print(f"Configured input device not found; using {path}.")
            return device

        device.close()

    if permission_errors:
        paths = ", ".join(permission_errors)
        raise SystemExit(
            f"No permission to read input device(s): {paths}. "
            "Run with sudo or add this user to the input group."
        )

    raise SystemExit(
        "No keyboard/keypad input device found. Connect the controller and "
        "check the event device."
    )


class KeyboardCmdVelNode(Node):
    def __init__(self):
        super().__init__("keyboard_cmd_vel_node")

        self.declare_parameter("cmd_vel_topic", "/diff_drive_controller/cmd_vel")
        self.declare_parameter("frame_id", "base_link")
        self.declare_parameter("linear_speed", 0.2)
        self.declare_parameter("angular_speed", 0.8)
        self.declare_parameter("publish_rate", 20.0)

        self.cmd_vel_topic = self.get_parameter("cmd_vel_topic").value
        self.frame_id = self.get_parameter("frame_id").value
        self.linear_speed = float(self.get_parameter("linear_speed").value)
        self.angular_speed = float(self.get_parameter("angular_speed").value)
        publish_rate = float(self.get_parameter("publish_rate").value)

        self.publisher = self.create_publisher(
            TwistStamped,
            self.cmd_vel_topic,
            10,
        )

        self.device = open_input_device()
        self.get_logger().info(f"Listening to: {self.device.path} {self.device.name}")
        self.get_logger().info(
            "Ready. Arrow keys/keypad move, Space/KP5 stops, Esc exits."
        )

        self.pressed = set()
        self.last_motion_event = {}
        self.running = True

        self.last_cmd = None

        timer_period = 1.0 / publish_rate
        self.timer = self.create_timer(timer_period, self.loop)

    def handle_key_event(self, event):
        if event.type != ecodes.EV_KEY:
            return

        key_event = categorize(event)

        if key_event.keystate in (KEY_PRESSED, KEY_REPEATED):
            if event.code in UP_KEYS | DOWN_KEYS | LEFT_KEYS | RIGHT_KEYS:
                self.last_motion_event[event.code] = time.monotonic()

            if key_event.keystate == KEY_REPEATED:
                return

            self.get_logger().info(
                f"Pressed: {key_event.keycode}, ecode: {event.code}"
            )

            if event.code in EXIT_KEYS:
                self.running = False
                rclpy.shutdown()
                return

            self.pressed.add(event.code)

        elif key_event.keystate == KEY_RELEASED:
            self.pressed.discard(event.code)
            self.last_motion_event.pop(event.code, None)

    def expire_stale_motion_keys(self):
        now = time.monotonic()

        for key_code, last_seen in list(self.last_motion_event.items()):
            if now - last_seen > MOTION_KEY_TIMEOUT:
                self.pressed.discard(key_code)
                del self.last_motion_event[key_code]
                self.get_logger().info("Motion key timed out; stopping robot.")

    def read_input_events(self):
        readable, _, _ = select([self.device], [], [], 0.0)

        if not readable:
            return

        for event in self.device.read():
            self.handle_key_event(event)

    def get_command(self):
        linear_x = 0.0
        angular_z = 0.0

        if self.pressed & STOP_KEYS:
            return linear_x, angular_z

        if self.pressed & UP_KEYS:
            linear_x = self.linear_speed

        elif self.pressed & DOWN_KEYS:
            linear_x = -self.linear_speed

        elif self.pressed & LEFT_KEYS:
            angular_z = self.angular_speed

        elif self.pressed & RIGHT_KEYS:
            angular_z = -self.angular_speed

        return linear_x, angular_z

    def publish_cmd_vel(self, linear_x, angular_z):
        msg = TwistStamped()

        msg.header.stamp = self.get_clock().now().to_msg()
        msg.header.frame_id = self.frame_id

        msg.twist.linear.x = linear_x
        msg.twist.linear.y = 0.0
        msg.twist.linear.z = 0.0

        msg.twist.angular.x = 0.0
        msg.twist.angular.y = 0.0
        msg.twist.angular.z = angular_z

        self.publisher.publish(msg)

    def loop(self):
        self.read_input_events()
        self.expire_stale_motion_keys()

        cmd = self.get_command()
        self.publish_cmd_vel(cmd[0], cmd[1])

        if cmd != self.last_cmd:
            self.get_logger().info(
                f"linear.x={cmd[0]:.3f} m/s, angular.z={cmd[1]:.3f} rad/s"
            )
            self.last_cmd = cmd

    def destroy_node(self):
        self.get_logger().info("Stopping robot...")

        try:
            self.publish_cmd_vel(0.0, 0.0)
            time.sleep(0.1)
        except Exception:
            pass

        try:
            self.device.close()
        except Exception:
            pass

        super().destroy_node()


def main(args=None):
    rclpy.init(args=args)

    node = KeyboardCmdVelNode()

    try:
        rclpy.spin(node)
    except KeyboardInterrupt:
        pass
    finally:
        node.destroy_node()

        if rclpy.ok():
            rclpy.shutdown()


if __name__ == "__main__":
    main()
