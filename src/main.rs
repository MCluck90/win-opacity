#[cfg(windows)]
extern crate winapi;

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
			result = String::from_utf8_lossy(exact_text).trim().to_string();
		}
	}
	result
}

fn is_window_visible(handle: Handle) -> bool {
	unsafe {
		winapi::um::winuser::IsWindowVisible(handle) == TRUE
	}
}

fn get_visible_windows() -> Vec<Handle> {
	get_all_windows()
		.into_iter()
		.filter(|&win| is_window_visible(win) && get_window_title(win).len() > 0)
		.collect::<Vec<_>>()
}

fn main() {
	for win in get_visible_windows() {
		println!("{}", get_window_title(win));
	}
}
