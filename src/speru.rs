#![allow(non_snake_case,non_camel_case_types,dead_code)]
pub mod Speru {
use std::cmp::min; 
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



pub fn Levenshtein(word1:&str,word2:&str) -> usize {
        let (l1,l2) = (word1.len(),word2.len());
        let (word1,word2) = (word1.as_bytes(),word2.as_bytes());
        let mut dp = vec![vec![0;l1+1];l2+1];
        for j in 0..=l1 {
            dp[0][j] = j;
        }
        for i in 0..=l2{
            dp[i][0] = i;
        }
        for i in 1..=l2{
            for j in 1..=l1{
                dp[i][j] = if word1[j-1] == word2[i-1] {dp[i-1][j-1]} 
                else {min(
                    min(
                        dp[i-1][j] + 1,
                        dp[i][j-1] + 1 
                        ),
                    dp[i-1][j-1] + 1
                          )
                };
            }
        }
        dp[l2][l1]
}

#[derive(Debug,Clone)]
pub struct BK_Tree{
pub root: Option<BK_Node>
}

impl BK_Tree {
    pub fn new() -> BK_Tree {
        Self { root:None }
    }

    pub fn insert(&mut self,word: &str) {
        match &mut self.root{
            Some(x) => x.insert(word),
            None => self.root = Some(BK_Node::new(word))
        }
    }

    pub fn from_vec(vocab:Vec<String>) -> BK_Tree {
        let mut root = BK_Tree::new();
        vocab.iter().for_each( |x| root.insert(x) );
        root
    }
}

#[derive(Debug,Clone)]
// See about using word id : word mappings to prevent cloniing of strings alot
pub struct BK_Node{
pub word: String,
pub children: HashMap<usize,BK_Node>,
}

impl BK_Node {
    pub fn new(root: &str) -> BK_Node {
        Self { word: root.to_string(), children: HashMap::new()}
    }

    pub fn insert(&mut self,word: &str) {
        let dist = Levenshtein(&self.word,word);
        match self.children.get_mut(&dist) {
            Some(x) => {x.insert(word);},
            None => {self.children.insert(dist, BK_Node::new(word));}
        }
    }
   
}


pub struct O_BK_Tree{
pub root: Option<O_BK_Node>
}

impl O_BK_Tree {
    pub fn new() -> O_BK_Tree {
        Self { root:None }
    }

    pub fn insert(&mut self,id:usize) {
        match &mut self.root{
            Some(x) => x.insert(id),
            None => self.root = Some(O_BK_Node::new(id))
        }
    }

    pub fn from_vec(ids:Vec<usize>) -> O_BK_Tree {
        let mut root = O_BK_Tree::new();
        ids.iter().for_each( |x| root.insert(*x) );
        root
    }
}

#[derive(Debug,Clone)]
// See about using word id : word mappings to prevent cloniing of strings alot
pub struct O_BK_Node{
pub id: usize,
pub children: HashMap<usize,O_BK_Node>,
}

impl O_BK_Node {
    pub fn new(root: usize) -> O_BK_Node {
        Self { id: root, children: HashMap::new()}
    }

    pub fn insert(&mut self,id:usize) {
        let dist = Levenshtein(&get_str(self.id).unwrap(),&get_str(id).unwrap());
        match self.children.get_mut(&dist) {
            Some(x) => {x.insert(id);},
            None => {self.children.insert(dist, O_BK_Node::new(id));}
        }
    }
   
}






}