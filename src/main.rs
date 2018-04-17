extern crate win_opacity;
use win_opacity::*;

fn main() {
	for win in get_visible_windows() {
		if get_window_title(win).contains("Firefox") {
			println!("{}", get_window_title(win));
			set_opacity(win, 220);
		}
	}
}
