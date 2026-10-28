#----------------------------------------------------------------
# Generated CMake target import file.
#----------------------------------------------------------------

# Commands may need to know the format version.
set(CMAKE_IMPORT_FILE_VERSION 1)

# Import target "amr_hardware::amr_hardware" for configuration ""
set_property(TARGET amr_hardware::amr_hardware APPEND PROPERTY IMPORTED_CONFIGURATIONS NOCONFIG)
set_target_properties(amr_hardware::amr_hardware PROPERTIES
  IMPORTED_LOCATION_NOCONFIG "${_IMPORT_PREFIX}/lib/libamr_hardware.so"
  IMPORTED_SONAME_NOCONFIG "libamr_hardware.so"
  )

list(APPEND _cmake_import_check_targets amr_hardware::amr_hardware )
list(APPEND _cmake_import_check_files_for_amr_hardware::amr_hardware "${_IMPORT_PREFIX}/lib/libamr_hardware.so" )

# Commands beyond this point should not need to know the version.
set(CMAKE_IMPORT_FILE_VERSION)
