mod processor;
#[cfg(target_os = "windows")]
pub mod windows {
    use crate::utils::windows::Error::SysProbeResult;

    use super::processor;

    #[derive(Debug)]
    pub struct HardwareInfo {
        processor_info: processor::windows::ProcessorInfo,
    }

    pub fn get_hardware_info() -> SysProbeResult<HardwareInfo> {
        Ok(
            HardwareInfo {
                processor_info: processor::windows::get_processor_info()?,
            }
        )
    }
}