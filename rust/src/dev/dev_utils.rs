use super::nvme_device::InboxDriver;
use crate::dev::disk::{get_physical_drv_number_from_logical_drv, ioctl, open};
use sscanf;
use std::sync::Mutex;
use std::{
    ffi::c_void,
    fmt,
    mem::{size_of_val, zeroed},
    ptr::null_mut,
};
use windows_sys::{
    core::*,
    Win32::{
        Devices::{DeviceAndDriverInstallation::*, Properties::*},
        Foundation::*,
        Storage::FileSystem::GetLogicalDriveStringsA,
        System::Ioctl::*,
    },
};

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct PciBdf {
    segment: i32,
    bus: i32,
    device: i32,
    function: i32,
}

impl PciBdf {
    pub fn new(segment: i32, bus: i32, device: i32, function: i32) -> Self {
        Self {
            segment,
            bus,
            device,
            function,
        }
    }

    pub fn parse(location_info: &str) -> Option<Self> {
        sscanf::sscanf!(location_info, "PCI bus {i32}, device {i32}, function {i32}")
            .ok()
            .map(|(bus, device, function)| Self {
                segment: 0,
                bus,
                device,
                function,
            })
    }
}

impl fmt::Debug for PciBdf {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            fmt,
            "PCI bus {:02x}, device {:02x}, function {:02x} (Segment: {})",
            self.bus, self.device, self.function, self.segment
        )
    }
}

impl fmt::Display for PciBdf {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{:04X}:{:02X}:{:02X}:{:02X}",
            self.segment, self.bus, self.device, self.function
        )
    }
}

#[derive(Debug, Clone, Copy)]
struct DevInstance {
    devinst: u32,
    status: u32,
    problem: u32,
}

impl DevInstance {
    pub fn new(devinst: u32) -> Option<Self> {
        let mut status: u32 = 0;
        let mut problem: u32 = 0;
        if unsafe { CM_Get_DevNode_Status(&mut status, &mut problem, devinst, 0) } == CR_SUCCESS {
            Some(Self {
                devinst,
                status,
                problem,
            })
        } else {
            None
        }
    }

    pub fn get_device_property(&self, property_key: *const DEVPROPKEY) -> Option<Vec<u16>> {
        let mut buffer: Vec<u16> = vec![0; 260];
        let mut buffer_len: u32 = buffer.len() as u32;
        let mut property_type: u32 = 0; // assuming propertytype is a u32 (adjust if needed)

        if unsafe {
            CM_Get_DevNode_PropertyW(
                self.devinst,
                property_key,
                &mut property_type,
                buffer.as_mut_ptr() as *mut u8,
                &mut buffer_len,
                0,
            )
        } != CR_SUCCESS
        {
            return None;
        }

        let trimed: Vec<u16> = buffer.into_iter().take_while(|&c| c != 0).collect();
        Some(trimed)
    }

    pub fn service(&self) -> Option<String> {
        if let Some(trimed) = self.get_device_property(&DEVPKEY_Device_Service) {
            return Some(String::from_utf16_lossy(&trimed));
        }
        None
    }

    pub fn location_info(&self) -> Option<String> {
        if let Some(trimed) = self.get_device_property(&DEVPKEY_Device_LocationInfo) {
            return Some(String::from_utf16_lossy(&trimed));
        }
        None
    }

    pub fn pcibdf(&self) -> PciBdf {
        let mut bdf = PciBdf::default();
        if let Some(trimed) = self.get_device_property(&DEVPKEY_Device_LocationInfo) {
            bdf = PciBdf::parse(&String::from_utf16_lossy(&trimed)).unwrap_or_default();
        }
        bdf
    }

    pub fn instance_id(&self) -> Option<Vec<u16>> {
        self.get_device_property(&DEVPKEY_Device_InstanceId)
    }

    pub fn enable(&self) -> CONFIGRET {
        let cr = unsafe { CM_Enable_DevNode(self.devinst, 0) };
        if cr != CR_SUCCESS {
            println!("enable {} failed ({})", self.devinst, cr);
        }
        cr
    }

    pub fn disable(&self) -> CONFIGRET {
        let cr =
            unsafe { CM_Disable_DevNode(self.devinst, CM_DISABLE_HARDWARE | CM_DISABLE_UI_NOT_OK) };
        if cr != CR_SUCCESS {
            println!("disable {} failed ({})", self.devinst, cr);
        }
        cr
    }

    pub fn remove(&self) -> CONFIGRET {
        let cr = unsafe {
            CM_Query_And_Remove_SubTreeA(
                self.devinst,
                null_mut(),
                null_mut(),
                0,
                CM_REMOVE_NO_RESTART,
            )
        };
        if cr != CR_SUCCESS {
            println!("remove {} failed ({})", self.devinst, cr);
        }
        cr
    }

    pub fn restart(&self) -> CONFIGRET {
        let cr = unsafe { CM_Setup_DevNode(self.devinst, CM_SETUP_DEVNODE_READY) };
        if cr != CR_SUCCESS {
            println!("restart {} failed ({})", self.devinst, cr);
        }
        cr
    }

    pub fn refresh(&self) -> CONFIGRET {
        let mut devinst: u32 = 0;
        let cr = unsafe { CM_Locate_DevNodeA(&mut devinst, null_mut(), CM_LOCATE_DEVNODE_NORMAL) };
        if cr == CR_SUCCESS {
            unsafe { CM_Reenumerate_DevNode(devinst, 0) }
        } else {
            println!("refresh {} failed ({})", self.devinst, cr);
            cr
        }
    }

    pub fn parent(&self) -> Option<Self> {
        let mut parent: u32 = 0;
        if unsafe { CM_Get_Parent(&mut parent, self.devinst, 0) } == CR_SUCCESS {
            return DevInstance::new(parent);
        }
        None
    }

    pub fn handle(&self) -> u32 {
        self.devinst
    }
}

impl fmt::Display for DevInstance {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:>2}", self.devinst)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LogicalDrive;
static SHARED_DATA: Mutex<Vec<(i32, String)>> = Mutex::new(Vec::new());

impl LogicalDrive {
    fn enumerate() {
        let mut data = SHARED_DATA.lock().unwrap();
        if data.len() <= 0 {
            let mut buffer = vec![0u8; 1024];
            if unsafe { GetLogicalDriveStringsA(buffer.len() as u32, buffer.as_mut_ptr()) } > 0 {
                let drive_list: Vec<String> = buffer
                    .split(|&e| e == 0x0)
                    .filter(|v| !v.is_empty())
                    .map(|v| String::from_utf8_lossy(v).into_owned()) // Convert &[u8] to String
                    .collect();
                for drive in &drive_list {
                    let mut drive_mut = drive.clone(); // Make a mutable copy
                    if drive_mut.ends_with('\\') {
                        drive_mut.pop();
                    }
                    let _disk_no = get_physical_drv_number_from_logical_drv(drive_mut.to_string());
                    data.push((_disk_no, drive_mut.to_string()));
                }
            }
        }
    }

    fn get_drives(number: i32) -> Vec<String> {
        Self::enumerate();
        let data = SHARED_DATA.lock().unwrap();
        data.iter()
            .filter(|(n, _)| number >= 0 && *n == number)
            .map(|(_, text)| text.clone())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct PhysicalDisk {
    devinst: DevInstance,
    interface_path: String,
    device_path: String,
    disk_number: i32,
    nsid: i32,
    drives: Vec<String>,
    pub dev_type: u8,
    driver: Option<InboxDriver>,
}

impl PhysicalDisk {
    pub fn new(devinst: u32) -> Option<Self> {
        DevInstance::new(devinst).map(|devinst| Self {
            devinst,
            interface_path: String::new(),
            device_path: String::new(),
            disk_number: -1,
            nsid: -1,
            drives: vec![],
            dev_type: 2,
            driver: None,
        })
    }

    pub fn inspect(&mut self) -> &mut Self {
        self.get_interface_path()
            .get_device_number()
            .get_nsid()
            .enum_child_volumes()
    }

    pub fn path(&self) -> String {
        self.device_path.clone()
    }

    pub fn disable(&self) -> CONFIGRET {
        match self.drives.iter().find(|drive| drive.as_str() == "C:") {
            Some(_) => CR_NOT_DISABLEABLE,
            None => self.devinst.disable(),
        }
    }

    fn get_nsid(&mut self) -> &mut Self {
        if let Some(ref device_id) = self.devinst.instance_id() {
            self.nsid = String::from_utf16_lossy(device_id)
                .split('&')
                .last()
                .unwrap()
                .parse::<i32>()
                .unwrap()
                + 1;
        }
        self
    }

    fn get_interface_path(&mut self) -> &mut Self {
        unsafe {
            if let Some(ref device_id) = self.devinst.instance_id() {
                let guid: *const GUID = &GUID_DEVINTERFACE_DISK;
                let mut iface_list_size: u32 = 0;
                let ret = CM_Get_Device_Interface_List_SizeW(
                    &mut iface_list_size,
                    guid,
                    device_id.as_ptr(),
                    CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
                );
                if ret != CR_SUCCESS {
                    println!(
                        "get size of {:?} failed: size {} ret {:?}",
                        device_id, iface_list_size, ret
                    );
                    return self;
                }

                let mut iface_list: Vec<u16> = vec![0; iface_list_size as usize];
                if CM_Get_Device_Interface_ListW(
                    guid,
                    device_id.as_ptr(),
                    iface_list.as_mut_ptr(),
                    iface_list_size,
                    CM_GET_DEVICE_INTERFACE_LIST_PRESENT,
                ) != CR_SUCCESS
                {
                    return self;
                }

                let interface: Vec<u16> = iface_list.into_iter().take_while(|&c| c != 0).collect();
                self.interface_path = String::from_utf16_lossy(&interface).to_string();
            }
        }
        self
    }

    fn get_device_number(&mut self) -> &mut Self {
        unsafe {
            let handle = open(&self.interface_path, 'r');
            if handle != INVALID_HANDLE_VALUE {
                let mut sdn: STORAGE_DEVICE_NUMBER = zeroed();
                if let Ok(_ret) = ioctl(
                    handle,
                    IOCTL_STORAGE_GET_DEVICE_NUMBER,
                    Some((null_mut(), 0)),
                    Some((&mut sdn as *mut _ as *mut c_void, size_of_val(&sdn))),
                ) {
                    CloseHandle(handle);
                    self.disk_number = sdn.DeviceNumber as i32;
                    self.device_path = format!("\\\\.\\PhysicalDrive{0}", self.disk_number);
                }
            }
        }
        self
    }

    fn enum_child_volumes(&mut self) -> &mut Self {
        self.drives = LogicalDrive::get_drives(self.disk_number);
        self
    }

    pub fn open(&mut self) -> &mut Self {
        self.driver = match InboxDriver::open(&self.path()) {
            Ok(driver) => Some(driver),
            Err(e) => {
                eprintln!("Failed to open driver: {}", e);
                None
            }
        };
        self
    }

    pub fn get_driver(&self) -> &InboxDriver {
        self.driver.as_ref().unwrap()
    }
}

impl fmt::Display for PhysicalDisk {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            " L PhyDisk {:<2} ({}): nsid {} - {:?}",
            self.disk_number, self.devinst, self.nsid, self.drives
        )
    }
}

#[derive(Debug, Clone)]
pub struct NvmeController {
    devinst: DevInstance,
    interface_path: String,
    bdf: PciBdf,
    disks: Vec<PhysicalDisk>,
    pub dev_type: u8,
    driver: Option<InboxDriver>,
}

impl NvmeController {
    pub fn new(devinst: u32, interface_path: String) -> Option<Self> {
        if let Some(devinst) = DevInstance::new(devinst) {
            match devinst.service() {
                Some(service) if service == "stornvme" => {
                    return Some(Self {
                        devinst,
                        interface_path,
                        bdf: Default::default(),
                        disks: vec![],
                        dev_type: 1,
                        driver: None,
                    })
                }
                _ => return None,
            }
        }
        None
    }

    pub fn inspect(&mut self) -> &mut Self {
        self.bdf = self.devinst.pcibdf();
        self
    }

    pub fn enum_child_disks(&mut self) -> &mut Self {
        unsafe {
            let mut child = 0;
            let mut result = CM_Get_Child(&mut child, self.devinst.handle(), 0);
            while result == CR_SUCCESS {
                if let Some(mut disk) = PhysicalDisk::new(child) {
                    disk.inspect();
                    self.disks.push(disk);
                }
                result = CM_Get_Sibling(&mut child, child, 0);
            }
        }
        self
    }

    pub fn by_num(&mut self, driveno: i32) -> Option<&mut PhysicalDisk> {
        self.disks
            .iter_mut()
            .find(|disk| disk.disk_number == driveno)
    }

    pub fn path(&self) -> String {
        self.interface_path.clone()
    }

    pub fn enable(&self) -> CONFIGRET {
        let mut cr = self.devinst.enable();
        if cr == CR_SUCCESS {
            for disk in &self.disks {
                cr = disk.devinst.enable();
                std::thread::sleep(std::time::Duration::from_millis(500));
            }
        }
        cr
    }

    pub fn disable(&self) -> CONFIGRET {
        let mut cr = CR_SUCCESS;
        for disk in &self.disks {
            cr |= disk.disable();
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        if cr == CR_SUCCESS {
            self.devinst.disable()
        } else {
            cr
        }
    }

    pub fn remove(&self) -> CONFIGRET {
        for disk in &self.disks {
            disk.devinst.remove();
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        self.devinst.remove()
    }

    pub fn restart(&self) -> CONFIGRET {
        self.remove();
        let cr = self.devinst.restart();
        std::thread::sleep(std::time::Duration::from_millis(500));
        cr
    }

    pub fn rescan(&self) -> CONFIGRET {
        let mut cr = CR_SUCCESS;
        self.remove();
        if let Some(parent) = self.devinst.parent() {
            parent.disable();
            std::thread::sleep(std::time::Duration::from_millis(1000));
            cr = parent.enable();
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        cr
    }

    pub fn refresh(&self) -> CONFIGRET {
        self.devinst.refresh()
    }

    pub fn open(&mut self) -> &mut Self {
        self.driver = match InboxDriver::open(&self.path()) {
            Ok(driver) => Some(driver),
            Err(e) => {
                eprintln!("Failed to open driver: {}", e);
                None
            }
        };
        self
    }

    pub fn get_driver(&self) -> &InboxDriver {
        self.driver.as_ref().unwrap()
    }
}

impl fmt::Display for NvmeController {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "({}) {}/{}\n",
            self.devinst,
            self.devinst.parent().unwrap().pcibdf(),
            self.bdf
        )?;
        for disk in &self.disks {
            writeln!(fmt, "{}", disk)?;
        }
        Ok(())
    }
}

pub struct NvmeControllerList {
    controllers: Vec<NvmeController>,
}

impl NvmeControllerList {
    pub fn new() -> Self {
        Self {
            controllers: vec![],
        }
    }

    pub fn enumerate(&mut self) -> &mut Self {
        unsafe {
            let guid: *const GUID = &GUID_DEVINTERFACE_STORAGEPORT;
            let mut iface_list_size: u32 = 0;
            if CM_Get_Device_Interface_List_SizeW(
                &mut iface_list_size,
                guid,
                null_mut(),
                CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES,
            ) != CR_SUCCESS
            {
                return self;
            }

            let mut iface_list: Vec<u16> = vec![0; iface_list_size as usize];
            if CM_Get_Device_Interface_ListW(
                guid,
                null_mut(),
                iface_list.as_mut_ptr() as *mut u16,
                iface_list_size,
                CM_GET_DEVICE_INTERFACE_LIST_ALL_DEVICES,
            ) != CR_SUCCESS
            {
                return self;
            }

            let interfaces: Vec<_> = iface_list
                .split(|&e| e == 0x0)
                .filter(|v| !v.is_empty())
                .collect();

            let mut devinst = 0;
            let mut propertytype: DEVPROPTYPE = 0;
            for interface in interfaces {
                let iface_list_str = String::from_utf16_lossy(interface);
                // println!("{} {:?}", devinst, iface_list_str);
                let mut current_device: Vec<u16> = vec![0; 256];
                let mut device_id_size: u32 = current_device.len() as u32;
                let ret = CM_Get_Device_Interface_PropertyW(
                    interface.as_ptr(),
                    &DEVPKEY_Device_InstanceId,
                    &mut propertytype,
                    current_device.as_mut_ptr() as *mut u8,
                    &mut device_id_size,
                    0,
                );
                if ret != CR_SUCCESS || propertytype != DEVPROP_TYPE_STRING {
                    continue;
                }

                if CM_Locate_DevNodeW(
                    &mut devinst,
                    current_device.as_ptr(),
                    CM_LOCATE_DEVNODE_NORMAL,
                ) != CR_SUCCESS
                {
                    continue;
                }

                if let Some(mut controller) = NvmeController::new(devinst, iface_list_str) {
                    controller.inspect().enum_child_disks();
                    self.controllers.push(controller);
                }
            }
            self.controllers.sort_by(|a, b| a.bdf.cmp(&b.bdf));
        }
        self
    }

    pub fn by_num(&mut self, driveno: i32) -> Option<&mut PhysicalDisk> {
        self.controllers
            .iter_mut()
            .find_map(|controller| controller.by_num(driveno))
    }

    pub fn by_bus(&mut self, bus: i32) -> Option<&mut NvmeController> {
        self.controllers
            .iter_mut()
            .find(|controller| controller.bdf == PciBdf::new(0, bus, 0, 0))
    }
}

impl fmt::Display for NvmeControllerList {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        let mut index = 0;
        for controller in &self.controllers {
            write!(fmt, "NVME {}: {}", index, controller)?;
            index += 1;
        }
        Ok(())
    }
}
