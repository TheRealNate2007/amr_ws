#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};




// Corresponds to panthera_interfaces__srv__MoveToJoint_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveToJoint_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub joint_angles: [f64; 6],

    /// Max velocity scaling factor (0.0~1.0)
    pub velocity_scaling: f64,

    /// Max acceleration scaling factor (0.0~1.0)
    pub acceleration_scaling: f64,

}



impl Default for MoveToJoint_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveToJoint_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MoveToJoint_Request {
  type RmwMsg = super::srv::rmw::MoveToJoint_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_angles: msg.joint_angles,
        velocity_scaling: msg.velocity_scaling,
        acceleration_scaling: msg.acceleration_scaling,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        joint_angles: msg.joint_angles,
      velocity_scaling: msg.velocity_scaling,
      acceleration_scaling: msg.acceleration_scaling,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      joint_angles: msg.joint_angles,
      velocity_scaling: msg.velocity_scaling,
      acceleration_scaling: msg.acceleration_scaling,
    }
  }
}


// Corresponds to panthera_interfaces__srv__MoveToJoint_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveToJoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for MoveToJoint_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveToJoint_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MoveToJoint_Response {
  type RmwMsg = super::srv::rmw::MoveToJoint_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to panthera_interfaces__srv__MoveToPose_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveToPose_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub x: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub y: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub z: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub roll: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub pitch: f64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub yaw: f64,

    /// Use Cartesian path planning (linear motion)
    pub cartesian_path: bool,

    /// Max velocity scaling factor (0.0~1.0)
    pub velocity_scaling: f64,

    /// Max acceleration scaling factor (0.0~1.0)
    pub acceleration_scaling: f64,

}



impl Default for MoveToPose_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveToPose_Request::default())
  }
}

impl rosidl_runtime_rs::Message for MoveToPose_Request {
  type RmwMsg = super::srv::rmw::MoveToPose_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        roll: msg.roll,
        pitch: msg.pitch,
        yaw: msg.yaw,
        cartesian_path: msg.cartesian_path,
        velocity_scaling: msg.velocity_scaling,
        acceleration_scaling: msg.acceleration_scaling,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
      cartesian_path: msg.cartesian_path,
      velocity_scaling: msg.velocity_scaling,
      acceleration_scaling: msg.acceleration_scaling,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
      cartesian_path: msg.cartesian_path,
      velocity_scaling: msg.velocity_scaling,
      acceleration_scaling: msg.acceleration_scaling,
    }
  }
}


// Corresponds to panthera_interfaces__srv__MoveToPose_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveToPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for MoveToPose_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::MoveToPose_Response::default())
  }
}

impl rosidl_runtime_rs::Message for MoveToPose_Response {
  type RmwMsg = super::srv::rmw::MoveToPose_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to panthera_interfaces__srv__GripperControl_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub action: std::string::String,

}



impl Default for GripperControl_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GripperControl_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GripperControl_Request {
  type RmwMsg = super::srv::rmw::GripperControl_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        action: msg.action.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        action: msg.action.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      action: msg.action.to_string(),
    }
  }
}


// Corresponds to panthera_interfaces__srv__GripperControl_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: std::string::String,

}



impl Default for GripperControl_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GripperControl_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GripperControl_Response {
  type RmwMsg = super::srv::rmw::GripperControl_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        success: msg.success,
        message: msg.message.as_str().into(),
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      success: msg.success,
        message: msg.message.as_str().into(),
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      success: msg.success,
      message: msg.message.to_string(),
    }
  }
}


// Corresponds to panthera_interfaces__srv__Enable_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Enable_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub enable_request: bool,

}



impl Default for Enable_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Enable_Request::default())
  }
}

impl rosidl_runtime_rs::Message for Enable_Request {
  type RmwMsg = super::srv::rmw::Enable_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        enable_request: msg.enable_request,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      enable_request: msg.enable_request,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      enable_request: msg.enable_request,
    }
  }
}


// Corresponds to panthera_interfaces__srv__Enable_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Enable_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub enable_response: bool,

}



impl Default for Enable_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::Enable_Response::default())
  }
}

impl rosidl_runtime_rs::Message for Enable_Response {
  type RmwMsg = super::srv::rmw::Enable_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        enable_response: msg.enable_response,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      enable_response: msg.enable_response,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      enable_response: msg.enable_response,
    }
  }
}


// Corresponds to panthera_interfaces__srv__GoZero_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoZero_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub use_mit_mode: bool,

}



impl Default for GoZero_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GoZero_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GoZero_Request {
  type RmwMsg = super::srv::rmw::GoZero_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        use_mit_mode: msg.use_mit_mode,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      use_mit_mode: msg.use_mit_mode,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      use_mit_mode: msg.use_mit_mode,
    }
  }
}


// Corresponds to panthera_interfaces__srv__GoZero_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoZero_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub code: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for GoZero_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GoZero_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GoZero_Response {
  type RmwMsg = super::srv::rmw::GoZero_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        code: msg.code,
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      code: msg.code,
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      code: msg.code,
      status: msg.status,
    }
  }
}


// Corresponds to panthera_interfaces__srv__GripperSrv_Request

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperSrv_Request {
    /// Target gripper position (meters, 0.0~0.04)
    pub gripper_angle: f64,

    /// Max effort/torque
    pub gripper_effort: f64,

    /// 0=position mode, 1=effort mode
    pub gripper_code: u8,

    /// If true, reset gripper zero position
    pub set_zero: bool,

}



impl Default for GripperSrv_Request {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GripperSrv_Request::default())
  }
}

impl rosidl_runtime_rs::Message for GripperSrv_Request {
  type RmwMsg = super::srv::rmw::GripperSrv_Request;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        gripper_angle: msg.gripper_angle,
        gripper_effort: msg.gripper_effort,
        gripper_code: msg.gripper_code,
        set_zero: msg.set_zero,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      gripper_angle: msg.gripper_angle,
      gripper_effort: msg.gripper_effort,
      gripper_code: msg.gripper_code,
      set_zero: msg.set_zero,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      gripper_angle: msg.gripper_angle,
      gripper_effort: msg.gripper_effort,
      gripper_code: msg.gripper_code,
      set_zero: msg.set_zero,
    }
  }
}


// Corresponds to panthera_interfaces__srv__GripperSrv_Response

// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperSrv_Response {
    /// 0=success, non-zero=error code
    pub code: i64,


    // This member is not documented.
    #[allow(missing_docs)]
    pub status: bool,

}



impl Default for GripperSrv_Response {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::srv::rmw::GripperSrv_Response::default())
  }
}

impl rosidl_runtime_rs::Message for GripperSrv_Response {
  type RmwMsg = super::srv::rmw::GripperSrv_Response;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        code: msg.code,
        status: msg.status,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      code: msg.code,
      status: msg.status,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      code: msg.code,
      status: msg.status,
    }
  }
}






#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__MoveToJoint() -> *const std::ffi::c_void;
}

// Corresponds to panthera_interfaces__srv__MoveToJoint
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveToJoint;

impl rosidl_runtime_rs::Service for MoveToJoint {
    type Request = MoveToJoint_Request;
    type Response = MoveToJoint_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__MoveToJoint() }
    }
}




#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__MoveToPose() -> *const std::ffi::c_void;
}

// Corresponds to panthera_interfaces__srv__MoveToPose
#[allow(missing_docs, non_camel_case_types)]
pub struct MoveToPose;

impl rosidl_runtime_rs::Service for MoveToPose {
    type Request = MoveToPose_Request;
    type Response = MoveToPose_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__MoveToPose() }
    }
}




#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__GripperControl() -> *const std::ffi::c_void;
}

// Corresponds to panthera_interfaces__srv__GripperControl
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperControl;

impl rosidl_runtime_rs::Service for GripperControl {
    type Request = GripperControl_Request;
    type Response = GripperControl_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__GripperControl() }
    }
}




#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__Enable() -> *const std::ffi::c_void;
}

// Corresponds to panthera_interfaces__srv__Enable
#[allow(missing_docs, non_camel_case_types)]
pub struct Enable;

impl rosidl_runtime_rs::Service for Enable {
    type Request = Enable_Request;
    type Response = Enable_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__Enable() }
    }
}




#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__GoZero() -> *const std::ffi::c_void;
}

// Corresponds to panthera_interfaces__srv__GoZero
#[allow(missing_docs, non_camel_case_types)]
pub struct GoZero;

impl rosidl_runtime_rs::Service for GoZero {
    type Request = GoZero_Request;
    type Response = GoZero_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__GoZero() }
    }
}




#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__GripperSrv() -> *const std::ffi::c_void;
}

// Corresponds to panthera_interfaces__srv__GripperSrv
#[allow(missing_docs, non_camel_case_types)]
pub struct GripperSrv;

impl rosidl_runtime_rs::Service for GripperSrv {
    type Request = GripperSrv_Request;
    type Response = GripperSrv_Response;

    fn get_type_support() -> *const std::ffi::c_void {
        // SAFETY: No preconditions for this function.
        unsafe { rosidl_typesupport_c__get_service_type_support_handle__panthera_interfaces__srv__GripperSrv() }
    }
}


