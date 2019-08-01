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
        n: usize,
        m: usize,
        uv: [[usize1; 2]; m],
        s: usize1,
        t: usize1,
    );

    let mut edge = Vec::new();
    edge.resize(n, Vec::new());

    for e in &uv {
        edge[e[0]].push(e[1]);
    }

    let mut vis = Vec::new();
    vis.resize(n, Vec::new());
    for i in 0..n {
        vis[i].resize(3, 0);
    }

    let mut qu: VecDeque<usize> = VecDeque::new();
    qu.push_back(s);

    vis[s][2] = 0;

    for cnt in 2..3*n+4 {
        if qu.len() == 0 {
            break;
        }
        let mut n_qu: VecDeque<usize> = VecDeque::new();
        let q_size = qu.len();
        for _ in 0..q_size {
            let front = qu.pop_front().unwrap();
            for x in &edge[front] {
                if vis[*x][(cnt+1)%3] > 0 {
                    continue;
                  }

                vis[*x][(cnt+1)%3] = cnt+1;

                n_qu.push_back(*x);
            }
        }
        qu = n_qu;
    }

    println!("{}", if vis[t][2] == 0 { -1 } else { vis[t][2] as i32 / 3 });
}

fn main() {
    let stack_size = 1 << 28;
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
