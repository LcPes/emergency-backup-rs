use sysinfo::Disks;

/// Function to get the list of external devices, name and available space in bytes, filtering disks if they are removable. The list refresh every time you attach a new external device.
pub fn get_ext_devices() -> Vec<(String, u64)> {
    let disk_list = Disks::new_with_refreshed_list();

    disk_list
        .into_iter()
        .filter(|d| d.is_removable())
        .map(|d| (d.name().to_str().unwrap().to_string(), d.available_space()))
        .collect::<Vec<(String, u64)>>()
}
