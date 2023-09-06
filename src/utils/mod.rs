#[cfg(target_os = "windows")]
pub mod windows {
    use std::{
        collections::HashMap,
        ffi::c_void,
        alloc,
        str::FromStr,
        slice
    };

    use chrono::TimeZone;
    use windows::{
        Win32::System::{Wmi::{IWbemLocator, WbemLocator, WBEM_FLAG_FORWARD_ONLY, WBEM_FLAG_RETURN_IMMEDIATELY, WBEM_INFINITE},
        Com::{CoCreateInstance, CLSCTX_INPROC_SERVER}, Ole::{VarFormat, VARFORMAT_FIRST_DAY_SYSTEMDEFAULT, VARFORMAT_FIRST_WEEK_SYSTEMDEFAULT},
        Variant::VariantClear,
        Registry::{HKEY, REG_ROUTINE_FLAGS, RegGetValueW}},
        core::{PCWSTR, HSTRING, BSTR}
    };

    pub fn get_wmi(class_name: &str, fields: &str) -> Result<HashMap<String, String>, windows::core::Error> {
        // Store the data and get all the fields
        let mut data: HashMap<String, String> = HashMap::new();
        let remove_spaces = fields.replace(' ', "");
        let class_fields: Vec<&str> = remove_spaces.split(',').collect();
    
        unsafe {
            // Connect to WMI
            let locator: IWbemLocator = CoCreateInstance(&WbemLocator, None, CLSCTX_INPROC_SERVER)?;
            let server = locator.ConnectServer(&BSTR::from("root\\cimv2"), None, None, None, 0, None, None)?;
    
            // Make the query in WQL
            let query = server.ExecQuery(&BSTR::from("WQL"), &BSTR::from(format!("SELECT {} FROM {}", fields, class_name)), WBEM_FLAG_FORWARD_ONLY | WBEM_FLAG_RETURN_IMMEDIATELY, None,)?;
    
            // Store the data and continue through the query
            let mut row = [None; 1];
            let mut returned = 0;
            query.Next(WBEM_INFINITE, &mut row, &mut returned).ok()?;
    
            if let Some(row) = &row[0] {
                for &field in &class_fields {
                    let mut value = Default::default();
                    let pc_wstr = PCWSTR(HSTRING::from(field).as_ptr());
    
                    // Get the data
                    row.Get(pc_wstr, 0, &mut value, None, None)?;
                    data.insert(field.to_string(), VarFormat(&value, None, VARFORMAT_FIRST_DAY_SYSTEMDEFAULT, VARFORMAT_FIRST_WEEK_SYSTEMDEFAULT, 0)?.to_string());
    
                    VariantClear(&mut value)?;
                }
            }
        };
    
        Ok(data)
    }
    
    pub fn timestamp_to_unix_time(timestamp: &str) -> Option<i64> {
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
            let offset = chrono::FixedOffset::east_opt(offset * 60)?;
    
            // Create a chrono::DateTime object
            let datetime = offset.with_ymd_and_hms(year, month, day, hour, min, sec).unwrap();
    
            // Convert the DateTime to Unix timestamp
            Some(datetime.timestamp())
        } else {
            None // Parsing failed
        }
    }
    
    pub fn get_registry_key<'a>(hkey: HKEY, subkey: &'a str, key: &'a str, flags: REG_ROUTINE_FLAGS) -> Result<String, windows::core::Error> {
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
            let layout = alloc::Layout::from_size_align(data_length as usize, 1).unwrap();
            data = alloc::alloc(layout) as *mut c_void;
    
            // Now get the data
            RegGetValueW(hkey, PCWSTR(subkey_hstring.as_ptr()), PCWSTR(key_hstring.as_ptr()), flags, None, Some(data), Some(&mut data_length))?;
    
            // Convert the data and get the null terminator
            let data_slice = slice::from_raw_parts(data as *const u16, 12 as usize);
            let null_terminator_pos = data_slice.iter().position(|&x| x == 0);
    
            // Deallocate the memory the we allocated earilier
            alloc::dealloc(data as *mut u8, layout);
    
            // Match the null terminator in case it doesn't exist
            Ok(String::from_utf16_lossy(match null_terminator_pos {
                Some(pos) => &data_slice[..pos],
                None => &data_slice,
            }))
        }
    }
}