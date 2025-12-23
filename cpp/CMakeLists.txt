# CMake 최소 요구 버전 설정
cmake_minimum_required(VERSION 3.21)

# 프로젝트 이름과 C++ 언어 설정
project(nvme-cpp-tool LANGUAGES CXX)

# C++20 표준을 사용하도록 설정
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# 옵션: 정적 런타임(/MT) 사용 여부 및 정적 실행 파일 시도
option(USE_STATIC_MSVC_RUNTIME "Use static MSVC runtime (/MT)" OFF)
option(BUILD_STATIC_EXECUTABLE "Attempt to build a static executable (adds -static for non-MSVC)" OFF)

# MSVC에서 정적 런타임을 사용하도록 설정 (CMake >= 3.15에서 권장 방식)
if(MSVC AND USE_STATIC_MSVC_RUNTIME)
    set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
    message(STATUS "Using static MSVC runtime (MultiThreaded/MultithreadedDebug)")
endif()

# BUILD_STATIC_EXECUTABLE이 설정되면 비-MSVC에서는 -static을 링커에 추가합니다.
if(BUILD_STATIC_EXECUTABLE)
    if(MSVC)
        if(NOT USE_STATIC_MSVC_RUNTIME)
            set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")
        endif()
        message(STATUS "Requested static executable on MSVC: static runtime enabled")
    else()
        if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
            set(CMAKE_EXE_LINKER_FLAGS "${CMAKE_EXE_LINKER_FLAGS} -static")
            message(STATUS "Added -static to linker flags for static executable")
        endif()
    endif()
endif()

# 이 리포지토리의 CMake 설정은 Windows에서만 사용됩니다.
if(NOT WIN32)
    message(FATAL_ERROR "This project is intended to be configured and built on Windows only.")
endif()

# 실행 파일 이름 설정
set(EXECUTABLE_NAME "nvme")

# 소스 파일 목록 정의
set(SOURCES
    main.cpp
    dev_utils.cpp
    disk.cpp
    lib.cpp
    nvme_device.cpp
    nvme_print.cpp
    scsi.cpp
)

# 실행 파일 타겟 생성
add_executable(${EXECUTABLE_NAME} ${SOURCES})

# 필요한 Windows 라이브러리 링크
# Cfgmgr32: 장치 관리에 필요 (dev_utils.cpp)
# SetupAPI: 장치 정보 조회에 필요 (nvme_device.cpp)
target_link_libraries(${EXECUTABLE_NAME} PRIVATE
    Cfgmgr32
    SetupAPI
)

# MSVC 컴파일러를 사용할 경우, 경고 수준을 높이고 경고를 오류로 처리하여 코드 품질 향상
if(MSVC)
    target_compile_options(${EXECUTABLE_NAME} PRIVATE /W2 /WX)
    target_compile_options(${EXECUTABLE_NAME} PRIVATE /permissive-)
else()
    if(CMAKE_CXX_COMPILER_ID STREQUAL "GNU" OR CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
        target_compile_options(${EXECUTABLE_NAME} PRIVATE -Wall -Wextra)
    endif()
endif()

# Print configuration summary
message(STATUS "Configuration summary:")
message(STATUS "  Host: ${CMAKE_HOST_SYSTEM_NAME} ${CMAKE_HOST_SYSTEM_PROCESSOR}")
message(STATUS "  Target: Windows ${CMAKE_SYSTEM_PROCESSOR}")