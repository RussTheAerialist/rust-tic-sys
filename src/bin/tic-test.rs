use tic_sys::get_device_list;

fn main() {
	unsafe {
		let devices = get_device_list().expect("Unable to get list of devices");
		devices.tics.iter().for_each(|e| {
			println!("{:?}", e);
		});
	}
}