#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__AddthreeInts_Request() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__srv__AddthreeInts_Request__init(msg: *mut AddthreeInts_Request) -> bool;
    fn my_robot_interfaces__srv__AddthreeInts_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddthreeInts_Request>, size: usize) -> bool;
    fn my_robot_interfaces__srv__AddthreeInts_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddthreeInts_Request>);
    fn my_robot_interfaces__srv__AddthreeInts_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddthreeInts_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<AddthreeInts_Request>) -> bool;
}

// Corresponds to my_robot_interfaces__srv__AddthreeInts_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddthreeInts_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub a: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub b: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub c: i64,

}



impl Default for AddthreeInts_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__srv__AddthreeInts_Request__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__srv__AddthreeInts_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddthreeInts_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__AddthreeInts_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__AddthreeInts_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__AddthreeInts_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddthreeInts_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddthreeInts_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/srv/AddthreeInts_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__AddthreeInts_Request() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__AddthreeInts_Response() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__srv__AddthreeInts_Response__init(msg: *mut AddthreeInts_Response) -> bool;
    fn my_robot_interfaces__srv__AddthreeInts_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<AddthreeInts_Response>, size: usize) -> bool;
    fn my_robot_interfaces__srv__AddthreeInts_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<AddthreeInts_Response>);
    fn my_robot_interfaces__srv__AddthreeInts_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<AddthreeInts_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<AddthreeInts_Response>) -> bool;
}

// Corresponds to my_robot_interfaces__srv__AddthreeInts_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddthreeInts_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub sum: i64,

}



impl Default for AddthreeInts_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__srv__AddthreeInts_Response__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__srv__AddthreeInts_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for AddthreeInts_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__AddthreeInts_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__AddthreeInts_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__AddthreeInts_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for AddthreeInts_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for AddthreeInts_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/srv/AddthreeInts_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__AddthreeInts_Response() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__CatchTurtle_Request() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__srv__CatchTurtle_Request__init(msg: *mut CatchTurtle_Request) -> bool;
    fn my_robot_interfaces__srv__CatchTurtle_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Request>, size: usize) -> bool;
    fn my_robot_interfaces__srv__CatchTurtle_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Request>);
    fn my_robot_interfaces__srv__CatchTurtle_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CatchTurtle_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Request>) -> bool;
}

// Corresponds to my_robot_interfaces__srv__CatchTurtle_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CatchTurtle_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: rosidl_runtime_rs::String,

}



impl Default for CatchTurtle_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__srv__CatchTurtle_Request__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__srv__CatchTurtle_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CatchTurtle_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CatchTurtle_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CatchTurtle_Request where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/srv/CatchTurtle_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__CatchTurtle_Request() }
  }
}


#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__CatchTurtle_Response() -> *const std::ffi::c_void;
}

#[link(name = "my_robot_interfaces__rosidl_generator_c")]
extern "C" {
    fn my_robot_interfaces__srv__CatchTurtle_Response__init(msg: *mut CatchTurtle_Response) -> bool;
    fn my_robot_interfaces__srv__CatchTurtle_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Response>, size: usize) -> bool;
    fn my_robot_interfaces__srv__CatchTurtle_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Response>);
    fn my_robot_interfaces__srv__CatchTurtle_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<CatchTurtle_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<CatchTurtle_Response>) -> bool;
}

// Corresponds to my_robot_interfaces__srv__CatchTurtle_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CatchTurtle_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for CatchTurtle_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !my_robot_interfaces__srv__CatchTurtle_Response__init(&mut msg as *mut _) {
        panic!("Call to my_robot_interfaces__srv__CatchTurtle_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for CatchTurtle_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { my_robot_interfaces__srv__CatchTurtle_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for CatchTurtle_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for CatchTurtle_Response where Self: Sized {
  const TYPE_NAME: &'static str = "my_robot_interfaces/srv/CatchTurtle_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__my_robot_interfaces__srv__CatchTurtle_Response() }
  }
}






#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__srv__AddthreeInts() -> *const std::ffi::c_void;
}

// Corresponds to my_robot_interfaces__srv__AddthreeInts
#[allow(missing_docs, non_camel_case_types)]
pub struct AddthreeInts;

impl rosidl_runtime_rs::Service for AddthreeInts {
    type Request = AddthreeInts_Request;
    type Response = AddthreeInts_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__srv__AddthreeInts() }
    }
}




#[link(name = "my_robot_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__srv__CatchTurtle() -> *const std::ffi::c_void;
}

// Corresponds to my_robot_interfaces__srv__CatchTurtle
#[allow(missing_docs, non_camel_case_types)]
pub struct CatchTurtle;

impl rosidl_runtime_rs::Service for CatchTurtle {
    type Request = CatchTurtle_Request;
    type Response = CatchTurtle_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__my_robot_interfaces__srv__CatchTurtle() }
    }
}


