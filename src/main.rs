#![allow(non_snake_case,non_camel_case_types,dead_code)]

mod helper;
mod speru;
mod read;
mod tree;
mod dist;

use std::path::Path;
use speru::Speru;
use read::Read::key_to_char;
use evdev::{Device, EventSummary, KeyCode};
use helper::Helper;

use crate::tree::Tree;
fn main() -> std::io::Result<()>{
let clargs = Helper::Parse_args();
let mut wordlist = vec![];
if clargs.dbg {
    println!("{:?}\n",clargs);
}

for i in &clargs.dict_file {
    wordlist.append(&mut Helper::extract_vocab(Path::new(i),clargs.retain_capitalise));
}
if clargs.dbg {println!("{wordlist:?}");}

if clargs.optimise{
    wordlist.iter().for_each(|x| if !Speru::set_word(x) {std::process::exit(Helper::ERR)});
    let id_list = Speru::Id2Str_Map.read().unwrap().clone().into_keys().collect::<Vec<usize>>();
    let root = Tree::O_BK_Tree::from_vec(id_list); 
if let Some(pid) = clargs.proc_id {
    // Add the pid attach code here     


    Ok(())
}else{
let mut dev = Device::open("/dev/input/event4")?;
    let mut current_word = String::new();
    let mut shift = false;
    loop {
        for event in dev.fetch_events()? {
            if let EventSummary::Key(_, key, value) = event.destructure() {
                if value == 0 {
                    continue;
                }
                match key {
                    KeyCode::KEY_LEFTSHIFT | KeyCode::KEY_RIGHTSHIFT => {
                        shift = true;
                    }
                    KeyCode::KEY_BACKSPACE => {
                        current_word.pop();
                    }
                    KeyCode::KEY_SPACE | KeyCode::KEY_ENTER => {
                        if !current_word.is_empty() {
                            println!("Completed word: {}", current_word);
                            println!("{:?}",root.get_n_strs(&current_word));
                            current_word.clear();
                        }
                    }
                    _ => {
                        if let Some(c) = key_to_char(key, shift) {
                            current_word.push(c);
                            println!("Current word = {}", current_word);
                        }
                    }
                }
            }
        }
    }
}
}else{
    wordlist.iter().for_each(|x| if !Speru::set_word(x) {std::process::exit(Helper::ERR)});
    let root = Tree::BK_Tree::from_vec(wordlist); 
if let Some(pid) = clargs.proc_id {
    // Add the pid attach code here     


    Ok(())
}else{
    let mut dev = Device::open("/dev/input/event4")?;
    
    let mut current_word = String::new();
    let mut shift = false;
    loop {

        for event in dev.fetch_events()? {
            if let EventSummary::Key(_, key, value) = event.destructure() {
                if value == 0 {
                    continue;
                }
                match key {
                    KeyCode::KEY_LEFTSHIFT | KeyCode::KEY_RIGHTSHIFT => {
                        shift = true;
                    }
                    KeyCode::KEY_BACKSPACE => {
                        current_word.pop();
                    }
                    KeyCode::KEY_SPACE | KeyCode::KEY_ENTER => {
                        if !current_word.is_empty() {
                            println!("Completed word: {}", current_word);
                            println!("{:?}",root.get_n_strs(&current_word));
                            current_word.clear();
                        }
                    }
                    _ => {
                        if let Some(c) = key_to_char(key, shift) {
                            current_word.push(c);
                            println!("Current word = {}", current_word);
                        }
                    }
                }
            }
        }
    }

}
}



}
