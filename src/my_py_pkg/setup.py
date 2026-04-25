from setuptools import find_packages, setup

package_name = 'my_py_pkg'

setup(
    name=package_name,
    version='0.0.0',
    packages=find_packages(exclude=['test']),
    data_files=[
        ('share/ament_index/resource_index/packages',
            ['resource/' + package_name]),
        ('share/' + package_name, ['package.xml']),
    ],
    install_requires=['setuptools'],
    zip_safe=True,
    maintainer='fog',
    maintainer_email='fog@todo.todo',
    description='TODO: Package description',
    license='TODO: License declaration',
    extras_require={
        'test': [
            'pytest',
        ],
    },
    entry_points={
        'console_scripts': [
            'robot_pub = my_py_pkg.robot_pub:main',
            'robot_sub = my_py_pkg.robot_sub:main',
            'number_pub = my_py_pkg.number_pub:main',
            'number_count = my_py_pkg.number_count:main',
            'add_two_ints_server = my_py_pkg.add_two_ints_server:main',
            'add_three_ints_server = my_py_pkg.add_three_ints_server:main',
            'add_two_ints_client = my_py_pkg.add_two_ints_client:main',
            'weather_show_pub = my_py_pkg.weather_show:main',

        ],
    },
)
