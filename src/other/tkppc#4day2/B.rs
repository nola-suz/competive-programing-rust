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


// https://qiita.com/SatoshiTerasaki/items/2e615af7374d7d3bc192
fn xorshift32() -> Box<FnMut() -> u32> {
    let mut y = 2463534242 ;
    Box::new(move || {
        y = y ^ (y << 13);
        y = y ^ (y >> 17);
        y = y ^ (y << 5);
        y
    })
}

fn dfs(x: usize, p: usize, p_cnt: usize, edge: &Vec<Vec<usize>>, c: &Vec<usize>, ok: &mut bool) -> usize {
    let mut t_cnt = 0;
    let n_cnt = if p_cnt + c[x] > 0 { 1 } else { 0 };

    for y in &edge[x] {
        if *y == p {
            continue;
        }

        t_cnt += dfs(*y, x, n_cnt, edge, c, ok);
    }

    if t_cnt + p_cnt >= 3 {
        *ok = false;
    }

    return if t_cnt + c[x] > 0 { 1 } else { 0 };
}

fn solve() {
    input!(
        n: usize,
        m: usize,
        ab: [[usize; 2]; n-1],
        _c: [usize; m],
    );

    let mut c = Vec::new();
    c.resize(n, 0);
    for cc in _c {
        c[cc-1] = 1;
    }

    let mut edge = Vec::new();
    edge.resize(n, Vec::new());
    for e in ab {
        let a = e[0];
        let b = e[1];
        edge[a-1].push(b-1);
        edge[b-1].push(a-1);
    }

    let mut ok = true;

    dfs(0, 1000000, 0, &edge, &c, &mut ok);

    let mut xor32 = xorshift32();

    for i in 0..30 {
        let start = xor32() as usize % n;
        dfs(start, 1000000, 0, &edge, &c, &mut ok);
    }

    println!("{}", if ok { "Yes" } else { "trumpet" } );
}

fn main() {
    let stack_size = 1 << 28;
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
