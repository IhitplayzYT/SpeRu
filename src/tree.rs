pub mod Tree{

    use crate::{dist::Dist, helper::Helper::{CLI, D_THRESHOLD, Metric}, speru::Speru::*};
    use std::collections::HashMap;
    


pub struct BK_Tree{
pub root: Option<BK_Node>,
pub dist: usize,
pub distance_fxn: Box<dyn Fn(&str,&str) -> usize>
}

impl BK_Tree {
    pub fn new(mtrc: Metric) -> BK_Tree {
        let (dist, distance_fxn): (usize,Box<dyn Fn(&str, &str) -> usize>) = match mtrc{
            Metric::Levenshtein(y) => (y, Box::new(|a,b | Dist::Levenshtein(a,b))),
            Metric::DamerauLevenshtein(y) => (y, Box::new(|a,b | Dist::damerau_levenshtein(a,b))),
            Metric::Hamming(y) => (y, Box::new(|a,b | Dist::hamming(a,b))),
            Metric::Qgram(y,x) => (y, Box::new(move |a,b | Dist::qgram_distance(a,b,x))),
            Metric::Lcs(y) => (y, Box::new(|a,b | Dist::lcs_length(a,b))),
            Metric::DoubleMetaphone(y,x) => (y, Box::new(move |a,b | Dist::metaphone_distance(a,b,&x))),
        };
        Self { root:None,distance_fxn,dist}
    }

    pub fn insert(&mut self,word: &str) {
        match &mut self.root{
            Some(x) => x.insert(word,&self.distance_fxn),
            None => self.root = Some(BK_Node::new(word))
        }
    }

    pub fn from_vec(vocab:Vec<String>) -> BK_Tree {
        let mut root = BK_Tree::new(Metric::Levenshtein(D_THRESHOLD));
        vocab.iter().for_each( |x| root.insert(x) );
        root
    }
    pub fn get_n_strs(&self,word: &str) -> Vec<String> {
    let mut ret = Vec::new();
        if let Some(root) = &self.root {
            root.get_n_strs(word, self.dist,&mut ret,&self.distance_fxn)
        }
        ret
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

    pub fn insert(&mut self,word: &str,d_func: &Box<dyn Fn(&str,&str) -> usize>) {
        let dist = d_func(&self.word,word);
        match self.children.get_mut(&dist) {
            Some(x) => {x.insert(word,d_func);},
            None => {self.children.insert(dist, BK_Node::new(word));}
        }
    }

    pub fn get_n_strs(&self,word:&str,dist:usize,ret:&mut Vec<String>,d_func: &Box<dyn Fn(&str,&str) -> usize>){
        let d = d_func(&self.word[..],word);
        if d <= dist{
            ret.push(self.word.clone());
        }
        let (mn,mx) = (d.saturating_sub(dist),d.saturating_add(dist));
         for (&e_dist,child) in &self.children{
            if e_dist >= mn && e_dist <= mx{
                child.get_n_strs(word, dist,ret,d_func);
            }

         }
    }

   
}


pub struct O_BK_Tree{
pub root: Option<O_BK_Node>,
pub dist: usize,
pub distance_fxn: Box<dyn Fn(&str,&str) -> usize>
}

impl O_BK_Tree {
    pub fn new(mtrc: Metric) -> O_BK_Tree {
        let (dist, distance_fxn): (usize,Box<dyn Fn(&str, &str) -> usize>) = match mtrc{
            Metric::Levenshtein(y) => (y, Box::new(|a,b | Dist::Levenshtein(a,b))),
            Metric::DamerauLevenshtein(y) => (y, Box::new(|a,b | Dist::damerau_levenshtein(a,b))),
            Metric::Hamming(y) => (y, Box::new(|a,b | Dist::hamming(a,b))),
            Metric::Qgram(y,x) => (y, Box::new(move |a,b | Dist::qgram_distance(a,b,x))),
            Metric::Lcs(y) => (y, Box::new(|a,b | Dist::lcs_length(a,b))),
            Metric::DoubleMetaphone(y,x) => (y, Box::new(move |a,b | Dist::metaphone_distance(a,b,&x))),
        };
        Self { root:None,dist,distance_fxn}
    }

    pub fn insert(&mut self,id:usize) {
        match &mut self.root{
            Some(x) => x.insert(id,&self.distance_fxn),
            None => self.root = Some(O_BK_Node::new(id))
        }
    }

    pub fn from_vec(ids:Vec<usize>) -> O_BK_Tree {
        let mut root = O_BK_Tree::new(Metric::Levenshtein(D_THRESHOLD));
        ids.iter().for_each( |x| root.insert(*x) );
        root
    }

    pub fn get_n_ids(&self,word: &str) -> Vec<usize> {
        let mut ret = Vec::new();
        if let Some(root) = &self.root {
            root.get_n_ids(word, self.dist, &mut ret,&self.distance_fxn);
        }
        ret
    }
    pub fn get_n_strs(&self,word: &str) -> Vec<String> {
        if let Some(root) = &self.root {
            return root.get_n_strs(word, self.dist,&self.distance_fxn);
        }
        vec![]
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

    pub fn insert(&mut self,id:usize,d_func: & Box<dyn Fn(&str,&str) -> usize>) {
        let dist = d_func(&get_str(self.id).unwrap(),&get_str(id).unwrap());
        match self.children.get_mut(&dist) {
            Some(x) => {x.insert(id,d_func);},
            None => {self.children.insert(dist, O_BK_Node::new(id));}
        }
    }
    pub fn get_n_ids(&self,word:&str,dist:usize,ret:&mut Vec<usize>,d_func: & Box<dyn Fn(&str,&str) -> usize>){
        let d = d_func(&get_str(self.id).unwrap()[..],word);
        if d <= dist{
            ret.push(self.id);
        }
        let (mn,mx) = (d.saturating_sub(dist),d.saturating_add(dist));
         for (&e_dist,child) in &self.children{
            if e_dist >= mn && e_dist <= mx{
                child.get_n_ids(word, dist,ret,d_func);
            }

         }
    }

    pub fn get_n_strs(&self,word:&str,dist:usize,d_func: & Box<dyn Fn(&str,&str) -> usize>) -> Vec<String>{
        let mut ret = Vec::new();
        self.get_n_ids(word, dist, &mut ret,d_func);
        ret.iter().map(|&x| get_str(x).unwrap_or("".to_string()) ).collect::<Vec<String>>()
    }
 

   
}





}