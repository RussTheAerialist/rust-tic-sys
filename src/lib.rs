use std::ptr::null_mut;

mod ffi {
    #![allow(non_camel_case_types)]
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

mod device;

pub use device::Device;
use ffi::*;

#[derive(Debug, PartialEq)]
pub enum TicError {
    Success,
    Message(String),
    Generic(*mut tic_error),
}

impl TicError {
    fn is_success(&self) -> bool {
        self == &TicError::Success
    }
}

impl From<*mut ffi::tic_error> for TicError {
    fn from(e: *mut ffi::tic_error) -> Self {
        if e.is_null() {
            unsafe {
            ffi::tic_error_free(e);
            }
            return TicError::Success;
        }

        unsafe {
            let response_raw = ffi::tic_error_get_message(e);
            let response_cstr = std::ffi::CStr::from_ptr(response_raw);
            let ret = TicError::Message(response_cstr.to_string_lossy().to_string().clone());
            ffi::tic_error_free(e);

            ret
        }
    }
}

type Result<T> = std::result::Result<T, TicError>;

pub struct DeviceList {
    devices: *mut *mut ffi::tic_device,
    count: size_t,
    pub tics: Vec<Device>,
}

impl Drop for DeviceList {
    fn drop(&mut self) {
        unsafe {
            if !self.devices.is_null() {
                let devices_array : &[*mut ffi::tic_device] = std::slice::from_raw_parts_mut(self.devices, self.count as usize);
                devices_array.into_iter().for_each(|&e| tic_device_free(e));

                tic_list_free(self.devices);
            }
        }
    }
}

pub unsafe fn get_device_list() -> Result<DeviceList> {
    let mut devices_raw: *mut ffi::tic_device = std::ptr::null_mut();
    let mut devices_raw_ptr: *mut *mut ffi::tic_device = &mut devices_raw;
    let devices: *mut *mut *mut ffi::tic_device = &mut devices_raw_ptr;

    let mut count: size_t = 0;
    let count_ptr = &mut count;
    let error: TicError = ffi::tic_list_connected_devices(devices, count_ptr).into();

    if !error.is_success() {
        return Err(error);
    }

    let devices_array : &[*mut ffi::tic_device] = std::slice::from_raw_parts_mut(devices_raw_ptr, count as usize) ;
    let tics : Vec<Device> = devices_array.into_iter().map(|&e| e.into()).collect();

    Ok(DeviceList {
        devices: devices_raw_ptr,
        count,
        tics,
    })
}

#[cfg(test)]
mod tests {
    use crate::get_device_list;

    #[test]
    fn test_device_list() {
        let res = unsafe {
            let devices = get_device_list().expect("Unable to get list of devices");
            assert_eq!(devices.count, 1);
            assert_eq!(devices.tics.len(), 1);
        };
    }
}
