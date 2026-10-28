#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__msg__ArmPose() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__msg__ArmPose__init(msg: *mut ArmPose) -> bool;
    fn panthera_interfaces__msg__ArmPose__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ArmPose>, size: usize) -> bool;
    fn panthera_interfaces__msg__ArmPose__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ArmPose>);
    fn panthera_interfaces__msg__ArmPose__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ArmPose>, out_seq: *mut rosidl_runtime_rs::Sequence<ArmPose>) -> bool;
}

// Corresponds to panthera_interfaces__msg__ArmPose
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__msg__ArmPose__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__msg__ArmPose__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ArmPose {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__ArmPose__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__ArmPose__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__ArmPose__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ArmPose {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ArmPose where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/msg/ArmPose";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__msg__ArmPose() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__msg__ArmStatus() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__msg__ArmStatus__init(msg: *mut ArmStatus) -> bool;
    fn panthera_interfaces__msg__ArmStatus__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<ArmStatus>, size: usize) -> bool;
    fn panthera_interfaces__msg__ArmStatus__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<ArmStatus>);
    fn panthera_interfaces__msg__ArmStatus__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<ArmStatus>, out_seq: *mut rosidl_runtime_rs::Sequence<ArmStatus>) -> bool;
}

// Corresponds to panthera_interfaces__msg__ArmStatus
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// ArmStatus.msg - Arm status feedback

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct ArmStatus {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,

    /// Overall arm state
    /// Whether the arm is enabled and accepting commands
    pub arm_enabled: bool,

    /// 0=idle, 1=moving, 2=error
    pub motion_status: u8,

    /// Human-readable error description
    pub error_message: rosidl_runtime_rs::String,

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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__msg__ArmStatus__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__msg__ArmStatus__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for ArmStatus {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__ArmStatus__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__ArmStatus__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__ArmStatus__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for ArmStatus {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for ArmStatus where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/msg/ArmStatus";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__msg__ArmStatus() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__msg__EndPoseEuler() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__msg__EndPoseEuler__init(msg: *mut EndPoseEuler) -> bool;
    fn panthera_interfaces__msg__EndPoseEuler__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<EndPoseEuler>, size: usize) -> bool;
    fn panthera_interfaces__msg__EndPoseEuler__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<EndPoseEuler>);
    fn panthera_interfaces__msg__EndPoseEuler__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<EndPoseEuler>, out_seq: *mut rosidl_runtime_rs::Sequence<EndPoseEuler>) -> bool;
}

// Corresponds to panthera_interfaces__msg__EndPoseEuler
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// EndPoseEuler.msg - End-effector pose feedback in Euler angles

#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct EndPoseEuler {

    // This member is not documented.
    #[allow(missing_docs)]
    pub header: std_msgs::msg::rmw::Header,


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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__msg__EndPoseEuler__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__msg__EndPoseEuler__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for EndPoseEuler {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__EndPoseEuler__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__EndPoseEuler__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__EndPoseEuler__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for EndPoseEuler {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for EndPoseEuler where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/msg/EndPoseEuler";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__msg__EndPoseEuler() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__msg__PosCmd() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__msg__PosCmd__init(msg: *mut PosCmd) -> bool;
    fn panthera_interfaces__msg__PosCmd__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<PosCmd>, size: usize) -> bool;
    fn panthera_interfaces__msg__PosCmd__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<PosCmd>);
    fn panthera_interfaces__msg__PosCmd__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<PosCmd>, out_seq: *mut rosidl_runtime_rs::Sequence<PosCmd>) -> bool;
}

// Corresponds to panthera_interfaces__msg__PosCmd
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

/// PosCmd.msg - Combined end-effector pose + gripper command

#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__msg__PosCmd__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__msg__PosCmd__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for PosCmd {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__PosCmd__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__PosCmd__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__msg__PosCmd__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for PosCmd {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for PosCmd where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/msg/PosCmd";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__msg__PosCmd() }
  }
}


