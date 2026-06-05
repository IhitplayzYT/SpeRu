#![allow(non_snake_case,non_camel_case_types,dead_code)]

mod helper;
use std::sync::atomic;

use helper::Helper;
fn main() {
let clargs = Helper::Parse_args();
let mut wordlist = vec![];
if clargs.dbg {
    println!("{:?}\n",clargs);
}

for i in &clargs.dict_file {
       wordlist.append(&mut Helper::extract_vocab(i.to_string()));
}

if let Some(pid) = clargs.proc_id {

}else{
 // some dep lets us take direct eventx input do that instead of attaching to a pid's  fd/0
}

}
