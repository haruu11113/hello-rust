use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;


fn getLowest(str_dic: HashMap<&char, usize>, k: usize) -> (char, usize) {
    // mut s: str_dicのKeyの配列を並べ替えた最初の文字
    let mut s = &str_dic.keys().collect().first();
    // mut n: str_dicのうちvalueがsのもののkey
    let mut n = str_dic.iter().filter(|&(k, v)| k == s);
    return (&s, n);
}

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: String,
    }

    // mut s: 文字列のベクトル
    let mut s: Vec<char> = s.chars().collect();
    let mut s_dic = {};
    let mut s_dic: HashMap<&char, usize>= HashMap::new();


    for (n, i) in s.iter().enumerate() {
        s_dic.insert(i, &n);
    }
    getLowest(s_dic, 0);
    println!("");
}
