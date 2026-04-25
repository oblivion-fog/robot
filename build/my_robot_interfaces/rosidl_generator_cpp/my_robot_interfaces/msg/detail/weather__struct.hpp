// generated from rosidl_generator_cpp/resource/idl__struct.hpp.em
// with input from my_robot_interfaces:msg/Weather.idl
// generated code does not contain a copyright notice

#ifndef MY_ROBOT_INTERFACES__MSG__DETAIL__WEATHER__STRUCT_HPP_
#define MY_ROBOT_INTERFACES__MSG__DETAIL__WEATHER__STRUCT_HPP_

#include <algorithm>
#include <array>
#include <cstdint>
#include <memory>
#include <string>
#include <vector>

#include "rosidl_runtime_cpp/bounded_vector.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


#ifndef _WIN32
# define DEPRECATED__my_robot_interfaces__msg__Weather __attribute__((deprecated))
#else
# define DEPRECATED__my_robot_interfaces__msg__Weather __declspec(deprecated)
#endif

namespace my_robot_interfaces
{

namespace msg
{

// message struct
template<class ContainerAllocator>
struct Weather_
{
  using Type = Weather_<ContainerAllocator>;

  explicit Weather_(rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->temperature = 0.0;
      this->humidity = 0.0;
    }
  }

  explicit Weather_(const ContainerAllocator & _alloc, rosidl_runtime_cpp::MessageInitialization _init = rosidl_runtime_cpp::MessageInitialization::ALL)
  {
    (void)_alloc;
    if (rosidl_runtime_cpp::MessageInitialization::ALL == _init ||
      rosidl_runtime_cpp::MessageInitialization::ZERO == _init)
    {
      this->temperature = 0.0;
      this->humidity = 0.0;
    }
  }

  // field types and members
  using _temperature_type =
    double;
  _temperature_type temperature;
  using _humidity_type =
    double;
  _humidity_type humidity;

  // setters for named parameter idiom
  Type & set__temperature(
    const double & _arg)
  {
    this->temperature = _arg;
    return *this;
  }
  Type & set__humidity(
    const double & _arg)
  {
    this->humidity = _arg;
    return *this;
  }

  // constant declarations

  // pointer types
  using RawPtr =
    my_robot_interfaces::msg::Weather_<ContainerAllocator> *;
  using ConstRawPtr =
    const my_robot_interfaces::msg::Weather_<ContainerAllocator> *;
  using SharedPtr =
    std::shared_ptr<my_robot_interfaces::msg::Weather_<ContainerAllocator>>;
  using ConstSharedPtr =
    std::shared_ptr<my_robot_interfaces::msg::Weather_<ContainerAllocator> const>;

  template<typename Deleter = std::default_delete<
      my_robot_interfaces::msg::Weather_<ContainerAllocator>>>
  using UniquePtrWithDeleter =
    std::unique_ptr<my_robot_interfaces::msg::Weather_<ContainerAllocator>, Deleter>;

  using UniquePtr = UniquePtrWithDeleter<>;

  template<typename Deleter = std::default_delete<
      my_robot_interfaces::msg::Weather_<ContainerAllocator>>>
  using ConstUniquePtrWithDeleter =
    std::unique_ptr<my_robot_interfaces::msg::Weather_<ContainerAllocator> const, Deleter>;
  using ConstUniquePtr = ConstUniquePtrWithDeleter<>;

  using WeakPtr =
    std::weak_ptr<my_robot_interfaces::msg::Weather_<ContainerAllocator>>;
  using ConstWeakPtr =
    std::weak_ptr<my_robot_interfaces::msg::Weather_<ContainerAllocator> const>;

  // pointer types similar to ROS 1, use SharedPtr / ConstSharedPtr instead
  // NOTE: Can't use 'using' here because GNU C++ can't parse attributes properly
  typedef DEPRECATED__my_robot_interfaces__msg__Weather
    std::shared_ptr<my_robot_interfaces::msg::Weather_<ContainerAllocator>>
    Ptr;
  typedef DEPRECATED__my_robot_interfaces__msg__Weather
    std::shared_ptr<my_robot_interfaces::msg::Weather_<ContainerAllocator> const>
    ConstPtr;

  // comparison operators
  bool operator==(const Weather_ & other) const
  {
    if (this->temperature != other.temperature) {
      return false;
    }
    if (this->humidity != other.humidity) {
      return false;
    }
    return true;
  }
  bool operator!=(const Weather_ & other) const
  {
    return !this->operator==(other);
  }
};  // struct Weather_

// alias to use template instance with default allocator
using Weather =
  my_robot_interfaces::msg::Weather_<std::allocator<void>>;

// constant definitions

}  // namespace msg

}  // namespace my_robot_interfaces

#endif  // MY_ROBOT_INTERFACES__MSG__DETAIL__WEATHER__STRUCT_HPP_
