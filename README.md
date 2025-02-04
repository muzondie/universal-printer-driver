# Universal Printer Driver  

A universal driver for printers on 64-bit Windows 10 and newer. Automatically detects connected printers, scanners, or related hardware and installs the correct drivers without manual input. Built in Rust for reliability and performance.  

## Download  
1. Go to the [Releases](https://github.com/muzondie/universal-printer-driver/releases) tab on GitHub.  
2. Download the latest `.zip` file.  
3. Unzip the file and run `UniversalPrinterDriver.exe`.  

## Usage  
1. **Run the application** after installation.  
2. The tool will **auto-scan** for connected devices.  
3. Drivers install silently, no user input needed.  
4. Use the GUI to adjust settings, check printer status, or force updates.  

## Features  
- Works with **all printer brands and models** (including HP, Canon, Epson, Brother, Lexmark, Xerox, Ricoh, Kyocera, Konica Minolta, Dell, Samsung, OKI Data, Sharp, Panasonic, Toshiba, Fujitsu, Zebra Technologies, Seiko Instruments, Datamax-O'Neil, Honeywell, Honeywell, Primera Technology, SATO, Dymo, Sindoh, RISO, TallyGenicom, Clover Imaging Group, Olivetti, Kodak, Formlabs and more).  
- Automatic detection of new or existing printers/scanners.  
- Self-updating driver database (no manual downloads).  
- Lightweight background operation (minimal CPU/RAM usage).  
- GUI for checking device status, troubleshooting, or manual overrides.  
- Supports 64-bit Windows 10, 11, and future versions.  
- No internet connection required after initial setup.  
- Secure, non-invasive installation (no bundled software).  

## Build from Source  
1. Install [Rust](https://www.rust-lang.org/tools/install).  
2. Clone the repository:  
   ```bash  
   git clone https://github.com/muzondie/universal-printer-driver.git  
   ```  
3. Build the project:  
   ```bash  
   cd universal-printer-driver  
   cargo build --release  
   ```  
4. Find the executable in `target/release/`.  

## Contributing  
This project does not accept issues, pull requests, or feature requests at this time due to limited maintenance capacity. You may fork the repository under the terms of the MIT license.  

## License  
MIT License. See [LICENSE](LICENSE) for details.