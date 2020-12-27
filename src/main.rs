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

		device.engage().expect("Unable to engage the motors");

		let variables = device.variables().expect("Unable to read variables from device");
		let position = variables.position();
		info!("Current Position: {}", position);

		let new_position = if position < 0 { 200 } else { -200 };
		device.set_position(new_position).expect("Unable to set position");

		info!("Leaving safe start");
		device.finish_startup().expect("Unable to finish startup");
		std::thread::sleep(std::time::Duration::from_secs(2));
		device.idle().expect("Unable to idle the motors");
	}
}