# generated from rosidl_cmake/cmake/rosidl_cmake_aggregate_target-extras.cmake.in

# Create a convenience aggregate target panthera_interfaces::panthera_interfaces
# that links all generated interface targets, so downstream packages can use
# a single modern CMake target name instead of ${panthera_interfaces_TARGETS}.
if(panthera_interfaces_TARGETS AND NOT TARGET panthera_interfaces::panthera_interfaces)
  add_library(panthera_interfaces::panthera_interfaces INTERFACE IMPORTED)
  set_target_properties(panthera_interfaces::panthera_interfaces PROPERTIES
    INTERFACE_LINK_LIBRARIES "${panthera_interfaces_TARGETS}")
endif()
