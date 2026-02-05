cmake_minimum_required(VERSION 3.21)
project(nvme-cpp-tool LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

option(USE_STATIC_RUNTIME "Use static runtime (/MT on MSVC) or build static executable (-static on GCC/Clang)" OFF)

if(USE_STATIC_RUNTIME)
    if(MSVC)
        set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
        message(STATUS "Requested static build on MSVC: static runtime enabled (MultiThreaded/MultithreadedDebug)")
    else()
        if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
            set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -static")
            message(STATUS "Requested static build on non-MSVC: added -static to linker flags")
        endif()
    endif()
endif()

if(NOT WIN32)
    message(FATAL_ERROR "This project is intended to be configured and built on Windows only.")
endif()

set(EXECUTABLE_NAME "nvme")
set(SOURCES
    main.cpp
    dev_utils.cpp
    disk.cpp
    lib.cpp
    nvme_device.cpp
    nvme_print.cpp
    scsi.cpp
)

add_executable(${EXECUTABLE_NAME} ${SOURCES})

target_link_libraries(${EXECUTABLE_NAME} PRIVATE
    Cfgmgr32
    SetupAPI
)

if(MSVC)
    target_compile_options(${EXECUTABLE_NAME} PRIVATE /W2 /WX)
    target_compile_options(${EXECUTABLE_NAME} PRIVATE /permissive-)
else()
    if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
        target_compile_options(${EXECUTABLE_NAME} PRIVATE -Wall -Wextra)
    endif()
endif()

message(STATUS "Configuration summary:")
message(STATUS "  Host: ${CMAKE_HOST_SYSTEM_NAME} ${CMAKE_HOST_SYSTEM_PROCESSOR}")
message(STATUS "  Target: Windows ${CMAKE_SYSTEM_PROCESSOR}")