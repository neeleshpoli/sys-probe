#[derive(Debug)]
pub struct BasicInfo {
    edition: String,
    version: String,
    friendly_version: String,
    install_date: i64,
    uptime: i32,
    username: String,
    domain: String,
    boot_mode: String,
    boot_state: String,
    model: String,
}

#[cfg(target_os = "windows")]
pub mod windows{
    use std::env;

    use chrono::Utc;
    use windows::Win32::System::Registry::{HKEY_LOCAL_MACHINE, RRF_RT_REG_SZ};

    use super::BasicInfo;

    use crate::utils::windows::{get_wmi, get_registry_key, timestamp_to_unix_time};

    pub fn get_basic_info() -> Result<BasicInfo, windows::core::Error> {
        let os_info = get_wmi("Win32_OperatingSystem", "Caption, Version, InstallDate, LastBootUpTime")?;
        let computer_info = get_wmi("Win32_ComputerSystem", "UserName, Domain, BootupState, Model, BootupState")?;
    
        Ok(
            BasicInfo {
                edition: os_info.get("Caption").expect("Windows Edition not found!").to_string(),
                version: os_info.get("Version").expect("Windows Version not found!").to_string(),
                friendly_version: get_registry_key(HKEY_LOCAL_MACHINE, r"SOFTWARE\Microsoft\Windows NT\CurrentVersion", "DisplayVersion", RRF_RT_REG_SZ)?,
                install_date: timestamp_to_unix_time(os_info.get("InstallDate").expect("Windows Install Date not found!")).unwrap(),
                uptime: (Utc::now().timestamp() - timestamp_to_unix_time(os_info.get("LastBootUpTime").unwrap()).unwrap()) as i32,
                username: computer_info.get("UserName").expect("Windows Username not found").to_string(),
                domain: computer_info.get("Domain").expect("Windows Domain not found").to_string(),
                boot_mode: env::var("firmware_type").unwrap(),
                boot_state: computer_info.get("BootupState").expect("Boot state not found").to_string(),
                model: computer_info.get("Model").expect("Model not found").to_string(),
            }
        )
    }
}