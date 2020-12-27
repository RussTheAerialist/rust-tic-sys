use super::ffi;
use super::{Result, TicError};

#[derive(Debug)]
pub struct Device {
	pub name: String,
	pub serial_number: String,
	device: *mut ffi::tic_device,
}

impl Device {
	pub fn open(self) -> Result<OpenDevice> {
		let mut handle : *mut ffi::tic_handle = std::ptr::null_mut();

		let error : TicError = unsafe {
			let handle_ptr : *mut *mut ffi::tic_handle = &mut handle;
			ffi::tic_handle_open(self.device, handle_ptr).into()
		};

		error.ok(|| ())?;

		let firmware = unsafe {
			let response_raw = ffi::tic_get_firmware_version_string(handle);
			let response_cstr = std::ffi::CStr::from_ptr(response_raw);
			response_cstr.to_string_lossy().to_string().clone()
		};

		Ok(OpenDevice { handle, firmware })
	}
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

			Device { name, serial_number, device: d }
    }
}

pub struct OpenDevice {
	pub firmware: String,
	handle: *mut ffi::tic_handle,
}

impl OpenDevice {
	pub fn finish_startup(&self) -> Result<()> {
		let error : TicError = unsafe {
			ffi::tic_exit_safe_start(self.handle).into()
		};

		error.ok(|| ())
	}

	pub fn keep_alive(&self) -> Result<()> {
		let error : TicError = unsafe {
			ffi::tic_reset_command_timeout(self.handle).into()
		};

		error.ok(|| ())
	}

	pub fn set_position(&self, position: i32) -> Result<()> {
		let error : TicError = unsafe {
			ffi::tic_set_target_position(self.handle, position).into()
		};

		error.ok(|| ())
	}

}

impl Drop for OpenDevice {
    fn drop(&mut self) {
			unsafe {
				ffi::tic_handle_close(self.handle);
			}
    }
}