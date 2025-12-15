#include <iostream>
#include <windows.h>
#include <setupapi.h>
#include <initguid.h>
#include <devpkey.h>
#include <string>
#include <vector>

#pragma comment(lib, "setupapi.lib")
#pragma comment(lib, "Propsys.lib")

void GetDeviceServiceExample()
{
    HDEVINFO hDevInfo;
    SP_DEVINFO_DATA deviceInfoData;

    // Request a device information set for all devices present on the system.
    hDevInfo = SetupDiGetClassDevsW(NULL, L"PCI", NULL, DIGCF_ALLCLASSES | DIGCF_PRESENT);
    if (hDevInfo == INVALID_HANDLE_VALUE)
    {
        std::cerr << "Failed to create device info list. Error: " << GetLastError() << std::endl;
        return;
    }

    deviceInfoData.cbSize = sizeof(SP_DEVINFO_DATA);

    for (DWORD i = 0; SetupDiEnumDeviceInfo(hDevInfo, i, &deviceInfoData); i++)
    {
        DEVPROPTYPE propType;
        DWORD requiredSize = 0;

        // First, query for the size of the property.
        if (!SetupDiGetDevicePropertyW(
                hDevInfo,
                &deviceInfoData,
                &DEVPKEY_Device_Service,
                &propType,
                NULL,
                0,
                &requiredSize,
                0) && GetLastError() != ERROR_INSUFFICIENT_BUFFER)
        {
            // This property doesn't exist for this device, or another error occurred.
            continue;
        }

        // The property exists, but is empty.
        if (requiredSize == 0)
        {
            continue;
        }

        // Use std::vector for automatic memory management.
        std::vector<wchar_t> serviceName(requiredSize / sizeof(wchar_t));

        // Now, retrieve the actual property data.
        if (SetupDiGetDevicePropertyW(
                hDevInfo,
                &deviceInfoData,
                &DEVPKEY_Device_Service,
                &propType,
                reinterpret_cast<PBYTE>(serviceName.data()),
                requiredSize,
                NULL,
                0))
        {
            std::wcout << L"Found Device with Service: " << serviceName.data() << std::endl;
        }
    }

    SetupDiDestroyDeviceInfoList(hDevInfo);
}

int main()
{
    GetDeviceServiceExample();
    return 0;
}
