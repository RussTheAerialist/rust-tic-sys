extern crate pretty_env_logger;
#[macro_use] extern crate log;
use tic_sys::get_device_list;

fn main() {
	pretty_env_logger::try_init().expect("Unable to initialize");

	unsafe {
		info!("Enumerating devices");
		let mut devices = get_device_list().expect("Unable to get list of devices");
		if devices.tics.len() == 0 {
			error!("No devices found");
			return;
		}

		devices.tics.iter().for_each(|e| {
			info!("{} S/N {}", e.name, e.serial_number);
		});

		info!("Opening first device");
		let device = devices.tics.remove(0).open().expect("Unable to open device");
		info!("Running firmware {}", device.firmware);
		info!("Leaving safe start");
		device.finish_startup().expect("Unable to finish startup");
	}
}