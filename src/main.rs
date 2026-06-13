#![allow(non_snake_case,non_camel_case_types,dead_code)]

mod helper;
mod speru;
mod read;
mod tree;
mod dist;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use speru::Speru;
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

if clargs.optimise{
    wordlist.iter().for_each(|x| if !Speru::set_word(x) {std::process::exit(Helper::ERR)});
    let id_list = Speru::Id2Str_Map.read().unwrap().clone().into_keys().collect::<Vec<usize>>();
    let root = Tree::O_BK_Tree::from_vec(id_list); 
}else{
    wordlist.iter().for_each(|x| if !Speru::set_word(x) {std::process::exit(Helper::ERR)});
    let root = Tree::BK_Tree::from_vec(wordlist); 
}

if let Some(pid) = clargs.proc_id {
    // Add the pid attach code here     


    Ok(())
}else{
    let mut tty = File::open("/dev/pts/5")?;
    let mut buf = [0u8; 4096];
    loop {
        let n = tty.read(&mut buf)?;
        if n > 0 {
            let s = String::from_utf8_lossy(&buf[..n]);
            
            // do the autcorrect over here 

        }
    }
}

}
