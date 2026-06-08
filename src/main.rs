#![allow(non_snake_case,non_camel_case_types,dead_code)]

mod helper;
mod speru;

use std::path::Path;
use speru::Speru;
use helper::Helper;
fn main() {
let clargs = Helper::Parse_args();
let mut wordlist = vec![];
if clargs.dbg {
    println!("{:?}\n",clargs);
}

for i in &clargs.dict_file {
    wordlist.append(&mut Helper::extract_vocab(Path::new(i)));
}

if clargs.optimise{
    wordlist.iter().for_each(|x| if !Speru::set_word(x) {std::process::exit(Helper::ERR)});
    let id_list = Speru::Id2Str_Map.read().unwrap().clone().into_keys().collect::<Vec<usize>>();
    let root = Speru::O_BK_Tree::from_vec(id_list); 
}else{
    wordlist.iter().for_each(|x| if !Speru::set_word(x) {std::process::exit(Helper::ERR)});
    let root = Speru::BK_Tree::from_vec(wordlist); 
}

if let Some(pid) = clargs.proc_id {

}else{
 // some dep lets us take direct eventx input do that instead of attaching to a pid's  fd/0
}

}
