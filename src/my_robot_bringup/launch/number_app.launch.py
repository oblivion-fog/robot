from launch import LaunchDescription
from launch_ros.actions import Node


def generate_launch_description():
    # Create a launch description and add the nodes to it
    ld=LaunchDescription()
    number_pub = Node(
        package="my_py_pkg",
        executable="number_pub"
        )
    
    number_count = Node(
        package="my_py_pkg",
        executable="number_count"
        )
    
    # Add the nodes to the launch description
    ld.add_action(number_pub)
    ld.add_action(number_count)
    
    return ld