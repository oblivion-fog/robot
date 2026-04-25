// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from my_robot_interfaces:msg/Weather.idl
// generated code does not contain a copyright notice

#ifndef MY_ROBOT_INTERFACES__MSG__DETAIL__WEATHER__BUILDER_HPP_
#define MY_ROBOT_INTERFACES__MSG__DETAIL__WEATHER__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "my_robot_interfaces/msg/detail/weather__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace my_robot_interfaces
{

namespace msg
{

namespace builder
{

class Init_Weather_humidity
{
public:
  explicit Init_Weather_humidity(::my_robot_interfaces::msg::Weather & msg)
  : msg_(msg)
  {}
  ::my_robot_interfaces::msg::Weather humidity(::my_robot_interfaces::msg::Weather::_humidity_type arg)
  {
    msg_.humidity = std::move(arg);
    return std::move(msg_);
  }

private:
  ::my_robot_interfaces::msg::Weather msg_;
};

class Init_Weather_temperature
{
public:
  Init_Weather_temperature()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_Weather_humidity temperature(::my_robot_interfaces::msg::Weather::_temperature_type arg)
  {
    msg_.temperature = std::move(arg);
    return Init_Weather_humidity(msg_);
  }

private:
  ::my_robot_interfaces::msg::Weather msg_;
};

}  // namespace builder

}  // namespace msg

template<typename MessageType>
auto build();

template<>
inline
auto build<::my_robot_interfaces::msg::Weather>()
{
  return my_robot_interfaces::msg::builder::Init_Weather_temperature();
}

}  // namespace my_robot_interfaces

#endif  // MY_ROBOT_INTERFACES__MSG__DETAIL__WEATHER__BUILDER_HPP_
