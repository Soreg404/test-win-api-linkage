use std::ffi::{c_char, c_int, c_uint, c_void};

#[link(name="user32")]
unsafe extern "C" {
	fn MessageBoxA(
		h_instance: *const c_void,
		content: *const c_char,
		title: *const c_char,
		mg_type: c_uint,
	) -> c_int;
}

fn main() {
	unsafe {
		MessageBoxA(
			std::ptr::null(),
			c"content".as_ptr(),
			c"title".as_ptr(),
			0,
		);
	}
}
