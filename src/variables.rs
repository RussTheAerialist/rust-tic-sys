use super::ffi;
use super::Result;

pub struct Variables {
	pub(crate) variables: *mut ffi::tic_variables,
}

impl Variables {
	pub fn position(&self) -> i32 {
		unsafe {
			ffi::tic_variables_get_current_position(self.variables)
		}
	}
}

impl Drop for Variables {
    fn drop(&mut self) {
        unsafe {
					ffi::tic_variables_free(self.variables);
				}
    }
}