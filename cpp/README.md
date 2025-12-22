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

참고:
- 이 프로젝트는 Windows에서 구성/빌드하도록 설계되어 있습니다. 리눅스에서 mingw 등으로 크로스 빌드하려면 추가 설정이 필요합니다.
- Windows 전용 라이브러리(`Cfgmgr32`, `SetupAPI`)는 타겟이 Windows일 때만 링크됩니다.

문제가 발생하면 빌드 로그와 사용한 `cmake` 명령을 알려주세요.
