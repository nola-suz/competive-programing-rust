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

fn greed(a: i32, b: i32, mut c: i32) -> i32 {
    let mut ans = 0;
    let mut tmp = c / b;
    ans += tmp;
    c -= b * tmp;

    let tmp = c / a;
    ans += tmp;
    c -= a * tmp;

    ans + c
}

fn opt(a: i32, b: i32, mut c: i32) -> i32 {
    let mut ans = c / a + c % a;
    let mut tmp = 0;

    while c >= b {
        c -= b;
        tmp += 1;
        ans = min(ans, tmp + c / a + c % a);
    }
    ans
}

fn f(a: i32, b: i32) -> i32 {
    for i in 2..a*b+100 {
        let g = greed(a, b, i);
        let o = opt(a, b, i);
        if g != o {
            return i;
        }
    }
    -1
}

fn hoge(a: i64, b: i64) -> i64 {
    for i in 2..a+1 {
        if a - i <= 0 {
            break;
        }
        let l = (i-1) * a + 1;
        let r = l + a - i;
        if l <= b && b <= r {
            return a as i64 * i as i64;
        }
    }
    -1
}

fn solve() {
    input!(
        a: i64,
        b: i64,
    );

    println!("{}", hoge(a, b));
}

fn main() {
    let stack_size = 1 << 28;
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
