use universal_printer_driver::*;

fn main() -> anyhow::Result<()> {
    let mut driver = PrinterDriver::initialize()?;
    driver.start_gui()?;
    Ok(())
}