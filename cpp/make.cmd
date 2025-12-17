@echo off
setlocal

if "%1"=="clean" goto clean

if not exist build mkdir build

rem Compiler options from CMakeLists.txt and original make.cmd
set CMD_OPTS=/EHsc /std:c++20 /W2 /WX /permissive- /Zi /O2 /MT

rem Source files for the executable
set SOURCES=main.cpp dev_utils.cpp disk.cpp lib.cpp nvme_device.cpp nvme_print.cpp scsi.cpp

rem Required libraries
set LIBS=Cfgmgr32.lib SetupAPI.lib

rem Output executable name
set EXECUTABLE_NAME=nvme.exe

echo cl.exe %CMD_OPTS% /Fo:build\ /Fd:build\%EXECUTABLE_NAME%.pdb /Fe:build\%EXECUTABLE_NAME% %SOURCES% /link %LIBS%
cl.exe %CMD_OPTS% /Fo:build\ /Fd:build\%EXECUTABLE_NAME%.pdb /Fe:build\%EXECUTABLE_NAME% %SOURCES% /link %LIBS%

goto :EOF

:clean
if exist build rd /s /q build
echo Cleaned build directory.
goto :EOF

endlocal
