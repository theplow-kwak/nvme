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

### ARM64 빌드 문제 해결

- 에러 예시: `CMAKE_CXX_COMPILER: clarm64 is not a full path and was not found in the path` — 이는 `clarm64`라는 실행 파일을 PATH에서 찾을 수 없어서 발생합니다. 기본적으로 MSVC의 컴파일러 실행 파일 이름은 `cl.exe`이며, ARM64용 `cl.exe`는 Visual Studio 설치 디렉터리의 `bin/Hostx64/arm64` 같은 하위 폴더에 있습니다.

해결 방법:

1. Visual Studio의 개발자 명령 프롬프트(또는 `vcvarsall.bat`)로 ARM64 툴체인 환경을 활성화한 뒤 CMake를 실행하세요. 그러면 `cl.exe`가 PATH에 추가되어 CMake가 자동으로 컴파일러를 찾습니다.

```powershell
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat" x64_arm64
mkdir build-arm64
cd build-arm64
cmake -S .. -B . -G Ninja
cmake --build . --config Release
```

2. 또는 ARM64용 `cl.exe`의 전체 경로를 직접 지정하세요 (Visual Studio 설치 경로는 환경에 따라 다름):

```powershell
cmake -S .. -B build-arm64 -G Ninja \
	-DCMAKE_C_COMPILER="C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/<version>/bin/Hostx64/arm64/cl.exe" \
	-DCMAKE_CXX_COMPILER="C:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/<version>/bin/Hostx64/arm64/cl.exe"
cmake --build build-arm64 --config Release
```

참고: `clarm64` 같은 이름은 기본적으로 존재하지 않습니다. 대신 `cl.exe`(ARM64용)를 PATH에 놓거나 전체 경로를 지정해야 합니다.

### 정적 빌드(정적 런타임 / 정적 실행파일)

프로젝트의 CMake 설정에는 MSVC 정적 런타임 및 일반 정적 실행 파일 시도를 위한 옵션이 추가되어 있습니다.


- 정적 런타임 또는 정적 실행파일을 원하면 `USE_STATIC_RUNTIME=ON`을 지정하세요. (MSVC에서는 `/MT` 계열 런타임을 사용하도록 설정하고, GCC/Clang에서는 `-static` 링커 플래그를 추가합니다.)

```powershell
cmake -S . -B build -G Ninja -DUSE_STATIC_RUNTIME=ON
cmake --build build --config Release
```

참고:
- Windows에서 완전한(모든 DLL을 제거한) 정적 실행은 일반적으로 어렵습니다. 보통은 CRT만 정적으로 링크(`/MT`)하는 방식으로 충분합니다.
- 정적 빌드 옵션은 플랫폼과 라이브러리에 따라 링크 에러를 일으킬 수 있으니 필요 시 의존성(예: SetupAPI 같은 Windows 전용 DLL)에 대한 처리를 확인하세요.

참고:
- Ninja가 설치되어 있어야 합니다. (`choco install ninja` 또는 Visual Studio와 함께 설치)
- `CMAKE_C_COMPILER` / `CMAKE_CXX_COMPILER`는 사용 중인 컴파일러에 맞게 조정하세요
- ARM64 빌드 시 `clarm64` (ARM64용 MSVC 컴파일러) 또는 적절한 ARM64 컴파일러를 지정해야 합니다

참고:
- 이 프로젝트는 Windows에서 구성/빌드하도록 설계되어 있습니다. 리눅스에서 mingw 등으로 크로스 빌드하려면 추가 설정이 필요합니다.
- Windows 전용 라이브러리(`Cfgmgr32`, `SetupAPI`)는 타겟이 Windows일 때만 링크됩니다.

문제가 발생하면 빌드 로그와 사용한 `cmake` 명령을 알려주세요.
