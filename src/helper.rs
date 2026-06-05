#![allow(non_snake_case,non_camel_case_types,dead_code)]
pub mod Helper{
    use std::{path::Path, vec};


const DBG_STR:&str = "";
const OK: i32 = 0;
const ERR: i32 = 1;
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
pub dbg: bool
}

impl CLI {
    pub fn new(d_vec: Vec<String>,pid: Option<i32>) -> CLI{
        Self{
            dict_file: d_vec,
            proc_id:pid,
            dbg: false
        }
    }
}

pub fn extract_vocab(fpath: String) -> Vec<String> {
let mut ret = vec![];
if Path::is_dir(Path::new(&fpath[..])) {
    // Entire dir contains our dictionary
    let dir = std::fs::read_dir(fpath);
    for i in &dir{

        // TODO: FIXME:
    }
}else{
    let vocab = std::fs::read(fpath).unwrap_or_else(|_| { std::process::exit(ERR)});
    let mut word = "".to_string();
    for i in vocab{
        if i != b' ' || i != b'\n'{
            word += &i.to_string()[..];
        }else{
            ret.push(word.clone());
            word.clear();
        }
    }

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