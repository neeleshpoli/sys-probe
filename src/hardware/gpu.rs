pub mod windows {
    use crate::utils::windows::{Error::SysProbeResult, WMIWrapper};

    #[derive(Debug)]
    pub struct GraphicsInfo {
        name: String,
        memory: String,
        // May not return anything since the GPU may not be outputting anything
        horizontal_resolution: Option<u32>,
        vertical_resolution: Option<u32>,
        refresh_rate: Option<u32>,
        bits_per_pixel: Option<u32>,
    }

    pub fn get_graphics_info() -> SysProbeResult<Vec<GraphicsInfo>> {
        let wmi_getter = WMIWrapper::new("root\\CIMV2")?;
        let graphics_info = wmi_getter.get("Win32_VideoController", "Name, AdapterRAM, CurrentHorizontalResolution, CurrentVerticalResolution, CurrentRefreshRate, CurrentBitsPerPixel")?;

        let mut parsed_graphics_info: Vec<GraphicsInfo> = Vec::new();

        for info in graphics_info {
            parsed_graphics_info.push(
                GraphicsInfo {
                    name: {
                        println!("{}", info.get("Name").unwrap().to_string());
                        info.get("Name").unwrap().to_string()
                    },
                    memory: info.get("AdapterRAM").unwrap().to_string(),
                    horizontal_resolution: match info.get("CurrentHorizontalResolution").unwrap().as_str() {
                        "" => None,
                        _ => Some(info.get("CurrentHorizontalResolution").unwrap().to_string().parse().unwrap()),
                    },
                    vertical_resolution: match info.get("CurrentVerticalResolution").unwrap().as_str() {
                        "" => None,
                        _ => Some(info.get("CurrentVerticalResolution").unwrap().to_string().parse().unwrap()),
                    },
                    refresh_rate: match info.get("CurrentRefreshRate").unwrap().as_str() {
                        "" => None,
                        _ => Some(info.get("CurrentRefreshRate").unwrap().to_string().parse().unwrap()),
                    },
                    bits_per_pixel: match info.get("CurrentBitsPerPixel").unwrap().as_str() {
                        "" => None,
                        _ => Some(info.get("CurrentBitsPerPixel").unwrap().to_string().parse().unwrap()),
                    },
                }
            )
        }

        Ok(parsed_graphics_info)
    }
}