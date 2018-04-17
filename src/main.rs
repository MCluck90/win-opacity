#[cfg(windows)]
extern crate winapi;

use std::str;

use winapi::shared::minwindef::{BOOL, LPARAM, TRUE};
use winapi::um::winnt::LPSTR;
use winapi::shared::windef::HWND;

type Handle = HWND;

extern "system" fn enum_windows_callback(handle: Handle, lparam: LPARAM) -> BOOL {
	let windows_vec = lparam as *mut Vec<Handle>;

	if let Some(ref mut windows_collector) = unsafe { windows_vec.as_mut() } {
		windows_collector.push(handle);
	}
	TRUE
}

fn get_all_windows() -> Vec<Handle> {
	let mut windows: Vec<Handle> = Vec::new();
	unsafe {
		winapi::um::winuser::EnumWindows(
			Some(enum_windows_callback),
			&mut windows as *mut _ as LPARAM,
		)
	};
	windows
}

fn get_window_title(handle: Handle) -> String {
	const MAX_COUNT: usize = 256;
	let mut buffer = [0u8; MAX_COUNT];
	let mut result = String::new();
	unsafe {
		let length = winapi::um::winuser::GetWindowTextA(handle, &mut buffer as *mut _ as LPSTR, MAX_COUNT as i32);
		if length > 0 {
			let exact_text = std::slice::from_raw_parts(buffer.as_ptr(), length as usize);
			result = String::from_utf8_lossy(exact_text).to_string();
		}
	}
	result
}

fn main() {
	let windows = get_all_windows();
	for win in windows {
		// println!("{:?}", win);
		println!("{}", get_window_title(win));
	}
}
