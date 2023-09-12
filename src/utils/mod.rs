mod error;
#[cfg(target_os = "windows")]
pub mod windows {
    use std::{
        collections::HashMap,
        ffi::c_void,
        alloc,
        str::FromStr,
        slice, ptr::{null_mut, self}
    };

    use chrono::TimeZone;
    use windows::{
        Win32::System::{Wmi::{IWbemLocator, WbemLocator, WBEM_FLAG_FORWARD_ONLY, WBEM_FLAG_RETURN_IMMEDIATELY, WBEM_INFINITE, IWbemServices, WBEM_FLAG_NONSYSTEM_ONLY},
        Com::{CoCreateInstance, CLSCTX_INPROC_SERVER}, Ole::{VarFormat, VARFORMAT_FIRST_DAY_SYSTEMDEFAULT, VARFORMAT_FIRST_WEEK_SYSTEMDEFAULT, SafeArrayGetLBound, SafeArrayGetUBound, SafeArrayAccessData},
        Registry::{HKEY, REG_ROUTINE_FLAGS, RegGetValueW}},
        core::{PCWSTR, HSTRING, BSTR}
    };

    pub use super::error::windows_error as Error;
    

    pub struct WMIWrapper {
        server: IWbemServices
    }

    impl WMIWrapper {
        pub fn new(location: &str) -> Error::SysProbeResult<Self> {
            // Make the connection to WMI
            Ok(
                WMIWrapper {
                    server: unsafe {
                        let locator: IWbemLocator = CoCreateInstance(
                            &WbemLocator,
                            None,
                            CLSCTX_INPROC_SERVER
                        )?;
                        locator.ConnectServer(
                            &BSTR::from(location),
                            None,
                            None,
                            None,
                            0,
                            None,
                            None
                        )?
                    }
                }
            )
        }

        pub fn get(&self, class_name: &str, fields: &str) -> Error::SysProbeResult<Vec<HashMap<String, String>>> {
            // Make the query
            let query = unsafe {
                self.server.ExecQuery(
                    &BSTR::from("WQL"),
                    &BSTR::from(format!("SELECT {fields} FROM {class_name}")),
                    WBEM_FLAG_FORWARD_ONLY | WBEM_FLAG_RETURN_IMMEDIATELY,
                    None
                )?
            };

            // Store all the data from the query
            let mut instance_data: Vec<HashMap<String, String>> = Vec::new();

            // Loop through all the instances of the query
            loop {
                // Store the data
                let mut data: HashMap<String, String> = HashMap::new();
                // Store the result of a single instance
                let mut objs = [None; 1];
                let mut return_value = 0;

                // Get the next instance
                let result = unsafe {
                    query.Next(
                        WBEM_INFINITE,
                        &mut objs,
                        &mut return_value
                    ).ok()?
                };

                // Check if there are no more instances
                if return_value == 0 {
                    break;
                }

                // Get the field names
                let field_names = unsafe { 
                    let safe_array = objs.get(0).unwrap().as_ref().unwrap().GetNames(None, WBEM_FLAG_NONSYSTEM_ONLY, ptr::null())?;

                    let mut p_data = null_mut();
                    let lower_bound = SafeArrayGetLBound(safe_array, 1)?;
                    let upper_bound = SafeArrayGetUBound(safe_array, 1)?;

                    SafeArrayAccessData(safe_array, &mut p_data)?;
                    let data = p_data as *mut BSTR;
                    let slice = slice::from_raw_parts(data, (upper_bound + 1) as usize);
                    let data_slice = &slice[(lower_bound as usize)..];

                    let mut field_names: Vec<String> = Vec::new();
                    
                    for item in data_slice.iter() {
                        field_names.push(item.to_string());
                    }

                    field_names
                };

                // Get the data
                for field_name in field_names {
                    let mut value = Default::default();
                    unsafe {
                        let _ = objs.get(0).unwrap().as_ref().unwrap().Get(
                            PCWSTR(HSTRING::from(&field_name).as_ptr()),
                            0,
                            &mut value,
                            None,
                            None
                        );
                        data.insert(
                            field_name,
                            VarFormat(&value, None, VARFORMAT_FIRST_DAY_SYSTEMDEFAULT, VARFORMAT_FIRST_WEEK_SYSTEMDEFAULT, 0)?.to_string()
                        );
                    }
                }

                // Store the retrievd data
                instance_data.push(data);
            };

            // Return the data
            Ok(instance_data)
        }
    }
    
    pub fn timestamp_to_unix_time(timestamp: &str) -> Error::SysProbeResult<i64> {
        // Parse the components of the timestamp
        if let (Ok(year), Ok(month), Ok(day), Ok(hour), Ok(min), Ok(sec), Ok(fraction), Ok(offset)) = (
            i32::from_str(&timestamp[0..4]),
            u32::from_str(&timestamp[4..6]),
            u32::from_str(&timestamp[6..8]),
            u32::from_str(&timestamp[8..10]),
            u32::from_str(&timestamp[10..12]),
            u32::from_str(&timestamp[12..14]),
            u32::from_str(&timestamp[15..21]),
            i32::from_str(&timestamp[21..])
        ) {
            // Create a chrono::FixedOffset from the offset in minutes
            let offset = chrono::FixedOffset::east_opt(offset * 60).unwrap();
    
            // Create a chrono::DateTime object
            let datetime = offset.with_ymd_and_hms(year, month, day, hour, min, sec).unwrap();
    
            // Convert the DateTime to Unix timestamp
            Ok(datetime.timestamp())
        } else {
            Err(Error::SysProbeError::DateTimeParsingError()) // Parsing failed
        }
    }
    
    pub fn get_registry_key<'a>(hkey: HKEY, subkey: &'a str, key: &'a str, flags: REG_ROUTINE_FLAGS) -> Error::SysProbeResult<String> {
        unsafe {
            /* 
                Make the variables
                data: Stores the data
                data_length: Stores the length of data
                subkey/key_hstring: Stores the subkey/key as a HSTRING
            */
            let data: *mut c_void;
            let mut data_length: u32 = 0;
            let subkey_hstring = HSTRING::from(subkey);
            let key_hstring = HSTRING::from(key);
    
            // Get the data length to allocate memory
            RegGetValueW(hkey, PCWSTR(subkey_hstring.as_ptr()), PCWSTR(key_hstring.as_ptr()), flags, None, None, Some(&mut data_length))?;
    
            // Allocate the memory
            let layout = alloc::Layout::from_size_align(data_length as usize, 1)?;
            data = alloc::alloc(layout) as *mut c_void;
    
            // Now get the data
            RegGetValueW(hkey, PCWSTR(subkey_hstring.as_ptr()), PCWSTR(key_hstring.as_ptr()), flags, None, Some(data), Some(&mut data_length))?;
    
            // Convert the data and get the null terminator
            let data_slice = slice::from_raw_parts(data as *const u16, 12 as usize);
            let null_terminator_pos = data_slice.iter().position(|&x| x == 0);
    
            // Deallocate the memory that we allocated earilier
            alloc::dealloc(data as *mut u8, layout);
    
            // Match the null terminator in case it doesn't exist
            Ok(String::from_utf16_lossy(match null_terminator_pos {
                Some(pos) => &data_slice[..pos],
                None => &data_slice,
            }))
        }
    }
}