//cargo rustc --release -- -C link-args="-Wl,--subsystem,windows"

extern crate user32;
extern crate winapi;

use std::ptr::{null, null_mut};
use user32::{CreateWindowExW, DefWindowProcW, RegisterClassW, GetDlgItem, GetWindowTextLengthW, GetWindowTextW, UpdateWindow,
	GetMessageW, TranslateMessage, DispatchMessageW, MessageBoxW, PostQuitMessage, SetWindowLongPtrW, GetWindowLongPtrW, LoadImageW
};
use winapi::{c_int, UINT, LPARAM, DWORD, POINT, LPMSG, LRESULT, HMENU, GWL_USERDATA, LPVOID,
	WS_OVERLAPPEDWINDOW, WS_VISIBLE, BS_PUSHBUTTON, WS_CHILD, WM_DESTROY, WM_CREATE, WM_COMMAND,
    CW_USEDEFAULT, HINSTANCE, HWND, LPCWSTR, WNDCLASSW, WPARAM, MSG, MB_OK, WS_BORDER, WS_VSCROLL
};

use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;

fn to_wstring(str : &str) -> *const u16 {
    let v : Vec<u16> = OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect();
    v.as_ptr()
}

unsafe extern "system" fn windowproc(handle: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg{
        WM_CREATE => {
			SetWindowLongPtrW(handle, GWL_USERDATA, lparam);
		}
		WM_DESTROY => {
           PostQuitMessage(0);
        }
		WM_COMMAND => {
			let lp_param = GetWindowLongPtrW(handle, GWL_USERDATA);
			let w: &mut Win = &mut *(lp_param as *mut Win);
			w.procedures(handle, wparam);
		}
        _ => {}
    }
    return DefWindowProcW(handle, msg, wparam, lparam);
}

struct Win {
	handle: HWND,
	class: WNDCLASSW
}

trait WinProc{
	fn procedures(&mut self, handle: HWND, wparam: WPARAM) -> i32;
}

impl Win{
	fn new(title: &'static str, width: i32, height: i32) -> Win {
		unsafe {
			//let icon = LoadImageW(0 as HINSTANCE, to_wstring("ico.ico"), 1, 0, 0);
			//let cursor = LoadImageW(0 as HINSTANCE, to_wstring("ico.ico"), 2, 0, 0);
			let class = WNDCLASSW {
				style: 0,
				lpfnWndProc: Some(windowproc),
				cbClsExtra: 0, 
				cbWndExtra: std::mem::size_of::<c_int>() as c_int, //default 0
				hInstance: 0 as HINSTANCE,
				hIcon: null_mut(),
				hCursor: null_mut(),
				hbrBackground: null_mut(),
				lpszMenuName: null(),
				lpszClassName: to_wstring(title)
			};
			let atom = RegisterClassW(&class);
			
			let mut w = Win {
				handle: 0 as HWND,
				class: class,
			};
			let lp_param: LPVOID = &mut w as *mut _ as LPVOID;
			w.handle = CreateWindowExW(
				0,
				atom as LPCWSTR,
				class.lpszClassName,
				WS_OVERLAPPEDWINDOW | WS_VISIBLE,
				CW_USEDEFAULT,
				CW_USEDEFAULT,
				width,
				height,
				null_mut(),
				null_mut(),
				class.hInstance,
				lp_param
			);
			w
		}
	}
	
	fn update(&mut self) -> &mut Win {
		unsafe {
			UpdateWindow(self.handle);
		}
		self
	}
	
	fn win_loop(&mut self) -> &mut Win {
		unsafe {
			let mut msg = MSG {
				hwnd: null_mut(),
				message: 0 as UINT,
				wParam: 0 as WPARAM,
				lParam: 0 as LPARAM,
				time: 0 as DWORD,
				pt: POINT { x: 0, y: 0}
			};
			loop {
				let ret = GetMessageW(&mut msg as LPMSG, null_mut(), 0 as UINT, 0 as UINT);
				if ret == 0 {
					break;
				}
				TranslateMessage(&msg as *const MSG);
				DispatchMessageW(&msg as *const MSG);
			}
		}
		self
	}
	
	fn add_button(&mut self, title: &str,
							x: i32,
							y: i32,
							width: i32,
							height: i32,
							id: i32) -> HWND {
		unsafe {
			CreateWindowExW(
				0,
				to_wstring("BUTTON"),
				to_wstring(title),
				WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
				x,
				y,
				width,
				height,
				self.handle,
				id as HMENU,
				self.class.hInstance,
				null_mut()
			)
		}
	}
	
	fn add_edit(&mut self, title: &str,
							x: i32,
							y: i32,
							width: i32,
							height: i32,
							id: i32) -> HWND {
		unsafe {
			CreateWindowExW(
				0,
				to_wstring("EDIT"),
				to_wstring(title),
				WS_CHILD | WS_VISIBLE | WS_BORDER | WS_VSCROLL | 4,
				x,
				y,
				width,
				height,
				self.handle,
				id as HMENU,
				self.class.hInstance,
				null_mut()
			)
		}
	}
}

fn main() {
	const BTN: i32 = 1;
	const EDT: i32 = 2;
	
	impl WinProc for Win {
		fn procedures(&mut self, handle: HWND, wparam: WPARAM) -> i32 {
			unsafe {
				match wparam as c_int{
					BTN => {
						let edit = GetDlgItem(handle, EDT);
						let len = GetWindowTextLengthW(edit) + 1;
						
						let mut v: Vec<u16> = Vec::with_capacity(len as usize);
						let edit_str = v.as_mut_ptr();
						GetWindowTextW(edit, edit_str, len);
						//String::from_utf16_lossy(&v);
						MessageBoxW(handle, edit_str, to_wstring("Message"), MB_OK);
					}
					_ => {}
				}
			}
			1
		}
	}
	
	let mut window = Win::new("My_window", 600, 500);
	window.add_button("Click button", 5, 5, 80, 32, BTN);
	window.add_edit("What's wrong?", 5, 60, 160, 52, EDT);
	window.update();
	window.win_loop();
}
