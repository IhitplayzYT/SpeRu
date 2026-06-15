pub mod Read{
use evdev::{Device, EventSummary, KeyCode};
use std::io;

pub fn key_to_char(key: KeyCode, shift: bool) -> Option<char> {
    match (key, shift) {
        (KeyCode::KEY_A, false) => Some('a'),
        (KeyCode::KEY_B, false) => Some('b'),
        (KeyCode::KEY_C, false) => Some('c'),
        (KeyCode::KEY_D, false) => Some('d'),
        (KeyCode::KEY_E, false) => Some('e'),
        (KeyCode::KEY_F, false) => Some('f'),
        (KeyCode::KEY_G, false) => Some('g'),
        (KeyCode::KEY_H, false) => Some('h'),
        (KeyCode::KEY_I, false) => Some('i'),
        (KeyCode::KEY_J, false) => Some('j'),
        (KeyCode::KEY_K, false) => Some('k'),
        (KeyCode::KEY_L, false) => Some('l'),
        (KeyCode::KEY_M, false) => Some('m'),
        (KeyCode::KEY_N, false) => Some('n'),
        (KeyCode::KEY_O, false) => Some('o'),
        (KeyCode::KEY_P, false) => Some('p'),
        (KeyCode::KEY_Q, false) => Some('q'),
        (KeyCode::KEY_R, false) => Some('r'),
        (KeyCode::KEY_S, false) => Some('s'),
        (KeyCode::KEY_T, false) => Some('t'),
        (KeyCode::KEY_U, false) => Some('u'),
        (KeyCode::KEY_V, false) => Some('v'),
        (KeyCode::KEY_W, false) => Some('w'),
        (KeyCode::KEY_X, false) => Some('x'),
        (KeyCode::KEY_Y, false) => Some('y'),
        (KeyCode::KEY_Z, false) => Some('z'),

        (KeyCode::KEY_A, true) => Some('A'),
        (KeyCode::KEY_B, true) => Some('B'),
        (KeyCode::KEY_C, true) => Some('C'),
        (KeyCode::KEY_D, true) => Some('D'),
        (KeyCode::KEY_E, true) => Some('E'),
        (KeyCode::KEY_F, true) => Some('F'),
        (KeyCode::KEY_G, true) => Some('G'),
        (KeyCode::KEY_H, true) => Some('H'),
        (KeyCode::KEY_I, true) => Some('I'),
        (KeyCode::KEY_J, true) => Some('J'),
        (KeyCode::KEY_K, true) => Some('K'),
        (KeyCode::KEY_L, true) => Some('L'),
        (KeyCode::KEY_M, true) => Some('M'),
        (KeyCode::KEY_N, true) => Some('N'),
        (KeyCode::KEY_O, true) => Some('O'),
        (KeyCode::KEY_P, true) => Some('P'),
        (KeyCode::KEY_Q, true) => Some('Q'),
        (KeyCode::KEY_R, true) => Some('R'),
        (KeyCode::KEY_S, true) => Some('S'),
        (KeyCode::KEY_T, true) => Some('T'),
        (KeyCode::KEY_U, true) => Some('U'),
        (KeyCode::KEY_V, true) => Some('V'),
        (KeyCode::KEY_W, true) => Some('W'),
        (KeyCode::KEY_X, true) => Some('X'),
        (KeyCode::KEY_Y, true) => Some('Y'),
        (KeyCode::KEY_Z, true) => Some('Z'),

        _ => None,
    }
}


    
}