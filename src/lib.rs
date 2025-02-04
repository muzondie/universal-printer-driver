#[macro_use]
extern crate log;

mod driver_db;
mod detection;
mod installer;
mod gui;
mod utils;

use anyhow::Result;
use windows::Win32::Foundation::HWND;

pub struct PrinterDriver {
    handle: HWND,
    runtime: tokio::runtime::Runtime,
}

impl PrinterDriver {
    pub fn initialize() -> Result<Self> {
        utils::init_logging();
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
            
        Ok(Self {
            handle: HWND(0),
            runtime,
        })
    }

    pub fn start_gui(&mut self) -> Result<()> {
        self.runtime.block_on(gui::run_ui())?;
        Ok(())
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct PrinterProfile {
    vendor_id: u16,
    product_id: u16,
    driver_path: String,
    checksum: [u8; 32],
}