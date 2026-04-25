// generated from rosidl_generator_cpp/resource/idl__builder.hpp.em
// with input from my_robot_interfaces:srv/AddthreeInts.idl
// generated code does not contain a copyright notice

#ifndef MY_ROBOT_INTERFACES__SRV__DETAIL__ADDTHREE_INTS__BUILDER_HPP_
#define MY_ROBOT_INTERFACES__SRV__DETAIL__ADDTHREE_INTS__BUILDER_HPP_

#include <algorithm>
#include <utility>

#include "my_robot_interfaces/srv/detail/addthree_ints__struct.hpp"
#include "rosidl_runtime_cpp/message_initialization.hpp"


namespace my_robot_interfaces
{

namespace srv
{

namespace builder
{

class Init_AddthreeInts_Request_c
{
public:
  explicit Init_AddthreeInts_Request_c(::my_robot_interfaces::srv::AddthreeInts_Request & msg)
  : msg_(msg)
  {}
  ::my_robot_interfaces::srv::AddthreeInts_Request c(::my_robot_interfaces::srv::AddthreeInts_Request::_c_type arg)
  {
    msg_.c = std::move(arg);
    return std::move(msg_);
  }

private:
  ::my_robot_interfaces::srv::AddthreeInts_Request msg_;
};

class Init_AddthreeInts_Request_b
{
public:
  explicit Init_AddthreeInts_Request_b(::my_robot_interfaces::srv::AddthreeInts_Request & msg)
  : msg_(msg)
  {}
  Init_AddthreeInts_Request_c b(::my_robot_interfaces::srv::AddthreeInts_Request::_b_type arg)
  {
    msg_.b = std::move(arg);
    return Init_AddthreeInts_Request_c(msg_);
  }

private:
  ::my_robot_interfaces::srv::AddthreeInts_Request msg_;
};

class Init_AddthreeInts_Request_a
{
public:
  Init_AddthreeInts_Request_a()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  Init_AddthreeInts_Request_b a(::my_robot_interfaces::srv::AddthreeInts_Request::_a_type arg)
  {
    msg_.a = std::move(arg);
    return Init_AddthreeInts_Request_b(msg_);
  }

private:
  ::my_robot_interfaces::srv::AddthreeInts_Request msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::my_robot_interfaces::srv::AddthreeInts_Request>()
{
  return my_robot_interfaces::srv::builder::Init_AddthreeInts_Request_a();
}

}  // namespace my_robot_interfaces


namespace my_robot_interfaces
{

namespace srv
{

namespace builder
{

class Init_AddthreeInts_Response_sum
{
public:
  Init_AddthreeInts_Response_sum()
  : msg_(::rosidl_runtime_cpp::MessageInitialization::SKIP)
  {}
  ::my_robot_interfaces::srv::AddthreeInts_Response sum(::my_robot_interfaces::srv::AddthreeInts_Response::_sum_type arg)
  {
    msg_.sum = std::move(arg);
    return std::move(msg_);
  }

private:
  ::my_robot_interfaces::srv::AddthreeInts_Response msg_;
};

}  // namespace builder

}  // namespace srv

template<typename MessageType>
auto build();

template<>
inline
auto build<::my_robot_interfaces::srv::AddthreeInts_Response>()
{
  return my_robot_interfaces::srv::builder::Init_AddthreeInts_Response_sum();
}

}  // namespace my_robot_interfaces

#endif  // MY_ROBOT_INTERFACES__SRV__DETAIL__ADDTHREE_INTS__BUILDER_HPP_
