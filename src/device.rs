use super::ffi;

#[derive(Debug)]
pub struct Device {
	pub name: String,
	pub serial_number: String,
}

impl From<*mut ffi::tic_device> for Device {
    fn from(d: *mut ffi::tic_device) -> Self {
			let name = unsafe {
				let response_raw = ffi::tic_device_get_name(d);
				let response_cstr = std::ffi::CStr::from_ptr(response_raw);
				response_cstr.to_string_lossy().to_string().clone()
			};
			let serial_number = unsafe {
				let response_raw = ffi::tic_device_get_serial_number(d);
				let response_cstr = std::ffi::CStr::from_ptr(response_raw);
				response_cstr.to_string_lossy().to_string().clone()
			};

			Device { name, serial_number }
    }
}

pub struct OpenDevice {

}