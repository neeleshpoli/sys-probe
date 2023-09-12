mod utils;
mod basic_info;
mod hardware;

#[cfg(target_os = "windows")]
use windows::Win32::System::Com::{CoInitializeEx, CoInitializeSecurity, COINIT_MULTITHREADED, RPC_C_AUTHN_LEVEL_DEFAULT, RPC_C_IMP_LEVEL_IMPERSONATE, EOAC_NONE};
#[cfg(target_os = "windows")]
use crate::{
    utils::windows::Error::SysProbeResult,
    basic_info::windows::get_basic_info,
    hardware::windows::get_hardware_info
};

#[cfg(target_os = "windows")]
fn main() -> SysProbeResult<()> {
    // Initialize secutriy for WMI
    unsafe {
        CoInitializeEx(None, COINIT_MULTITHREADED)?;
        CoInitializeSecurity(None, -1, None, None, RPC_C_AUTHN_LEVEL_DEFAULT, RPC_C_IMP_LEVEL_IMPERSONATE, None, EOAC_NONE, None)?;
    }

    // Get basic info and print it
    println!("{:#?}", get_basic_info());
    println!("{:#?}", get_hardware_info());

    Ok(())
}