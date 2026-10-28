#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



// Corresponds to panthera_interfaces__msg__ArmPose

// This struct is not documented.
#[allow(missing_docs)]

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmPose {

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


    // This member is not documented.
    #[allow(missing_docs)]
    pub cartesian_path: bool,

}



impl Default for ArmPose {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ArmPose::default())
  }
}

impl rosidl_runtime_rs::Message for ArmPose {
  type RmwMsg = super::msg::rmw::ArmPose;

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
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
      cartesian_path: msg.cartesian_path,
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
    }
  }
}


// Corresponds to panthera_interfaces__msg__ArmStatus
/// ArmStatus.msg - Arm status feedback

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,

    /// Overall arm state
    /// Whether the arm is enabled and accepting commands
    pub arm_enabled: bool,

    /// 0=idle, 1=moving, 2=error
    pub motion_status: u8,

    /// Human-readable error description
    pub error_message: std::string::String,

    /// Per-motor mode and fault
    /// Control mode per motor
    pub motor_modes: [u8; 6],

    /// Fault code per motor (0 = no fault)
    pub motor_faults: [u8; 6],

    /// Joint limit flags: true means the joint has hit its position limit
    pub joint_at_limit: [bool; 6],

    /// Gripper state
    /// Gripper position
    pub gripper_position: f64,

    /// Gripper motor fault code
    pub gripper_fault: u8,

}



impl Default for ArmStatus {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::ArmStatus::default())
  }
}

impl rosidl_runtime_rs::Message for ArmStatus {
  type RmwMsg = super::msg::rmw::ArmStatus;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        arm_enabled: msg.arm_enabled,
        motion_status: msg.motion_status,
        error_message: msg.error_message.as_str().into(),
        motor_modes: msg.motor_modes,
        motor_faults: msg.motor_faults,
        joint_at_limit: msg.joint_at_limit,
        gripper_position: msg.gripper_position,
        gripper_fault: msg.gripper_fault,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      arm_enabled: msg.arm_enabled,
      motion_status: msg.motion_status,
        error_message: msg.error_message.as_str().into(),
        motor_modes: msg.motor_modes,
        motor_faults: msg.motor_faults,
        joint_at_limit: msg.joint_at_limit,
      gripper_position: msg.gripper_position,
      gripper_fault: msg.gripper_fault,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      arm_enabled: msg.arm_enabled,
      motion_status: msg.motion_status,
      error_message: msg.error_message.to_string(),
      motor_modes: msg.motor_modes,
      motor_faults: msg.motor_faults,
      joint_at_limit: msg.joint_at_limit,
      gripper_position: msg.gripper_position,
      gripper_fault: msg.gripper_fault,
    }
  }
}


// Corresponds to panthera_interfaces__msg__EndPoseEuler
/// EndPoseEuler.msg - End-effector pose feedback in Euler angles

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EndPoseEuler {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::Header,


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

}



impl Default for EndPoseEuler {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::EndPoseEuler::default())
  }
}

impl rosidl_runtime_rs::Message for EndPoseEuler {
  type RmwMsg = super::msg::rmw::EndPoseEuler;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Owned(msg.header)).into_owned(),
        x: msg.x,
        y: msg.y,
        z: msg.z,
        roll: msg.roll,
        pitch: msg.pitch,
        yaw: msg.yaw,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        header: std_msgs::msg::Header::into_rmw_message(std::borrow::Cow::Borrowed(&msg.header)).into_owned(),
      x: msg.x,
      y: msg.y,
      z: msg.z,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
      })
    }
  }

  fn from_rmw_message(msg: Self::RmwMsg) -> Self {
    Self {
      header: std_msgs::msg::Header::from_rmw_message(msg.header),
      x: msg.x,
      y: msg.y,
      z: msg.z,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
    }
  }
}


// Corresponds to panthera_interfaces__msg__PosCmd
/// PosCmd.msg - Combined end-effector pose + gripper command

#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct PosCmd {

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

    /// Gripper value (0.0=close, 1.0=fully open)
    pub gripper: f64,

    /// 0=joint-space planning, 1=cartesian path
    pub mode1: u8,

    /// Reserved for future use
    pub mode2: u8,

}



impl Default for PosCmd {
  fn default() -> Self {
    <Self as rosidl_runtime_rs::Message>::from_rmw_message(super::msg::rmw::PosCmd::default())
  }
}

impl rosidl_runtime_rs::Message for PosCmd {
  type RmwMsg = super::msg::rmw::PosCmd;

  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> {
    match msg_cow {
      std::borrow::Cow::Owned(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
        x: msg.x,
        y: msg.y,
        z: msg.z,
        roll: msg.roll,
        pitch: msg.pitch,
        yaw: msg.yaw,
        gripper: msg.gripper,
        mode1: msg.mode1,
        mode2: msg.mode2,
      }),
      std::borrow::Cow::Borrowed(msg) => std::borrow::Cow::Owned(Self::RmwMsg {
      x: msg.x,
      y: msg.y,
      z: msg.z,
      roll: msg.roll,
      pitch: msg.pitch,
      yaw: msg.yaw,
      gripper: msg.gripper,
      mode1: msg.mode1,
      mode2: msg.mode2,
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
      gripper: msg.gripper,
      mode1: msg.mode1,
      mode2: msg.mode2,
    }
  }
}


