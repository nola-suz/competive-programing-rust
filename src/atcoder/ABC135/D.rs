#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::{BufWriter, Write};

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

fn solve() {
    input!(
        s: chars,
    );
    let mut s: Vec<char> = s;
    s.reverse();
    let mut ans = Vec::new();
    ans.resize(13, 0i64);
    ans[0] = 1;

    let mut base = 1usize;

    for c in s {
        if c == '?' {
            let mut tmp = Vec::new();
            tmp.resize(13, 0i64);
            tmp.copy_from_slice(&ans[0..]);
            for j in 0..13 {
                ans[j] = 0;
            }
            for cc in 0..10usize {
                for j in 0..13 {
                    ans[(j + base * cc) % 13 as usize] += tmp[j];
                    ans[(j + base * cc) % 13 as usize] %= 1000000007;
                }
            }
        }
        else {
            let mut tmp = Vec::new();
            tmp.resize(13, 0i64);
            tmp.copy_from_slice(&ans[0..]);
            let cc = c as usize - '0' as usize;
            for j in 0..13 {
                ans[j] = 0;
            }
            for j in 0..13usize {
                ans[(j + base * cc) % 13] += tmp[j];
                ans[(j + base * cc) % 13] %= 1000000007;
            }
        }
        base *= 10;
        base %= 13;
    }
    println!("{}", ans[5] % 1000000007);
}

/*
??2??5
?44
7?4
?6?42???8??2??06243????9??3???7258??5??7???????774????4?1??17???9?5?70???76???
*/

fn main() {
    let stack_size = 1 << 28;
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
