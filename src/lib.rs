extern crate c_string;
extern crate libc;

use uwuifier::uwuify_str_sse;
// use terminated::NulTerminatedStr;
use std::ffi::CString;
use std::ffi::CStr;
//use c_string::CStrBuf;
use std::os::raw::c_char;


#[no_mangle]
pub extern "C" fn uwuify(s: *mut c_char) -> *const c_char {
	let c_str: &CStr = unsafe { CStr::from_ptr(s) };
	let str_slice: &str = c_str.to_str().unwrap();
	
	let uwu_str = uwuify_str_sse(str_slice);
	
	println!("{}", uwu_str);
	// uwu_str.push('\0');
	let new_uwu_str = CString::new(uwu_str).unwrap();
	let c_uwu_str: *const c_char = new_uwu_str.as_ptr();
	
	let owned_fmt = CString::new("\r\n\t%s\r\n").unwrap();
	let fmt: *const c_char = owned_fmt.as_ptr();
	
	unsafe {
		libc::printf(fmt, c_uwu_str);
	}
	return c_uwu_str;
	
	// let new_c_str = NulTerminatedStr::from_str_with_nul(&uwu_str);
	// println!("{}", new_c_str);
	// return NulTerminatedStr::as_ptr(&new_c_str);
}
