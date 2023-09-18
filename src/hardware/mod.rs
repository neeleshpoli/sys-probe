mod processor;
mod gpu;
#[cfg(target_os = "windows")]
pub mod windows {
    use crate::utils::windows::Error::SysProbeResult;

    use super::{processor, gpu::windows::{GraphicsInfo, get_graphics_info}};

    #[derive(Debug)]
    pub struct HardwareInfo {
        processor_info: processor::windows::ProcessorInfo,
        graphics_info: Vec<GraphicsInfo>,
    }

    pub fn get_hardware_info() -> SysProbeResult<HardwareInfo> {
        Ok(
            HardwareInfo {
                processor_info: processor::windows::get_processor_info()?,
                graphics_info: get_graphics_info()?,
            }
        )
    }
}