#![allow(non_snake_case,non_camel_case_types,dead_code)]
pub mod Helper{
    use std::{path::Path, vec};


pub const DBG_STR:&str = "";
pub const OK: i32 = 0;
pub const ERR: i32 = 1;
// Just a spce seperated or new line based custom file from which we'll extract our vocabulary
const DEFAULT_DICTIONARY: &str = "dictionary/en-dict.dict";

pub fn Help(){
println!("{DBG_STR}\n");
std::process::exit(OK);
}


#[derive(Debug,Clone,PartialEq)]
pub struct CLI {
pub dict_file: Vec<String>,
pub proc_id: Option<i32>,
pub dbg: bool,
pub optimise: bool,
pub learn: bool
}

impl CLI {
    pub fn new(d_vec: Vec<String>,pid: Option<i32>) -> CLI{
        Self{
            dict_file: d_vec,
            proc_id:pid,
            dbg: false,
            optimise: false,
            learn:false
        }
    }
}

pub fn extract_vocab(fpath: &Path) -> Vec<String> {
let mut ret = vec![];
if fpath.is_dir() {
    for i in  std::fs::read_dir(&fpath).unwrap().flatten(){
        ret.append(&mut extract_vocab(&i.path()));
    }
}else if fpath.ends_with(".dict") || fpath.ends_with(".txt") {
    let bytes = std::fs::read(fpath).unwrap_or_else(|_| { std::process::exit(ERR)});
    let file = String::from_utf8(bytes).unwrap();
    ret.append(&mut String::from_utf8(file.bytes().map(|ch| if ch == b'\n' {b' '} else {ch}).collect::<Vec<u8>>()).unwrap().split_ascii_whitespace().into_iter().map(|x| x.to_string()).collect::<Vec<String>>());
}
    ret
}




pub fn Parse_args() -> CLI {
let mut ret = CLI::new(vec![DEFAULT_DICTIONARY.to_string()], None);
let args = std::env::args().collect::<Vec<String>>();
    for i in &args{
        if i.as_str() == "-h" || i.as_str() == "--help" || i.as_str() == "-H" || i.as_str() == "--HELP"{
            Help();
        }
        else if i.as_str() == "-d" || i.as_str() == "--debug" || i.as_str() == "-D" || i.as_str() == "--DEBUG"{
            ret.dbg = true;
        }
        else if i.as_str() == "-o" || i.as_str() == "--optimise" || i.as_str() == "-O" || i.as_str() == "--OPTIMISE"{
            ret.optimise = true;
        }
        else if i.as_str() == "-l" || i.as_str() == "--learn" || i.as_str() == "-L" || i.as_str() == "--LEARN"{
            ret.learn = true;
        }
        else if i.as_str().starts_with("-p="){
            ret.proc_id = Some(i[3..].parse::<i32>().expect("Expected a valid non negative pid"))
        }

        if let Ok(ID) = i.parse::<i32>(){
            ret.proc_id = Some(ID);
        }else{
            if i.starts_with("-d=") || i.starts_with("-idir=") || i.starts_with("-dir="){
                if let Some(idx) = i.find("=") {
                    ret.dict_file.push(i[idx+1..].to_string());
                }else {
                    Help();
                }
            
            }

        }

    }
    ret
}





}