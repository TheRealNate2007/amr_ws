#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};



#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__MoveToJoint_Request() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__MoveToJoint_Request__init(msg: *mut MoveToJoint_Request) -> bool;
    fn panthera_interfaces__srv__MoveToJoint_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveToJoint_Request>, size: usize) -> bool;
    fn panthera_interfaces__srv__MoveToJoint_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveToJoint_Request>);
    fn panthera_interfaces__srv__MoveToJoint_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveToJoint_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveToJoint_Request>) -> bool;
}

// Corresponds to panthera_interfaces__srv__MoveToJoint_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__MoveToJoint_Request__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__MoveToJoint_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveToJoint_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToJoint_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToJoint_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToJoint_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveToJoint_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveToJoint_Request where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/MoveToJoint_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__MoveToJoint_Request() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__MoveToJoint_Response() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__MoveToJoint_Response__init(msg: *mut MoveToJoint_Response) -> bool;
    fn panthera_interfaces__srv__MoveToJoint_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveToJoint_Response>, size: usize) -> bool;
    fn panthera_interfaces__srv__MoveToJoint_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveToJoint_Response>);
    fn panthera_interfaces__srv__MoveToJoint_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveToJoint_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveToJoint_Response>) -> bool;
}

// Corresponds to panthera_interfaces__srv__MoveToJoint_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveToJoint_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for MoveToJoint_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__MoveToJoint_Response__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__MoveToJoint_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveToJoint_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToJoint_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToJoint_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToJoint_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveToJoint_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveToJoint_Response where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/MoveToJoint_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__MoveToJoint_Response() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__MoveToPose_Request() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__MoveToPose_Request__init(msg: *mut MoveToPose_Request) -> bool;
    fn panthera_interfaces__srv__MoveToPose_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveToPose_Request>, size: usize) -> bool;
    fn panthera_interfaces__srv__MoveToPose_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveToPose_Request>);
    fn panthera_interfaces__srv__MoveToPose_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveToPose_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveToPose_Request>) -> bool;
}

// Corresponds to panthera_interfaces__srv__MoveToPose_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__MoveToPose_Request__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__MoveToPose_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveToPose_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToPose_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToPose_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToPose_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveToPose_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveToPose_Request where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/MoveToPose_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__MoveToPose_Request() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__MoveToPose_Response() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__MoveToPose_Response__init(msg: *mut MoveToPose_Response) -> bool;
    fn panthera_interfaces__srv__MoveToPose_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<MoveToPose_Response>, size: usize) -> bool;
    fn panthera_interfaces__srv__MoveToPose_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<MoveToPose_Response>);
    fn panthera_interfaces__srv__MoveToPose_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<MoveToPose_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<MoveToPose_Response>) -> bool;
}

// Corresponds to panthera_interfaces__srv__MoveToPose_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct MoveToPose_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for MoveToPose_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__MoveToPose_Response__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__MoveToPose_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for MoveToPose_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToPose_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToPose_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__MoveToPose_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for MoveToPose_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for MoveToPose_Response where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/MoveToPose_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__MoveToPose_Response() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GripperControl_Request() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__GripperControl_Request__init(msg: *mut GripperControl_Request) -> bool;
    fn panthera_interfaces__srv__GripperControl_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Request>, size: usize) -> bool;
    fn panthera_interfaces__srv__GripperControl_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Request>);
    fn panthera_interfaces__srv__GripperControl_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperControl_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Request>) -> bool;
}

// Corresponds to panthera_interfaces__srv__GripperControl_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub action: rosidl_runtime_rs::String,

}



impl Default for GripperControl_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__GripperControl_Request__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__GripperControl_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperControl_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperControl_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperControl_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperControl_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperControl_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperControl_Request where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/GripperControl_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GripperControl_Request() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GripperControl_Response() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__GripperControl_Response__init(msg: *mut GripperControl_Response) -> bool;
    fn panthera_interfaces__srv__GripperControl_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Response>, size: usize) -> bool;
    fn panthera_interfaces__srv__GripperControl_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Response>);
    fn panthera_interfaces__srv__GripperControl_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperControl_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperControl_Response>) -> bool;
}

// Corresponds to panthera_interfaces__srv__GripperControl_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GripperControl_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub success: bool,


    // This member is not documented.
    #[allow(missing_docs)]
    pub message: rosidl_runtime_rs::String,

}



impl Default for GripperControl_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__GripperControl_Response__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__GripperControl_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperControl_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperControl_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperControl_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperControl_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperControl_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperControl_Response where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/GripperControl_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GripperControl_Response() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__Enable_Request() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__Enable_Request__init(msg: *mut Enable_Request) -> bool;
    fn panthera_interfaces__srv__Enable_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Enable_Request>, size: usize) -> bool;
    fn panthera_interfaces__srv__Enable_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Enable_Request>);
    fn panthera_interfaces__srv__Enable_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Enable_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<Enable_Request>) -> bool;
}

// Corresponds to panthera_interfaces__srv__Enable_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Enable_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub enable_request: bool,

}



impl Default for Enable_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__Enable_Request__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__Enable_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Enable_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__Enable_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__Enable_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__Enable_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Enable_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Enable_Request where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/Enable_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__Enable_Request() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__Enable_Response() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__Enable_Response__init(msg: *mut Enable_Response) -> bool;
    fn panthera_interfaces__srv__Enable_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<Enable_Response>, size: usize) -> bool;
    fn panthera_interfaces__srv__Enable_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<Enable_Response>);
    fn panthera_interfaces__srv__Enable_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<Enable_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<Enable_Response>) -> bool;
}

// Corresponds to panthera_interfaces__srv__Enable_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Enable_Response {

    // This member is not documented.
    #[allow(missing_docs)]
    pub enable_response: bool,

}



impl Default for Enable_Response {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__Enable_Response__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__Enable_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for Enable_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__Enable_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__Enable_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__Enable_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for Enable_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for Enable_Response where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/Enable_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__Enable_Response() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GoZero_Request() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__GoZero_Request__init(msg: *mut GoZero_Request) -> bool;
    fn panthera_interfaces__srv__GoZero_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoZero_Request>, size: usize) -> bool;
    fn panthera_interfaces__srv__GoZero_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoZero_Request>);
    fn panthera_interfaces__srv__GoZero_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoZero_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GoZero_Request>) -> bool;
}

// Corresponds to panthera_interfaces__srv__GoZero_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct GoZero_Request {

    // This member is not documented.
    #[allow(missing_docs)]
    pub use_mit_mode: bool,

}



impl Default for GoZero_Request {
  fn default() -> Self {
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__GoZero_Request__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__GoZero_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoZero_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GoZero_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GoZero_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GoZero_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoZero_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoZero_Request where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/GoZero_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GoZero_Request() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GoZero_Response() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__GoZero_Response__init(msg: *mut GoZero_Response) -> bool;
    fn panthera_interfaces__srv__GoZero_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GoZero_Response>, size: usize) -> bool;
    fn panthera_interfaces__srv__GoZero_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GoZero_Response>);
    fn panthera_interfaces__srv__GoZero_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GoZero_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GoZero_Response>) -> bool;
}

// Corresponds to panthera_interfaces__srv__GoZero_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__GoZero_Response__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__GoZero_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GoZero_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GoZero_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GoZero_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GoZero_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GoZero_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GoZero_Response where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/GoZero_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GoZero_Response() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GripperSrv_Request() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__GripperSrv_Request__init(msg: *mut GripperSrv_Request) -> bool;
    fn panthera_interfaces__srv__GripperSrv_Request__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperSrv_Request>, size: usize) -> bool;
    fn panthera_interfaces__srv__GripperSrv_Request__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperSrv_Request>);
    fn panthera_interfaces__srv__GripperSrv_Request__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperSrv_Request>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperSrv_Request>) -> bool;
}

// Corresponds to panthera_interfaces__srv__GripperSrv_Request
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__GripperSrv_Request__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__GripperSrv_Request__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperSrv_Request {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperSrv_Request__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperSrv_Request__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperSrv_Request__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperSrv_Request {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperSrv_Request where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/GripperSrv_Request";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GripperSrv_Request() }
  }
}


#[link(name = "panthera_interfaces__rosidl_typesupport_c")]
extern "C" {
    fn rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GripperSrv_Response() -> *const std::ffi::c_void;
}

#[link(name = "panthera_interfaces__rosidl_generator_c")]
extern "C" {
    fn panthera_interfaces__srv__GripperSrv_Response__init(msg: *mut GripperSrv_Response) -> bool;
    fn panthera_interfaces__srv__GripperSrv_Response__Sequence__init(seq: *mut rosidl_runtime_rs::Sequence<GripperSrv_Response>, size: usize) -> bool;
    fn panthera_interfaces__srv__GripperSrv_Response__Sequence__fini(seq: *mut rosidl_runtime_rs::Sequence<GripperSrv_Response>);
    fn panthera_interfaces__srv__GripperSrv_Response__Sequence__copy(in_seq: &rosidl_runtime_rs::Sequence<GripperSrv_Response>, out_seq: *mut rosidl_runtime_rs::Sequence<GripperSrv_Response>) -> bool;
}

// Corresponds to panthera_interfaces__srv__GripperSrv_Response
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]


// This struct is not documented.
#[allow(missing_docs)]

#[allow(non_camel_case_types)]
#[repr(C)]
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
    unsafe {
      let mut msg = std::mem::zeroed();
      if !panthera_interfaces__srv__GripperSrv_Response__init(&mut msg as *mut _) {
        panic!("Call to panthera_interfaces__srv__GripperSrv_Response__init() failed");
      }
      msg
    }
  }
}

impl rosidl_runtime_rs::SequenceAlloc for GripperSrv_Response {
  fn sequence_init(seq: &mut rosidl_runtime_rs::Sequence<Self>, size: usize) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperSrv_Response__Sequence__init(seq as *mut _, size) }
  }
  fn sequence_fini(seq: &mut rosidl_runtime_rs::Sequence<Self>) {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperSrv_Response__Sequence__fini(seq as *mut _) }
  }
  fn sequence_copy(in_seq: &rosidl_runtime_rs::Sequence<Self>, out_seq: &mut rosidl_runtime_rs::Sequence<Self>) -> bool {
    // SAFETY: This is safe since the pointer is guaranteed to be valid/initialized.
    unsafe { panthera_interfaces__srv__GripperSrv_Response__Sequence__copy(in_seq, out_seq as *mut _) }
  }
}

impl rosidl_runtime_rs::Message for GripperSrv_Response {
  type RmwMsg = Self;
  fn into_rmw_message(msg_cow: std::borrow::Cow<'_, Self>) -> std::borrow::Cow<'_, Self::RmwMsg> { msg_cow }
  fn from_rmw_message(msg: Self::RmwMsg) -> Self { msg }
}

impl rosidl_runtime_rs::RmwMessage for GripperSrv_Response where Self: Sized {
  const TYPE_NAME: &'static str = "panthera_interfaces/srv/GripperSrv_Response";
  fn get_type_support() -> *const std::ffi::c_void {
    // SAFETY: No preconditions for this function.
    unsafe { rosidl_typesupport_c__get_message_type_support_handle__panthera_interfaces__srv__GripperSrv_Response() }
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


