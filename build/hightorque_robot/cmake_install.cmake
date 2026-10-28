# Install script for directory: /home/amr-pc/amr_ws/src/hightorque_robot

# Set the install prefix
if(NOT DEFINED CMAKE_INSTALL_PREFIX)
  set(CMAKE_INSTALL_PREFIX "/home/amr-pc/amr_ws/install/hightorque_robot")
endif()
string(REGEX REPLACE "/$" "" CMAKE_INSTALL_PREFIX "${CMAKE_INSTALL_PREFIX}")

# Set the install configuration name.
if(NOT DEFINED CMAKE_INSTALL_CONFIG_NAME)
  if(BUILD_TYPE)
    string(REGEX REPLACE "^[^A-Za-z0-9_]+" ""
           CMAKE_INSTALL_CONFIG_NAME "${BUILD_TYPE}")
  else()
    set(CMAKE_INSTALL_CONFIG_NAME "Release")
  endif()
  message(STATUS "Install configuration: \"${CMAKE_INSTALL_CONFIG_NAME}\"")
endif()

# Set the component getting installed.
if(NOT CMAKE_INSTALL_COMPONENT)
  if(COMPONENT)
    message(STATUS "Install component: \"${COMPONENT}\"")
    set(CMAKE_INSTALL_COMPONENT "${COMPONENT}")
  else()
    set(CMAKE_INSTALL_COMPONENT)
  endif()
endif()

# Install shared libraries without execute permission?
if(NOT DEFINED CMAKE_INSTALL_SO_NO_EXE)
  set(CMAKE_INSTALL_SO_NO_EXE "1")
endif()

# Is this installation the result of a crosscompile?
if(NOT DEFINED CMAKE_CROSSCOMPILING)
  set(CMAKE_CROSSCOMPILING "FALSE")
endif()

# Set default install directory permissions.
if(NOT DEFINED CMAKE_OBJDUMP)
  set(CMAKE_OBJDUMP "/usr/bin/objdump")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  include("/home/amr-pc/amr_ws/build/hightorque_robot/ament_cmake_symlink_install/ament_cmake_symlink_install.cmake")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  foreach(file
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libhightorque_robot.so.4.4.7"
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libhightorque_robot.so.4"
      )
    if(EXISTS "${file}" AND
       NOT IS_SYMLINK "${file}")
      file(RPATH_CHECK
           FILE "${file}"
           RPATH "")
    endif()
  endforeach()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE SHARED_LIBRARY FILES
    "/home/amr-pc/amr_ws/build/hightorque_robot/libhightorque_robot.so.4.4.7"
    "/home/amr-pc/amr_ws/build/hightorque_robot/libhightorque_robot.so.4"
    )
  foreach(file
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libhightorque_robot.so.4.4.7"
      "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/libhightorque_robot.so.4"
      )
    if(EXISTS "${file}" AND
       NOT IS_SYMLINK "${file}")
      file(RPATH_CHANGE
           FILE "${file}"
           OLD_RPATH "/home/amr-pc/amr_ws/build/hightorque_robot/third_part/lcm/lcm:/home/amr-pc/amr_ws/build/hightorque_robot/third_part/serial_cmake:"
           NEW_RPATH "")
      if(CMAKE_INSTALL_DO_STRIP)
        execute_process(COMMAND "/usr/bin/strip" "${file}")
      endif()
    endif()
  endforeach()
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib" TYPE SHARED_LIBRARY FILES "/home/amr-pc/amr_ws/build/hightorque_robot/libhightorque_robot.so")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE FILE FILES
    "/home/amr-pc/amr_ws/src/hightorque_robot/include/hardware/robot.hpp"
    "/home/amr-pc/amr_ws/src/hightorque_robot/include/hardware/motor.hpp"
    "/home/amr-pc/amr_ws/src/hightorque_robot/include/hardware/canport.hpp"
    "/home/amr-pc/amr_ws/src/hightorque_robot/include/hardware/canboard.hpp"
    "/home/amr-pc/amr_ws/src/hightorque_robot/include/parse_robot_params.hpp"
    "/home/amr-pc/amr_ws/src/hightorque_robot/include/serial_struct.hpp"
    "/home/amr-pc/amr_ws/src/hightorque_robot/include/serial_driver.hpp"
    "/home/amr-pc/amr_ws/src/hightorque_robot/include/crc/crc8.hpp"
    "/home/amr-pc/amr_ws/src/hightorque_robot/include/crc/crc16.hpp"
    "/home/amr-pc/amr_ws/src/hightorque_robot/include/panthera/Panthera.hpp"
    "/home/amr-pc/amr_ws/src/hightorque_robot/msg/motor_msg/motor_msg.hpp"
    )
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include" TYPE DIRECTORY FILES "/home/amr-pc/amr_ws/src/hightorque_robot/include/" FILES_MATCHING REGEX "/[^/]*\\.hpp$" REGEX "/[^/]*\\.h$")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/include/motor_msg" TYPE DIRECTORY FILES "/home/amr-pc/amr_ws/src/hightorque_robot/msg/motor_msg/" FILES_MATCHING REGEX "/[^/]*\\.hpp$" REGEX "/[^/]*\\.h$")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/hightorque_robot/robot_param" TYPE DIRECTORY FILES "/home/amr-pc/amr_ws/src/hightorque_robot/robot_param/" FILES_MATCHING REGEX "/[^/]*\\.yaml$")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/share/hightorque_robot/urdf" TYPE DIRECTORY FILES "/home/amr-pc/amr_ws/src/hightorque_robot/Panthera-HT_description/urdf/" FILES_MATCHING REGEX "/[^/]*\\.urdf$")
endif()

if(CMAKE_INSTALL_COMPONENT STREQUAL "Unspecified" OR NOT CMAKE_INSTALL_COMPONENT)
  if(EXISTS "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/hightorque_robot/hightorque_robotTargets.cmake")
    file(DIFFERENT _cmake_export_file_changed FILES
         "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/hightorque_robot/hightorque_robotTargets.cmake"
         "/home/amr-pc/amr_ws/build/hightorque_robot/CMakeFiles/Export/8f991f9db2e053642bbf2fc328252ce5/hightorque_robotTargets.cmake")
    if(_cmake_export_file_changed)
      file(GLOB _cmake_old_config_files "$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/hightorque_robot/hightorque_robotTargets-*.cmake")
      if(_cmake_old_config_files)
        string(REPLACE ";" ", " _cmake_old_config_files_text "${_cmake_old_config_files}")
        message(STATUS "Old export file \"$ENV{DESTDIR}${CMAKE_INSTALL_PREFIX}/lib/cmake/hightorque_robot/hightorque_robotTargets.cmake\" will be replaced.  Removing files [${_cmake_old_config_files_text}].")
        unset(_cmake_old_config_files_text)
        file(REMOVE ${_cmake_old_config_files})
      endif()
      unset(_cmake_old_config_files)
    endif()
    unset(_cmake_export_file_changed)
  endif()
  file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/hightorque_robot" TYPE FILE FILES "/home/amr-pc/amr_ws/build/hightorque_robot/CMakeFiles/Export/8f991f9db2e053642bbf2fc328252ce5/hightorque_robotTargets.cmake")
  if(CMAKE_INSTALL_CONFIG_NAME MATCHES "^([Rr][Ee][Ll][Ee][Aa][Ss][Ee])$")
    file(INSTALL DESTINATION "${CMAKE_INSTALL_PREFIX}/lib/cmake/hightorque_robot" TYPE FILE FILES "/home/amr-pc/amr_ws/build/hightorque_robot/CMakeFiles/Export/8f991f9db2e053642bbf2fc328252ce5/hightorque_robotTargets-release.cmake")
  endif()
endif()

if(NOT CMAKE_INSTALL_LOCAL_ONLY)
  # Include the install script for each subdirectory.
  include("/home/amr-pc/amr_ws/build/hightorque_robot/third_part/lcm/cmake_install.cmake")
  include("/home/amr-pc/amr_ws/build/hightorque_robot/third_part/serial_cmake/cmake_install.cmake")

endif()

if(CMAKE_INSTALL_COMPONENT)
  set(CMAKE_INSTALL_MANIFEST "install_manifest_${CMAKE_INSTALL_COMPONENT}.txt")
else()
  set(CMAKE_INSTALL_MANIFEST "install_manifest.txt")
endif()

string(REPLACE ";" "\n" CMAKE_INSTALL_MANIFEST_CONTENT
       "${CMAKE_INSTALL_MANIFEST_FILES}")
file(WRITE "/home/amr-pc/amr_ws/build/hightorque_robot/${CMAKE_INSTALL_MANIFEST}"
     "${CMAKE_INSTALL_MANIFEST_CONTENT}")
