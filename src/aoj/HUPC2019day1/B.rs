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

fn pro(x: i32) -> i64 {
    let mut ret = 1i64;
    for i in 2..x+1 {
        if i * i > x || ret > 1i64<<32 {
            return ret;
        }
        if x % i == 0 {
            ret *= i as i64;
            if i * i != x {
                ret *= (x / i) as i64;
            }
        }
    }
    0
}

fn test(x: i32) -> i32 {
    let mut ans = 0;
    for i in 2..x+1 {
        if pro(i) >= 2 * i as i64 {
            ans += 1;
        }
    }
    ans
}

fn solve() {
    input!(
        q: usize,
        n: [usize; q],
    );

    let mut rui = Vec::new();
    let si = 200010usize;
    rui.resize(si, 0i32);
    for i in 2..si {
        let p = pro(i as i32);
        if p < 2 * i as i64 {
            continue;
        }
        rui[i] += 1;
    }

    for i in 1..si {
        rui[i] += rui[i-1];
    }

    for _q in n {
//        if rui[_q] != test(_q as i32) {
//            println!("assert {}: {} != {}", _q, rui[_q], test(_q as i32));
//        }
        println!("{}", rui[_q]);
    }

//    for i in 2..100 {
//        println!("{}: {}", i, pro(i));
//    }
}

fn main() {
    let stack_size = 1 << 28;
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
