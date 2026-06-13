#![allow(non_snake_case,non_camel_case_types,dead_code)]
pub mod Speru {
use std::collections::HashMap;
use rand::Rng;
use once_cell::sync::Lazy;
use std::sync::RwLock;


pub static Id2Str_Map: Lazy<RwLock<HashMap<usize,String>>> = Lazy::new(|| RwLock::new(HashMap::new()));
pub static Str2Id_Map: Lazy<RwLock<HashMap<String,usize>>> = Lazy::new(|| RwLock::new(HashMap::new()));

pub fn set_word(word: &str) -> bool{
    let mut rnd = 0;
    loop {
        rnd = rand::rng().random_range(0..=usize::MAX);
        if !Id2Str_Map.read().unwrap().contains_key(&rnd){
            break
        }
    }
    let word = word.to_string();
    Id2Str_Map.write().unwrap().insert(rnd, word.clone());
    Str2Id_Map.write().unwrap().insert(word, rnd);
    true
} 

pub fn set_id(word: &str,id:usize) -> bool {
    if Id2Str_Map.read().unwrap().contains_key(&id){
        return false;
    }
    let word = word.to_string();
    Id2Str_Map.write().unwrap().insert(id, word.clone());
    Str2Id_Map.write().unwrap().insert(word, id);
    true
}

pub fn get_id(word:&str) -> Option<usize>{
  Str2Id_Map.read().unwrap().get(word).copied()
}

pub fn get_str(id: usize) -> Option<String>{
  Id2Str_Map.read().unwrap().get(&id).cloned()
}

pub fn contains_id(id:usize) -> bool{
    Id2Str_Map.read().unwrap().contains_key(&id)
}
pub fn contains_word(word:&str) -> bool{
    Str2Id_Map.read().unwrap().contains_key(&word.to_string())
}







}