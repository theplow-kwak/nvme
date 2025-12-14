# CMake 최소 요구 버전 설정
cmake_minimum_required(VERSION 3.21)

# 프로젝트 이름과 C++ 언어 설정
project(nvme-cpp-tool LANGUAGES CXX)

# C++20 표준을 사용하도록 설정
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Windows 프로젝트일 경우에만 빌드 진행
if(NOT WIN32)
    message(FATAL_ERROR "This project can only be built on Windows.")
endif()

# 실행 파일 이름 설정
set(EXECUTABLE_NAME "nvme-cpp")

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
    target_compile_options(${EXECUTABLE_NAME} PRIVATE /W4 /WX)
endif()