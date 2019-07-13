#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::{BufWriter, Write};
use std::string::ToString;

// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
#[allow(unused_macros)]
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

#[allow(unused_macros)]
macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, bytes) => {
        read_value!($next, String).into_bytes()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn check(s: &str) -> bool {
    let rexes = vec![
        "AAGC", "GAGC", "CAGC", "TAGC", "AGCA", "AGCG", "AGCC", "AGCT", "AAGC", "AGGC", "ACGC",
        "ATGC", "AGAC", "AGGC", "AGCC", "AGTC", "AGAC", "GGAC", "CGAC", "TGAC", "GACA", "GACG",
        "GACC", "GACT", "AACG", "GACG", "CACG", "TACG", "ACGA", "ACGG", "ACGC", "ACGT",
    ];

    return !rexes.contains(&s);
}

fn solve() {
    input!(n: usize,);

    let m = 1000000007;
    let mut map = HashMap::new();
    map.insert("TTTT".to_string(), 1);
    for _i in 0..n {
        let mut n_map: HashMap<String, i32> = HashMap::new();
        for (k, v) in &map {
            for c in "ACGT".chars() {
                let s = format!("{}{}", &k, &c)[1..5].to_string();
                if check(&s) {
                    if n_map.get(&s) == None {
                        n_map.insert(s.to_string(), 0);
                    }
                    *n_map.get_mut(&s).unwrap() = (n_map[&s] + v) % m;
                }
            }
        }
        map = n_map;
    }

    let mut ans = 0;

    for (k, v) in &map {
        if check(k) {
            ans = (ans + v) % m;
        }
    }
    println!("{}", ans);
}

fn main() {
    let stack_size = 1 << 28;
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        assert!(!check("AGCA"));
        assert!(check("AAAA"));
    }
}
