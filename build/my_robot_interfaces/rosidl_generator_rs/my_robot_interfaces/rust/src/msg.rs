#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to my_robot_interfaces__msg__Weather

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Weather {

    // This member is not documented.
    #[allow(missing_docs)]
    pub temperature: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub humidity: f64,

}



impl Default for Weather {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Weather::default())
  }
}

impl rosidl_runtime_rs::Message for Weather {
  type RmwMsg = super::msg::rmw::Weather;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        temperature: msg.temperature,
        humidity: msg.humidity,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      temperature: msg.temperature,
      humidity: msg.humidity,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      temperature: msg.temperature,
      humidity: msg.humidity,
    }
  }
}


// Corresponds to my_robot_interfaces__msg__Turtle

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Turtle {

    // This member is not documented.
    #[allow(missing_docs)]
    pub name: std::string::String,


    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub theta: f64,

}



impl Default for Turtle {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::Turtle::default())
  }
}

impl rosidl_runtime_rs::Message for Turtle {
  type RmwMsg = super::msg::rmw::Turtle;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
        x: msg.x,
        y: msg.y,
        theta: msg.theta,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        name: msg.name.as_str().into(),
      x: msg.x,
      y: msg.y,
      theta: msg.theta,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      name: msg.name.to_string(),
      x: msg.x,
      y: msg.y,
      theta: msg.theta,
    }
  }
}


// Corresponds to my_robot_interfaces__msg__TurtleArray

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TurtleArray {

    // This member is not documented.
    #[allow(missing_docs)]
    pub turtles: Vec<super::msg::Turtle>,

}



impl Default for TurtleArray {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::TurtleArray::default())
  }
}

impl rosidl_runtime_rs::Message for TurtleArray {
  type RmwMsg = super::msg::rmw::TurtleArray;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        turtles: msg.turtles
          .into_iter()
          .map(|elem| super::msg::Turtle::into_rmw_message(std::borrow::Cow::Owned(elem)).into_owned())
          .collect(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        turtles: msg.turtles
          .iter()
          .map(|elem| super::msg::Turtle::into_rmw_message(std::borrow::Cow::Borrowed(elem)).into_owned())
          .collect(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      turtles: msg.turtles
          .into_iter()
          .map(super::msg::Turtle::from_rmw_message)
          .collect(),
    }
  }
}


