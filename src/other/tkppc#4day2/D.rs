#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
#[allow(unused_imports)]
use std::io::{BufWriter, Write};

use std::iter::*;

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
        n: usize,
        p: i64,
        q: i64,
        a: [i64; n],
    );

    if (p + q) % 2 == 1 {
        println!("0");
        return;
    }

    let ax = (p + q) / 2;
    let target = p - ax;

    let mut sum: Vec<i64> = Vec::new();
    sum.resize(n, 0);

    let mut a = a;
    a.reverse();

    let mut map = HashMap::new();

    for i in 0..n {
        {
            let t = map.get(&(target - a[i]));
            if t != None {
                sum[i] = *t.unwrap() as i64;
            }
        }
        if map.get(&a[i]) == None {
            map.insert(a[i], 0);
        }
        *map.get_mut(&a[i]).unwrap() = map[&a[i]] + 1;
    }

    for i in 1..n {
        sum[i] += sum[i-1];
    }

    let mut ans = 0;

    for i in 0..n-1 {
        if ax != a[n-1-i] {
            continue;
        }
        ans += sum[n-1 - i - 1];
    }
    println!("{}", ans);
}

fn main() {
    let stack_size = 1 << 28;
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
