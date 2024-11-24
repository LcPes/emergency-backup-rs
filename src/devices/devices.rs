use sysinfo::Disks;

/// Function to get the list of external devices, filtering disks if they are removable. The list refresh every time you attach a new external device.
pub fn get_ext_devices() -> Vec<String> {
    let disk_list = Disks::new_with_refreshed_list();

    disk_list
        .into_iter()
        .filter(|d| d.is_removable())
        .map(|d| d.name().to_str().unwrap().to_string())
        .collect::<Vec<String>>()
}
