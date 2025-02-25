use byte_unit::Byte;
use sysinfo::Disks;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Device {
    name: String,
    size: u64,
}

impl Device {
    pub fn new(name: String, size: u64) -> Device {
        Device { name, size }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }
}

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {:?}\nSpace available: {}",
            self.name,
            Byte::from(self.size).get_appropriate_unit(byte_unit::UnitType::Decimal)
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Folder {
    path: String,
    size: u64,
}

impl Folder {
    pub fn new(path: String, size: u64) -> Folder {
        Folder { path, size }
    }

    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }
}

impl std::fmt::Display for Folder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Path: {:?}\nSize: {}",
            self.path,
            Byte::from(self.size).get_appropriate_unit(byte_unit::UnitType::Decimal)
        )
    }
}

/// Function to get the list of external devices, name and available space in bytes, filtering disks if they are removable. The list refresh every time you attach a new external device.
pub fn get_ext_devices() -> Vec<Device> {
    let disk_list = Disks::new_with_refreshed_list();

    disk_list
        .into_iter()
        .filter(|d| d.is_removable())
        .map(|d| Device::new(d.name().to_str().unwrap().to_string(), d.available_space()))
        .collect::<Vec<Device>>()
}
