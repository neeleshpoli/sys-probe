mod processor;
pub mod windows {
    use super::processor;

    #[derive(Debug)]
    pub struct HardwareInfo {
        processor_info: processor::windows::ProcessorInfo,
    }

    pub fn get_hardware_info() -> Result<HardwareInfo, windows::core::Error> {
        Ok(
            HardwareInfo {
                processor_info: processor::windows::get_processor_info()?,
            }
        )
    }
}