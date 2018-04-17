extern crate crossterm;
extern crate winapi;
extern crate win_opacity;

use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use winapi::shared::windef::HWND as Handle;

enum Key {
	UpArrow,
	DownArrow,
	Enter
}

fn is_key_pressed(key: Key) -> bool {
	let key_code = match key {
		Key::UpArrow => 0x26,
		Key::DownArrow => 0x28,
		Key::Enter => 0x0D
	};
	unsafe {
		winapi::um::winuser::GetKeyState(key_code) < 0
	}
}

fn now_in_ms() -> u64 {
	let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
	time.as_secs() * 1000 +
	time.subsec_nanos() as u64 / 1_000_000
}

fn main() {
	let mut index: usize = 0;
	let windows = win_opacity::get_visible_windows();
	let max = windows.len();
	let mut cursor = crossterm::crossterm_cursor::get();
	println!("Which window would you like to change? (Use arrow keys)");
	render_list(index, &windows);

	let mut prev = now_in_ms();
	loop {
		let now = now_in_ms();
		let diff = now_in_ms() - prev;
		if diff > 200 {
			if is_key_pressed(Key::UpArrow) {
				if index == 0 {
					index = max - 1;
				} else {
					index = index - 1;
				}
				cursor.move_up(max as u16);
				render_list(index, &windows);
				prev = now;
			}
			if is_key_pressed(Key::DownArrow) {
				if index == max - 1 {
					index = 0;
				} else {
					index = index + 1;
				}
				cursor.move_up(max as u16);
				render_list(index, &windows);
				prev = now;
			}
		}
		if is_key_pressed(Key::Enter) {
			// This stuff just makes sure the users input doesn't carry over
			// to the actual terminal
			let mut buf = String::new();
			let _ = std::io::stdin().read_line(&mut buf);
			break;
		}
	}

	print!("Enter an opacity value (0-255): ");
	let _ = std::io::stdout().flush();
	let mut line = String::new();
	let stdin = std::io::stdin();
	let _ = stdin.read_line(&mut line);
	let opacity: u8 = line.trim().parse().unwrap();
	win_opacity::set_opacity(windows[index], opacity);
}

fn render_list(index: usize, windows: &Vec<Handle>) {
	// let windows = win_opacity::get_visible_windows();
	for (i, win) in windows.iter().enumerate() {
		let title = win_opacity::get_window_title(*win);
		println!("{} {}", if i == index { ">" } else { " "}, title);
	}
}