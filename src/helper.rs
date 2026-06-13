#![allow(non_snake_case,non_camel_case_types,dead_code)]
pub mod Helper{
    use std::{path::Path, vec};

pub const D_THRESHOLD:usize = 3;
pub const DBG_STR:&str = "SpeRu - Spell Recognition & Runtime Utility

USAGE: ./SpeRu [OPTIONS] [PID]

DESCRIPTION:
SpeRu provides spell correction, word prediction, dictionary learning,
and process-attached text analysis.

OPTIONS:
-h, --help
Display this help message and exit.

-d, --debug
    Enable verbose debug output.

-o, --optimise
    Enable optimisation mode.
    May trade startup cost for improved runtime performance.

-l, --learn
    Enable learning mode.
    Learned words and corrections may be persisted for future use.

-r, --retain 
    Retains the capitalisation of words in dictionary files provided

-p=<PID>
    Attach SpeRu to the specified process ID.

    Example:
        -p=1234

<PID>
    A numeric PID may also be supplied directly without -p.

    Example:
        SpeRu 1234

-m=<METRIC>
--metric=<METRIC>
    Allows to tweak the Distance function used by the spell corrector 

    <METRIC> can have the support following distance functions:
        Levenshtein => Levenshtein[5] -> Takes max distance for prune
        DamerauLevenshtein => DamerauLevenshtein[5] -> Takes max distance for prune
        Hamming => Hamming[5] -> Takes max distance for prune
        Qgram => Qgram[5,3] -> Takes max distance for prune and Degree of Qgrams
        Lcs => Lcs[5] -> Takes max distance for prune
        DoubleMetaphone => DoubleMetaphone[5,AVG] -> Takes max distance for prune and the Aggregator metric for getting numeric result from phonetic cross product

    Note:Shortforms can also be used:
    <METRIC SHORTFORMS> lev, d_lev, ham, qg, lcs, d_meta repectively


-d=<FILE>
-dir=<FILE>
-idir=<FILE>
    Load a dictionary file.

    May be specified multiple times.

    Examples:
        -d=/usr/share/dict/words
        -dir=./custom.dict
        -idir=/opt/dictionaries/medical.dict


EXAMPLES:

Display help:
    SpeRu --help

Attach to process:
    SpeRu -p=4242

Attach using positional PID:
    SpeRu 4242

Enable debug mode:
    SpeRu --debug 4242

Enable learning mode:
    SpeRu --learn

Use DamerauLevenshtein with a prune distance of 2:
    SpeRu --metric=DamerauLevenshtein[2]

Use Triagrams distance with a prune distance of 1:
    SpeRu --metric=Qgram[1,3]

Use DoubleMetaphone with Minimum cross product between primary and secondary phonetics with a prune distance of 4
    SpeRu --metric=DoubleMetaphone[4,MIN]

Load a custom dictionary:
    SpeRu -d=english.dict

Load multiple dictionaries:
    SpeRu \
        -d=english.dict \
        -d=technical.dict \
        -d=medical.dict

Optimised learning session:
    SpeRu \
        --optimise \
        --learn \
        -d=english.dict \
        -p=4242


EXIT STATUS:
0   Success
1   Invalid arguments or runtime failure

AUTHOR: Ihit Acharya
SpeRu Project
";
pub const OK: i32 = 0;
pub const ERR: i32 = 1;
// Just a spce seperated or new line based custom file from which we'll extract our vocabulary
const DEFAULT_DICTIONARY: &str = "dictionary/en-dict.dict";

pub fn Help(){
println!("{DBG_STR}\n");
std::process::exit(OK);
}

#[derive(Debug,Clone,PartialEq)]
pub enum Metric {
Levenshtein(usize),
DamerauLevenshtein(usize),
Hamming(usize),
Qgram(usize,usize),
Lcs(usize),
DoubleMetaphone(usize,DMeta)
}

#[derive(Debug,Clone,PartialEq,Copy)]
pub enum DMeta{
    AVG,
    MIN,
    MAX,
}



#[derive(Debug,Clone,PartialEq)]
pub struct CLI {
pub dict_file: Vec<String>,
pub retain_capitalise:bool,
pub proc_id: Option<i32>,
pub dbg: bool,
pub optimise: bool,
pub learn: bool,
pub metric: Metric
}

impl CLI {
    pub fn new(d_vec: Vec<String>,pid: Option<i32>) -> CLI{
        Self{
            dict_file: d_vec,
            proc_id:pid,
            dbg: false,
            optimise: false,
            learn:false,
            retain_capitalise:false,
            metric: Metric::Levenshtein(D_THRESHOLD)
        }
    }
}

pub fn extract_vocab(fpath: &Path,retain:bool) -> Vec<String> {
let mut ret = vec![];
if fpath.is_dir() {
    for i in  std::fs::read_dir(&fpath).unwrap().flatten(){
        ret.append(&mut extract_vocab(&i.path(),retain));
    }
}else if fpath.ends_with(".dict") || fpath.ends_with(".txt") {
    let bytes = std::fs::read(fpath).unwrap_or_else(|_| { std::process::exit(ERR)});
    let file = String::from_utf8(bytes).unwrap();
    ret.append(&mut String::from_utf8(file.bytes().map(|ch| if ch == b'\n' {b' '} else {ch}).collect::<Vec<u8>>()).unwrap().split_ascii_whitespace().into_iter().map(|x| {if !retain{x.to_lowercase().to_string()} else {x.to_string()}}).collect::<Vec<String>>());

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
        else if i.as_str() == "-r" || i.as_str() == "--retain" || i.as_str() == "-R" || i.as_str() == "--RETAIN"{
            ret.retain_capitalise = true;
        }
        else if i.as_str() == "-l" || i.as_str() == "--learn" || i.as_str() == "-L" || i.as_str() == "--LEARN"{
            ret.learn = true;
        }
        else if i.as_str().starts_with("-m=") || i.as_str().starts_with("--metric="){
            let idx = i.find("=").unwrap();
            let i = &i[idx+1..];
            if let Some(lb) = i.find("["){
                let rb = i[lb..].find("]").unwrap();
                let name = &i[..lb];
                let pl = i[lb+1..rb].trim();

            /*
            pub enum Metric {
            Levenshtein(usize),
            DamerauLevenshtein(usize),
            Hamming(usize),
            Qgram(usize,usize),
            Lcs(usize),
            DoubleMetaphone(usize,DMeta)
            }

            pub enum DMeta{
                AVG,
                MIN,
                MAX,
            }
            */

                match &name.to_lowercase()[..]{
                    "levenshtein" | "lev" => {ret.metric = Metric::Levenshtein(pl.parse::<usize>().expect("Levenshtein expects only one parameter the dist range.Eg: Levenshtein[5]"));},
                    "dameraulevenshtein" | "d_lev" => {ret.metric = Metric::DamerauLevenshtein(pl.parse::<usize>().expect("DamerauLevenshtein expects only one parameter the dist range.Eg: DamerauLevenshtein[5]"));},
                    "hamming" | "ham" => {ret.metric = Metric::Hamming(pl.parse::<usize>().expect("Hamming expects only one parameter the dist range.Eg: Hamming[5]"));},
                    "qgram" | "qg" => {
                        let (p1,p2) =  pl.split_at(pl.find(",").expect("Qgram expects two parameters the dist range and the degree.Eg: Qgram[5,3] -> Initialises a triagram distance approach"));
                        ret.metric = Metric::Qgram(p1.parse::<usize>().expect("Dist range in a unsigned integer"),p2.parse::<usize>().expect("Degree in a unsigned integer"));
                    } ,
                    "lcs" =>  {ret.metric = Metric::Lcs(pl.parse::<usize>().expect("Lcs expects only one parameter the dist range.Eg: Lcs[5]"));},
                    "doublemetaphone" | "d_meta" => {
                        let (p1,p2) =  pl.split_at(pl.find(",").expect("DoubleMetaphone expects two parameters the dist range and the Aggregating Metric.Eg: DoubleMetaphone[5,AVG] -> Initialises a DoubleMetaphone that averages out primary and secondary phonetic cross product"));
                        ret.metric = Metric::DoubleMetaphone(p1.parse::<usize>().expect("Dist range in a unsigned integer"),match &p2.trim().to_lowercase()[..]{
                            "avg" | "average" => DMeta::AVG,
                            "min" | "minimum" => DMeta::MIN,
                            "max" | "maximum" => DMeta::MAX,
                             _ => {panic!("DoubleMetaphone Aggregator can be from ONLY following Max,Min,Avg")}
                        });

                    },
                    _ => std::process::exit(ERR),
                }

            }
            



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