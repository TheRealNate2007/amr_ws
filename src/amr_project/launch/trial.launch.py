import os

from ament_index_python.packages import get_package_share_directory
from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.substitutions import LaunchConfiguration
from launch_ros.actions import Node
import xacro

def generate_launch_description():

    # 1. Capture the 'use_sim_time' configuration
    use_sim_time = LaunchConfiguration('use_sim_time')

    # 2. Process the Xacro file to an XML string
    # We use get_package_share_directory to find the physical path on your system [1]
    pkg_path = get_package_share_directory('amr_project')
    
    # Ensure this matches your actual folder name (previously you used 'description') [1]
    xacro_file = os.path.join(pkg_path, 'urdf_description', 'robot.urdf.xacro')
    
    # This line converts your Xacro templates and includes into a raw URDF string [1]
    robot_description_config = xacro.process_file(xacro_file).toxml()

    # 3. Create the robot_state_publisher node
    node_robot_state_publisher = Node(
        package='robot_state_publisher',
        executable='robot_state_publisher',
        output='screen',
        parameters=[{
            'robot_description': robot_description_config,
            'use_sim_time': use_sim_time
        }]
    )

    # 4. Return the LaunchDescription
    return LaunchDescription([
        DeclareLaunchArgument(
            'use_sim_time',
            default_value='false',
            description='Use simulation (Gazebo) clock if true'),
        node_robot_state_publisher
    ])