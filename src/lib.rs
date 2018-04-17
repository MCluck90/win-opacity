#[cfg(windows)]
extern crate winapi;

use winapi::shared::minwindef::{BOOL, DWORD, LPARAM, TRUE};
use winapi::shared::windef::HWND as Handle;
use winapi::um::winnt::{LONG, LPSTR};
use winapi::um::winuser;

extern "system" fn enum_windows_callback(handle: Handle, lparam: LPARAM) -> BOOL {
	let windows_vec = lparam as *mut Vec<Handle>;

	if let Some(ref mut windows_collector) = unsafe { windows_vec.as_mut() } {
		windows_collector.push(handle);
	}
	TRUE
}

pub fn get_all_windows() -> Vec<Handle> {
	let mut windows: Vec<Handle> = Vec::new();
	unsafe {
		winuser::EnumWindows(
			Some(enum_windows_callback),
			&mut windows as *mut _ as LPARAM,
		)
	};
	windows
}

pub fn get_window_title(handle: Handle) -> String {
	const MAX_COUNT: usize = 256;
	let mut buffer = [0u8; MAX_COUNT];
	let mut result = String::new();
	unsafe {
		let length = winuser::GetWindowTextA(handle, &mut buffer as *mut _ as LPSTR, MAX_COUNT as i32);
		if length > 0 {
			let exact_text = std::slice::from_raw_parts(buffer.as_ptr(), length as usize);
			result = String::from_utf8_lossy(exact_text).trim().to_string();
		}
	}
	result
}

pub fn is_window_visible(handle: Handle) -> bool {
	unsafe {
		winuser::IsWindowVisible(handle) == TRUE
	}
}

pub fn get_visible_windows() -> Vec<Handle> {
	get_all_windows()
		.into_iter()
		.filter(|&win| is_window_visible(win) && get_window_title(win).len() > 0)
		.collect::<Vec<_>>()
}

pub fn set_opacity(handle: Handle, opacity: u8) {
	const GWL_EXSTYLE: i32 = -20;
	const WS_EX_LAYERED: LONG = 0x80000;
	const LWA_ALPHA: DWORD = 0x2;
	unsafe {
		let window_long = winuser::GetWindowLongA(handle, GWL_EXSTYLE);
		winuser::SetWindowLongA(handle, GWL_EXSTYLE, window_long | WS_EX_LAYERED);
		winuser::SetLayeredWindowAttributes(handle, 0, opacity, LWA_ALPHA);
	}
}