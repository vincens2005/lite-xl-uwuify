extern crate libc;

use uwuifier::uwuify_str_sse;

use std::ffi::{CString, CStr};

use std::os::raw::c_char;


#[no_mangle]
pub extern "C" fn uwuify(s: *mut c_char) -> *const c_char {
	let c_str: &CStr = unsafe { CStr::from_ptr(s) };
	let str_slice: &str = c_str.to_str().unwrap();
	
	let uwu_str = uwuify_str_sse(str_slice);
	
	let new_uwu_str = CString::new(uwu_str).unwrap();
	let c_uwu_str: *const c_char = new_uwu_str.into_raw();

	return c_uwu_str;	
}
