use std::{
    cmp::Ordering,
    ffi::CString,
    mem::{size_of, transmute_copy},
};

use winapi::{
    ctypes::c_int,
    shared::windef::HWND,
    um::winuser::*,  
};

pub fn str_to_keycode(s: &str) -> Option<u16> {
    match s.len().cmp(&1) {
        Ordering::Less => None,
        Ordering::Equal => Some(s.to_ascii_uppercase().chars().next().unwrap() as u16),
        Ordering::Greater => {
            return match s {
                "space" => Some(VK_SPACE as u16),
                "up" => Some(VK_UP as u16),
                "left" => Some(VK_LEFT as u16),
                "right" => Some(VK_RIGHT as u16),
                _ => None,
            };
        }
    }
}

pub fn key_down(vk: u16) {
    trigger_key_event(0, vk);
}

pub fn key_up(vk: u16) {
    trigger_key_event(KEYEVENTF_KEYUP, vk);
}

fn trigger_key_event(flags: u32, vk: u16) {
    let mut input = INPUT {
        type_: INPUT_KEYBOARD,
        u: unsafe {
            transmute_copy(&KEYBDINPUT {
                wVk: vk,
                wScan: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            })
        },
    };
    unsafe { SendInput(1, &mut input as LPINPUT, size_of::<INPUT>() as c_int) };
}

pub fn focus_window(app: &str) -> Result<(), String>{
    let handle = find_handle(app);
    if handle.is_null() {
        Err(format!("Cannot focus {}, seems like it is not running", app))
    } else {
        unsafe { SetForegroundWindow(handle); }       
        Ok(())
    }
    
}


fn find_handle(app: &str) -> HWND {
    let cstr = CString::new(app).expect("{} cannot be converted to a C string");
    unsafe { FindWindowA(std::ptr::null(), cstr.as_ptr()) }
}

#[cfg(test)]
mod tests {
    use super::str_to_keycode;

    #[test]
    fn test_str_to_keycode() {
        provide_samples()
            .iter()
            .for_each(|(input, expected)| assert_eq!(*expected, str_to_keycode(&input)));
    }

    fn provide_samples() -> Vec<(String, Option<u16>)> {
        vec![
            ("a".into(), Some(0x41)),
            ("w".into(), Some(0x57)),
            ("s".into(), Some(0x53)),
            ("d".into(), Some(0x44)),
            ("space".into(), Some(0x20)),
            ("up".into(), Some(0x26)),
            ("left".into(), Some(0x25)),
            ("right".into(), Some(0x27)),
            ("control".into(), None),
        ]
    }
}