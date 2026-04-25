#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to my_robot_interfaces__srv__AddthreeInts_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
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
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddthreeInts_Request::default())
  }
}

impl rosidl_runtime_rs::Message for AddthreeInts_Request {
  type RmwMsg = super::srv::rmw::AddthreeInts_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        a: msg.a,
        b: msg.b,
        c: msg.c,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      a: msg.a,
      b: msg.b,
      c: msg.c,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      a: msg.a,
      b: msg.b,
      c: msg.c,
    }
  }
}


// Corresponds to my_robot_interfaces__srv__AddthreeInts_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct AddthreeInts_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub sum: i64,

}



impl Default for AddthreeInts_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::AddthreeInts_Response::default())
  }
}

impl rosidl_runtime_rs::Message for AddthreeInts_Response {
  type RmwMsg = super::srv::rmw::AddthreeInts_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        sum: msg.sum,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      sum: msg.sum,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      sum: msg.sum,
    }
  }
}


// Corresponds to my_robot_interfaces__srv__CatchTurtle_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CatchTurtle_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,

}



impl Default for CatchTurtle_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CatchTurtle_Request::default())
  }
}

impl rosidl_runtime_rs::Message for CatchTurtle_Request {
  type RmwMsg = super::srv::rmw::CatchTurtle_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
    }
  }
}


// Corresponds to my_robot_interfaces__srv__CatchTurtle_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CatchTurtle_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,

}



impl Default for CatchTurtle_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::CatchTurtle_Response::default())
  }
}

impl rosidl_runtime_rs::Message for CatchTurtle_Response {
  type RmwMsg = super::srv::rmw::CatchTurtle_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
    }
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


