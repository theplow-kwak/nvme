# C++ (cpp) 빌드 — CMake 사용 (Windows)

이 문서는 `cpp` 디렉터리의 C++ 소스 코드를 CMake와 Ninja를 사용하여 Windows 환경에서 빌드하는 방법을 안내합니다.

## 사전 요구 사항

1.  **Visual Studio 2026**: C++ 데스크톱 개발 워크로드가 설치되어 있어야 합니다.
    -   ARM64 빌드를 위해서는 "C++ ARM64 build tools" 개별 구성 요소를 추가로 설치해야 합니다.
2.  **Ninja**: 빠른 빌드를 위한 빌드 시스템입니다.
    -   Visual Studio Installer에서 "C++ CMake tools for Windows" 구성 요소를 통해 설치할 수 있습니다.
    -   또는 Chocolatey (`choco install ninja`)나 Scoop (`scoop install ninja`)으로 직접 설치할 수 있습니다.

## 빌드 방법

빌드는 **Visual Studio 개발자 명령 프롬프트** 내에서 수행하는 것을 권장합니다. 이 환경은 컴파일러와 링커 경로를 자동으로 설정해주어 빌드 과정을 단순화합니다.

### 1. 개발자 명령 프롬프트 열기

시작 메뉴에서 빌드하려는 아키텍처에 맞는 프롬프트를 검색하여 실행합니다.

-   **x64 (amd64) 빌드용**: `x64 Native Tools Command Prompt for VS 2026`
-   **ARM64 빌드용**: `x64_arm64 Cross Tools Command Prompt for VS 2026`

### 2. 소스 코드로 이동

명령 프롬프트에서 이 프로젝트의 `cpp` 디렉터리로 이동합니다.

```sh
cd /path/to/your/project/cpp
```

### 3. CMake 실행 및 빌드

아래 명령어를 사용하여 빌드 디렉터리를 생성하고, 프로젝트를 설정한 뒤, 빌드를 실행합니다.

#### **x64 (amd64) 릴리스 빌드**

```sh
# 빌드 디렉터리 생성 및 이동
mkdir build && cd build

# CMake 설정 (Ninja 사용)
cmake .. -G "Ninja" -DCMAKE_BUILD_TYPE=Release

# 빌드 실행
cmake --build .
```

#### **ARM64 릴리스 빌드**

```sh
# 빌드 디렉터리 생성 및 이동
mkdir build-arm64 && cd build-arm64

# CMake 설정 (Ninja 사용)
cmake .. -G "Ninja" -DCMAKE_BUILD_TYPE=Release

# 빌드 실행
cmake --build .
```

빌드가 완료되면 빌드 디렉터리(`build` 또는 `build-arm64`) 안에 `nvme.exe` 실행 파일이 생성됩니다.

## 정적(Static) 빌드

C/C++ 런타임 라이브러리를 정적으로 링크하려면 CMake 설정 시 `USE_STATIC_RUNTIME` 옵션을 `ON`으로 지정합니다. 이는 MSVC에서 `/MT` 컴파일 옵션을 활성화합니다.

```sh
# x64 정적 릴리스 빌드 예시
cmake .. -G "Ninja" -DCMAKE_BUILD_TYPE=Release -DUSE_STATIC_RUNTIME=ON
cmake --build .
```

**참고**: 이 옵션은 C/C++ 런타임만 정적으로 링크하며, `SetupAPI.lib`와 같은 시스템 라이브러리는 여전히 동적으로 링크됩니다.

## (대안) Visual Studio 생성기 사용

Ninja 대신 Visual Studio 솔루션(`.sln`) 파일을 생성하여 빌드할 수도 있습니다.

```sh
# x64용 솔루션 생성
cmake .. -G "Visual Studio 18 2026" -A x64

# ARM64용 솔루션 생성
cmake .. -G "Visual Studio 18 2026" -A ARM64
```