///! Utilities to get device information using udev - only supported on Linux
use std::error::Error;
use std::path::Path;
use udev as udevlib;

/// Get and assign `driver_ref` the driver for device at the `port_path`
pub fn get_driver(driver_ref: &mut Option<String>, port_path: &String) -> Result<(), Box<dyn Error>> {
    let path: String = format!("/sys/bus/usb/devices/{}", port_path);
    let device = udevlib::Device::from_syspath(&Path::new(&path))?;
    log::debug!("Got device driver {:?}", device.driver());
    *driver_ref = device.driver().map(|s| s.to_str().unwrap_or("").to_string());

    Ok(())
}