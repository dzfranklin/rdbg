cmake_minimum_required(VERSION 3.10)
project(wrapper)
include(ExternalProject)

# ExternalProject_Add(rr
#   PREFIX rr
#   SOURCE_DIR "${CMAKE_CURRENT_SOURCE_DIR}/vendor/rr"

#   CMAKE_ARGS
#     -DRR_BUILD_SHARED=ON
#     -DCMAKE_INSTALL_PREFIX=wrapper/build
# )
add_subdirectory("vendor/rr")
include_directories("${CMAKE_CURRENT_BINARY_DIR}/rr/src/rr-build")

add_library(wrapper STATIC wrapper.cpp)
set_target_properties(wrapper PROPERTIES PUBLIC_HEADER wrapper.h)
add_dependencies(wrapper rr)
target_link_libraries(wrapper rr)
install(TARGETS wrapper
  LIBRARY DESTINATION "wrapper/build/lib"
  PUBLIC_HEADER DESTINATION "wrapper/build/include"
)
