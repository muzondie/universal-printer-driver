use super::*;
use windows::Win32::Devices::Enumeration::{
    DEVICE_INTERFACE_CLASS_GUID, CM_Get_Device_ID_List_SizeA, CM_Get_Device_ID_ListA
};

pub async fn scan_printers() -> Result<Vec<String>> {
    let mut device_list = Vec::with_capacity(128);
    unsafe {
        let mut len = 0;
        CM_Get_Device_ID_List_SizeA(&mut len, core::ptr::null(), 0);
        let mut buffer = vec![0u8; len as usize];
        CM_Get_Device_ID_ListA(core::ptr::null(), buffer.as_mut_ptr(), len, 0);
        device_list = String::from_utf8_lossy(&buffer)
            .split('\0')
            .map(|s| s.to_string())
            .collect();
    }
    Ok(device_list.into_iter().filter(|s| !s.is_empty()).collect())
}