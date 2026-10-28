from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument, TimerAction
from launch.substitutions import Command, LaunchConfiguration, PathJoinSubstitution
from launch_ros.actions import Node
from launch_ros.substitutions import FindPackageShare


def generate_launch_description():
    use_can = LaunchConfiguration("use_can")
    can_interface = LaunchConfiguration("can_interface")
    node_id = LaunchConfiguration("node_id")

    robot_description_content = Command([
        "xacro ",
        PathJoinSubstitution([
            FindPackageShare("amr_hardware"),
            "urdf",
            "amr_ros2_control.urdf.xacro",
        ]),
        " use_can:=",
        use_can,
        " can_interface:=",
        can_interface,
        " node_id:=",
        node_id,
    ])

    robot_description = {"robot_description": robot_description_content}

    controllers_file = PathJoinSubstitution([
        FindPackageShare("amr_hardware"),
        "config",
        "ros2_control.yaml",
    ])

    return LaunchDescription([
        DeclareLaunchArgument("use_can", default_value="true"),
        DeclareLaunchArgument("can_interface", default_value="can0"),
        DeclareLaunchArgument("node_id", default_value="1"),

        Node(
            package="robot_state_publisher",
            executable="robot_state_publisher",
            parameters=[robot_description],
            output="screen",
        ),

        Node(
            package="controller_manager",
            executable="ros2_control_node",
            parameters=[robot_description, controllers_file],
            output="screen",
        ),

        TimerAction(
            period=2.0,
            actions=[
                Node(
                    package="controller_manager",
                    executable="spawner",
                    arguments=["joint_state_broadcaster"],
                    output="screen",
                ),
            ],
        ),

        TimerAction(
            period=3.0,
            actions=[
                Node(
                    package="controller_manager",
                    executable="spawner",
                    arguments=["diff_drive_controller"],
                    output="screen",
                ),
            ],
        ),
    ])
