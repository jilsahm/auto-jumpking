use std::mem::{size_of, transmute_copy};

use winapi::{
    um::winuser::*,
    ctypes::c_int,
};

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