# C++ (cpp) 빌드 — CMake 사용 (Windows)

`cpp` 디렉터리의 CMake 설정은 Windows 환경(Visual Studio / MSVC)을 기본으로 합니다.

간단한 MSVC(Visual Studio) 명령행 예시:

```powershell
mkdir build
cd build
# x64 (Visual Studio 2019/2022 등)
cmake -S .. -B . -G "Visual Studio 18 2026" -A x64
cmake --build . --config Debug

# ARM64 예시
cmake -S .. -B . -G "Visual Studio 18 2026" -A ARM64
cmake --build . --config Debug
```

Visual Studio에서 `Open Folder`로 루트 폴더를 연 뒤 CMake 설정을 통해 빌드할 수도 있습니다.

## Ninja를 사용한 빌드

Ninja 빌드 시스템을 사용하면 더 빠른 빌드가 가능합니다:

```powershell
mkdir build
cd build
# Ninja 제너레이터 사용 (MSVC)
cmake -S .. -B . -G Ninja -DCMAKE_C_COMPILER=cl -DCMAKE_CXX_COMPILER=cl
cmake --build .

# 또는 다음과 같이 릴리스 빌드
cmake -S .. -B . -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_C_COMPILER=cl -DCMAKE_CXX_COMPILER=cl
cmake --build .

# ARM64 빌드 (Ninja)
mkdir build-arm64
cd build-arm64
cmake -S .. -B . -G Ninja -DCMAKE_SYSTEM_PROCESSOR=ARM64 -DCMAKE_C_COMPILER=clarm64 -DCMAKE_CXX_COMPILER=clarm64
cmake --build .
```

참고:
- Ninja가 설치되어 있어야 합니다. (`choco install ninja` 또는 Visual Studio와 함께 설치)
- `CMAKE_C_COMPILER` / `CMAKE_CXX_COMPILER`는 사용 중인 컴파일러에 맞게 조정하세요
- ARM64 빌드 시 `clarm64` (ARM64용 MSVC 컴파일러) 또는 적절한 ARM64 컴파일러를 지정해야 합니다

참고:
- 이 프로젝트는 Windows에서 구성/빌드하도록 설계되어 있습니다. 리눅스에서 mingw 등으로 크로스 빌드하려면 추가 설정이 필요합니다.
- Windows 전용 라이브러리(`Cfgmgr32`, `SetupAPI`)는 타겟이 Windows일 때만 링크됩니다.

문제가 발생하면 빌드 로그와 사용한 `cmake` 명령을 알려주세요.
