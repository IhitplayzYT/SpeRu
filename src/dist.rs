pub mod Dist{
use std::{cmp::{min},collections::HashMap};
use rphonetic::{DoubleMetaphone,Encoder};
use crate::Helper::DMeta;

pub fn damerau_levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let m = a.len();
    let n = b.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 0..=m {
        dp[i][0] = i;
    }

    for j in 0..=n {
        dp[0][j] = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };

            dp[i][j] = std::cmp::min(
                std::cmp::min(
                    dp[i - 1][j] + 1,
                    dp[i][j - 1] + 1,
                ),
                dp[i - 1][j - 1] + cost,
            );

            if i > 1
                && j > 1
                && a[i - 1] == b[j - 2]
                && a[i - 2] == b[j - 1]
            {
                dp[i][j] = dp[i][j].min(dp[i - 2][j - 2] + 1);
            }
        }
    }

    dp[m][n]
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

pub fn hamming(a: &str, b: &str) -> usize {
    if a.chars().count() != b.chars().count() {
        return usize::MAX;
    }
    a.chars().zip(b.chars()).filter(|(x, y)| x != y).count()
}

pub fn lcs_length(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let m = a.len();
    let n = b.len();

    let mut dp = vec![vec![0; n + 1]; m + 1];

    for i in 1..=m {
        for j in 1..=n {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[m][n]
}


pub fn qgram_distance(a: &str, b: &str, q: usize) -> usize {
    fn counts(s: &str, q: usize) -> HashMap<String, usize> {
        let chars: Vec<char> = s.chars().collect();
        let mut map = HashMap::new();

        if chars.len() < q {
            return map;
        }

        for i in 0..=chars.len() - q {
            let gram: String = chars[i..i + q].iter().collect();
            *map.entry(gram).or_insert(0) += 1;
        }

        map
    }

    let a_counts = counts(a, q);
    let b_counts = counts(b, q);

    let mut dist = 0;

    for (gram, count_a) in &a_counts {
        let count_b = b_counts.get(gram).copied().unwrap_or(0);

        dist += count_a.abs_diff(count_b);
    }

    for (gram, count_b) in &b_counts {
        if !a_counts.contains_key(gram) {
            dist += count_b;
        }
    }

    dist
}


pub fn metaphone_distance(a: &str, b: &str,c:& DMeta) -> usize {
    let dm = DoubleMetaphone::default();
    let (a_p,a_s)= (dm.encode(a),dm.encode_alternate(a));

    let (b_p,b_s) = (dm.encode(b),dm.encode_alternate(b));
        match c{
        DMeta::MIN => [Levenshtein(&a_p, &b_p),Levenshtein(&a_p, &b_s),Levenshtein(&a_s, &b_p),Levenshtein(&a_s, &b_s),].into_iter().min().unwrap(),
        DMeta::MAX => [Levenshtein(&a_p, &b_p),Levenshtein(&a_p, &b_s),Levenshtein(&a_s, &b_p),Levenshtein(&a_s, &b_s),].into_iter().max().unwrap(),
        DMeta::AVG => [Levenshtein(&a_p, &b_p),Levenshtein(&a_p, &b_s),Levenshtein(&a_s, &b_p),Levenshtein(&a_s, &b_s),].into_iter().sum::<usize>() / 4,
        }
}


}