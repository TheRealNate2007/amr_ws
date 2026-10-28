from launch import LaunchDescription
from launch.actions import (
    DeclareLaunchArgument,
    IncludeLaunchDescription,
    TimerAction,
)
from launch.conditions import IfCondition
from launch.launch_description_sources import PythonLaunchDescriptionSource
from launch.substitutions import LaunchConfiguration, PathJoinSubstitution
from launch_ros.actions import Node
from launch_ros.substitutions import FindPackageShare


def generate_launch_description():
    can_interface = LaunchConfiguration("can_interface")
    node_id = LaunchConfiguration("node_id")
    serial_port = LaunchConfiguration("serial_port")
    use_rviz = LaunchConfiguration("use_rviz")
    use_keyboard=LaunchConfiguration("use_keyboard")


    hardware_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            PathJoinSubstitution([
                FindPackageShare("amr_hardware"),
                "launch",
                "bringup.launch.py",
            ])
        ),
        launch_arguments={
            "use_can": "true",
            "can_interface": can_interface,
            "node_id": node_id,
        }.items(),
    )

    lidar_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            PathJoinSubstitution([
                FindPackageShare("sllidar_ros2"),
                "launch",
                "sllidar_s3_launch.py",
            ])
        ),
        launch_arguments={
            "serial_port": serial_port,
            "frame_id": "laser_frame",
        }.items(),
    )

    slam_launch = IncludeLaunchDescription(
        PythonLaunchDescriptionSource(
            PathJoinSubstitution([
                FindPackageShare("slam_toolbox"),
                "launch",
                "online_async_launch.py",
            ])
        ),
        launch_arguments={
            "use_sim_time": "false",
            "slam_params_file":
                "/home/amr-pc/amr_ws/src/bringups/config/slam.yaml",
        }.items(),
    )

    manual_control_node= Node(
        package="keyboard_cmd_vel",
        executable="keyboard_cmd_vel_node",
        name="keyboard_control",
        output="screen",
        parameters=[{"use_sim_time": False}],
        condition=IfCondition(use_keyboard),
    )

    rviz_node = Node(
        package="rviz2",
        executable="rviz2",
        name="rviz",
        output="screen",
        parameters=[{"use_sim_time": False}],
        condition=IfCondition(use_rviz),
    )

    return LaunchDescription([
        DeclareLaunchArgument(
            "can_interface",
            default_value="can0",
        ),
        DeclareLaunchArgument(
            "node_id",
            default_value="1",
        ),
        DeclareLaunchArgument(
            "serial_port",
            default_value="/dev/ttyUSB0",
        ),
       
        DeclareLaunchArgument(
            "use_rviz",
            default_value="true",
        ),

        DeclareLaunchArgument(
            "use_keyboard",
            default_value="true",
        ),



        hardware_launch,
        lidar_launch,

        TimerAction(
            period=4.0,
            actions=[manual_control_node],
        ),

        TimerAction(
            period=5.0,
            actions=[slam_launch],
        ),

        TimerAction(
            period=7.0,
            actions=[rviz_node],
        ),
    ])