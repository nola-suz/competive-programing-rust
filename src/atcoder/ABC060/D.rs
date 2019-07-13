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

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn solve() {
    input!(n: usize, m: usize, ob: [[usize; 2]; n],);
    let mut wv: Vec<Vec<i64>> = Vec::new();
    wv.resize(4, Vec::new());
    let weight = ob[0][0];
    for o in ob {
        wv[o[0] - weight].push(o[1] as i64);
    }
    for i in 0..4 {
        wv[i].sort();
        wv[i].reverse();
        if i < 3 {
            wv[i].insert(0, 0);
        }
    }

    let mut ans = 0i64;
    let mut tmp1 = 0i64;
    for i in 0..wv[0].len() {
        tmp1 += wv[0][i];
        let mut tmp2 = tmp1;
        for j in 0..wv[1].len() {
            tmp2 += wv[1][j];
            let mut tmp3 = tmp2;
            for k in 0..wv[2].len() {
                tmp3 += wv[2][k];
                let mut tmp4 = tmp3;
                let mut w2 = weight * (i + j + k) + j + 2 * k;
                for l in 0..wv[3].len() {
                    if w2 + weight + 3 > m {
                        break;
                    }
                    w2 += weight + 3;
                    tmp4 += wv[3][l];
                }
                if w2 <= m {
                    ans = max(ans, tmp4);
                }
            }
        }
    }
    println!("{}", ans);
}

fn main() {
    let stack_size = 1 << 28;
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
