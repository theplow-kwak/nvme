# Set the minimum version of CMake that can be used
cmake_minimum_required(VERSION 3.15)

# Set the project name
project(nvme_cpp_tool VERSION 1.0)

# Set the C++ standard to 20
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Add the executable target with its source files
add_executable(${PROJECT_NAME}
    main.cpp
    nvme_device.cpp
    nvme_print.cpp
)

# Link against required Windows libraries
target_link_libraries(${PROJECT_NAME} PRIVATE SetupAPI)

# Optional: Set output directory for the executable
set(CMAKE_ARCHIVE_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_LIBRARY_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/lib)
set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

# Print a message to the user with build instructions
message(STATUS "CMake project created. To build, run the following commands:")
message(STATUS "  cd cpp")
message(STATUS "  mkdir build")
message(STATUS "  cd build")
message(STATUS "  cmake ..")
message(STATUS "  cmake --build .")
message(STATUS "The executable will be in the 'cpp/build/bin' directory.")
