pub mod windows {
    use crate::utils::windows::get_wmi;

    #[derive(Debug)]
    pub struct ProcessorInfo {
        name: String,
        cores: u32,
        threads: u32,
        current_clock_speed: u32,
        current_voltage: Option<u16>,
        max_clock_speed: u32,
        processor_socket: String,
        load_percentage: u16,
        voltage_caps: Option<String>,
    }

    pub fn get_processor_info() -> Result<ProcessorInfo, windows::core::Error> {
        let processor_info = get_wmi("Win32_Processor", "Name, NumberOfCores, ThreadCount, CurrentClockSpeed, CurrentVoltage, MaxClockSpeed, SocketDesignation, LoadPercentage, VoltageCaps")?;

        Ok(
            ProcessorInfo {
                name: processor_info.get("Name").unwrap().to_string(),
                cores: processor_info.get("NumberOfCores").unwrap().to_string().parse().unwrap(),
                threads: processor_info.get("ThreadCount").unwrap().to_string().parse().unwrap(),
                current_clock_speed: processor_info.get("CurrentClockSpeed").unwrap().to_string().parse().unwrap(),
                current_voltage: {
                    let value:u16 = processor_info.get("CurrentVoltage").unwrap().parse().unwrap();
                    if (value & 0x0080) != 0 {
                        Some((value & 0x7f) * 10)
                    } else {
                        None
                    }
                },
                max_clock_speed: processor_info.get("MaxClockSpeed").unwrap().to_string().parse().unwrap(),
                processor_socket: processor_info.get("SocketDesignation").unwrap().to_string(),
                load_percentage: processor_info.get("LoadPercentage").unwrap().to_string().parse().unwrap(),
                voltage_caps: match processor_info.get("VoltageCaps").unwrap().as_str() {
                    "" => None,
                    value => Some(value.to_string()),
                },
            }
        )
    }
}