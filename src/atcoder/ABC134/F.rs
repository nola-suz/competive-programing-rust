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

fn add(a: &mut usize, b: &usize) -> () {
    *a += *b;
    *a %= 1000000007;
}

fn solve() {
    input!(
        n: usize,
        _k: usize,
    );

    let mut dp = vec![vec![vec![0usize; 2525]; 51]; 51];

    dp[0][0][0] = 1;


    for i in 0..50 {
        for j in 0..51 {
            for k in 0..2525 {
                if k + 2*j >= 2525 {
                    continue;
                }
                let tmp = dp[i][j][k] * j;
                add(&mut dp[i+1][j][k+2*j], &tmp);
                if j as i32 -1 >= 0 {
                    let tmp = dp[i][j][k] * j * j;
                    add(&mut dp[i+1][j-1][k+2*j], &tmp);
                }
                if j+1 < 51 {
                    let tmp = dp[i][j][k];
                    add(&mut dp[i+1][j+1][k+2*j], &tmp);
                }
                let tmp = dp[i][j][k] * j;
                add(&mut dp[i+1][j][k+2*j], &tmp);
                let tmp = dp[i][j][k];
                add(&mut dp[i+1][j][k+2*j], &tmp);
            }
        }
    }
    println!("{}", dp[n][0][_k]);
}

fn main() {
    let stack_size = 1 << 28;
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
