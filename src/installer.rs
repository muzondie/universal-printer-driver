use super::*;
use windows::Win32::System::Com::{CoInitializeEx, COINIT_MULTITHREADED};

pub async fn install_driver(profile: &PrinterProfile) -> Result<()> {
    unsafe { CoInitializeEx(core::ptr::null_mut(), COINIT_MULTITHREADED)?; }
    
    tokio::task::spawn_blocking(move || {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }).await?;
    
    Ok(())
}