from setuptools import find_packages, setup
from glob import glob

package_name = 'bringups'

setup(
    name=package_name,
    version='0.0.1',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),

        ("share/" + package_name + "/launch", glob("launch/*.launch.py")),
        ("share/" + package_name + "/slam_config", glob("config/slam.yaml")),
        ("share/" + package_name + "/slam_config", glob("config/nav2_params.yaml")),


    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='amr-pc',
    maintainer_email='amr-pc@todo.todo',
    description='TODO: Package description',
    license='Apache-2.0',
    extras_require={
        'test': [
            'pytest',
        ],
    },
    entry_points={
        'console_scripts': [
        ],
    },
)
