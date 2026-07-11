// -*- coding: utf-8-unix -*-
#![allow(dead_code, unused_imports, unused_macros)]

use std::cmp::{max, min, Ordering, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::fmt::Debug;
use std::io::{self, Read};
use std::str::FromStr;

const INF_I32: i32 = 1_i32 << 30;
const INF_I64: i64 = 1_i64 << 60;
const INF_I128: i128 = 1_i128 << 120;
const INF_USIZE: usize = usize::MAX / 4;

const INF: i64 = INF_I64;
const UINF: usize = INF_USIZE;
const INF128: i128 = INF_I128;

const MOD_1E9_7: i64 = 1_000_000_007;
const MOD_998: i64 = 998_244_353;
const MOD: i64 = MOD_998;
const UMOD: usize = MOD as usize;

// =============================================================================
// Input
// =========================================================================
#[allow(dead_code)]
fn read<T: std::str::FromStr>() -> T {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

#[allow(dead_code)]
fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read::<String>()
        .split_whitespace()
        .map(|e| e.parse().ok().unwrap())
        .collect()
}

#[allow(dead_code)]
fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}
macro_rules! ru {
    ($($x:ident),+ $(,)?) => {
        let values = read_vec::<usize>();
        let mut iter = values.into_iter();
        $(
            let mut $x = iter
                .next()
                .expect("入力の要素数が不足しています");
        )+
    };
}

macro_rules! ri {
    ($($x:ident),+ $(,)?) => {
        let values = read_vec::<i64>();
        let mut iter = values.into_iter();
        $(
            let mut $x = iter
                .next()
                .expect("入力の要素数が不足しています");
        )+
    };
}
macro_rules! rc {
    ($x:ident $(,)?) => {
        let mut $x: Vec<char> = read::<String>().chars().collect();
    };
}
// =============================================================================
// Output / Debug
// =============================================================================

macro_rules! p {
    () => {
        println!();
    };
    ($value:expr $(,)?) => {
        println!("{}", $value);
    };
    ($fmt:literal, $($arg:tt)*) => {
        println!($fmt, $($arg)*);
    };
}

macro_rules! join {
    ($iter:expr, $separator:expr $(,)?) => {
        ($iter)
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join($separator)
    };
}

macro_rules! pv {
    ($iter:expr $(,)?) => {
        println!("{}", join!($iter, " "));
    };
    ($iter:expr, $separator:expr $(,)?) => {
        println!("{}", join!($iter, $separator));
    };
}

macro_rules! yesno {
    ($condition:expr $(,)?) => {
        println!("{}", if $condition { "Yes" } else { "No" });
    };
    ($condition:expr, $yes:expr, $no:expr $(,)?) => {
        println!("{}", if $condition { $yes } else { $no });
    };
}

macro_rules! debug {
    ($($value:expr),+ $(,)?) => {
        #[cfg(debug_assertions)]
        {
            eprint!("[{}:{}]", file!(), line!());
            $(
                eprint!(" {} = {:?}", stringify!($value), &$value);
            )+
            eprintln!();
        }
    };
}

// =============================================================================
// Scalar / Vec utilities
// =============================================================================

macro_rules! chmin {
    ($base:expr, $($value:expr),+ $(,)?) => {{
        let mut updated = false;
        $(
            let value = $value;
            if $base > value {
                $base = value;
                updated = true;
            }
        )+
        updated
    }};
}

macro_rules! chmax {
    ($base:expr, $($value:expr),+ $(,)?) => {{
        let mut updated = false;
        $(
            let value = $value;
            if $base < value {
                $base = value;
                updated = true;
            }
        )+
        updated
    }};
}
macro_rules! min {
    // iterable: Vec、配列、スライス、イテレータなど
    ($iter:expr $(,)?) => {{
        ($iter)
            .into_iter()
            .min()
            .expect("min! called on an empty iterator")
    }};

    // 複数の値
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let mut answer = $first;
        $(
            chmin!(answer, $rest);
        )+
        answer
    }};
}

macro_rules! max {
    // iterable: Vec、配列、スライス、イテレータなど
    ($iter:expr $(,)?) => {{
        ($iter)
            .into_iter()
            .max()
            .expect("max! called on an empty iterator")
    }};

    // 複数の値
    ($first:expr, $($rest:expr),+ $(,)?) => {{
        let mut answer = $first;
        $(
            chmax!(answer, $rest);
        )+
        answer
    }};
}

macro_rules! ndvec {
    ($value:expr; $len:expr) => {
        vec![$value; $len]
    };
    ($value:expr; $len:expr, $($rest:expr),+ $(,)?) => {
        vec![ndvec![$value; $($rest),+]; $len]
    };
}
macro_rules! for_nd {
    // 再帰の終端
    ($body:block $(,)?) => {
        $body
    };

    // ループを1段生成し、残りを再帰的に展開
    (
        $variable:ident in $range:expr,
        $($rest:tt)+
    ) => {
        for $variable in $range {
            for_nd!($($rest)+);
        }
    };
}

macro_rules! for_set {
    ($value:pat in $set:expr => $body:block) => {
        for $value in &$set {
            $body
        }
    };
}

macro_rules! for_map {
    ($key:pat, $value:pat in $map:expr => $body:block) => {
        for ($key, $value) in &$map {
            $body
        }
    };
}

macro_rules! sorted {
    ($iter:expr $(,)?) => {{
        let mut values = ($iter).into_iter().collect::<Vec<_>>();
        values.sort_unstable();
        values
    }};
}

macro_rules! sorted_unique {
    ($iter:expr $(,)?) => {{
        let mut values = sorted!($iter);
        values.dedup();
        values
    }};
}

macro_rules! sort_desc {
    ($values:expr $(,)?) => {{
        ($values).sort_unstable_by(|a, b| b.cmp(a));
    }};
}

macro_rules! prefix_sum {
    ($values:expr $(,)?) => {{
        let values = &$values;
        let mut prefix = Vec::with_capacity(values.len() + 1);
        prefix.push(Default::default());
        for &value in values.iter() {
            let next = prefix.last().copied().unwrap() + value;
            prefix.push(next);
        }
        prefix
    }};
}

macro_rules! prefix_xor {
    ($values:expr $(,)?) => {{
        let values = &$values;
        let mut prefix = Vec::with_capacity(values.len() + 1);
        prefix.push(Default::default());
        for &value in values.iter() {
            let next = prefix.last().copied().unwrap() ^ value;
            prefix.push(next);
        }
        prefix
    }};
}

// 数学的な floor(a / b)。b != 0。
macro_rules! div_floor {
    ($a:expr, $b:expr $(,)?) => {{
        let a = $a;
        let b = $b;
        let q = a / b;
        let r = a % b;
        if r != 0 && ((r > 0) != (b > 0)) {
            q - 1
        } else {
            q
        }
    }};
}

// 数学的な ceil(a / b)。b != 0。
macro_rules! div_ceil {
    ($a:expr, $b:expr $(,)?) => {{
        let a = $a;
        let b = $b;
        let q = a / b;
        let r = a % b;
        if r != 0 && ((r > 0) == (b > 0)) {
            q + 1
        } else {
            q
        }
    }};
}

macro_rules! has_bit {
    ($mask:expr, $bit:expr $(,)?) => {
        (($mask >> $bit) & 1) != 0
    };
}

macro_rules! set_bit {
    ($mask:expr, $bit:expr $(,)?) => {
        $mask |= 1 << $bit
    };
}

macro_rules! clear_bit {
    ($mask:expr, $bit:expr $(,)?) => {
        $mask &= !(1 << $bit)
    };
}

macro_rules! toggle_bit {
    ($mask:expr, $bit:expr $(,)?) => {
        $mask ^= 1 << $bit
    };
}

// =============================================================================
// Integer / floating-point binary search
// =============================================================================

// pred(ok) == true, pred(ng) == false を保ち、最後に true 側の境界を返す。
// ok < ng と ok > ng の双方に対応する。
macro_rules! binsearch {
    (ok = $ok:expr, ng = $ng:expr, $pred:expr $(,)?) => {{
        let mut ok = $ok;
        let mut ng = $ng;
        let mut pred = $pred;

        while ok.abs_diff(ng) > 1 {
            let mid = if ok < ng {
                ok + (ng - ok) / 2
            } else {
                ng + (ok - ng) / 2
            };

            if pred(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }};
}

// false ... true の単調列に対し、最初の true を返す。
// ng は false 側の番兵、ok は true 側の番兵。
macro_rules! first_true {
    ($ng:expr, $ok:expr, $pred:expr $(,)?) => {
        binsearch!(ok = $ok, ng = $ng, $pred)
    };
}

// true ... false の単調列に対し、最後の true を返す。
// ok は true 側の番兵、ng は false 側の番兵。
macro_rules! last_true {
    ($ok:expr, $ng:expr, $pred:expr $(,)?) => {
        binsearch!(ok = $ok, ng = $ng, $pred)
    };
}

// 浮動小数点版。回数を明示することで停止条件の曖昧さを避ける。
macro_rules! binsearch_f64 {
    (ok = $ok:expr, ng = $ng:expr, iter = $iter:expr, $pred:expr $(,)?) => {{
        let mut ok = $ok;
        let mut ng = $ng;
        let mut pred = $pred;

        for _ in 0..$iter {
            let mid = (ok + ng) * 0.5;
            if pred(mid) {
                ok = mid;
            } else {
                ng = mid;
            }
        }

        ok
    }};
}

// =============================================================================
// Sorted slice bounds
// =============================================================================

macro_rules! lower_bound {
    ($slice:expr, $value:expr $(,)?) => {{
        let value = $value;
        ($slice).partition_point(|x| x < &value)
    }};
}

macro_rules! upper_bound {
    ($slice:expr, $value:expr $(,)?) => {{
        let value = $value;
        ($slice).partition_point(|x| x <= &value)
    }};
}

macro_rules! equal_range {
    ($slice:expr, $value:expr $(,)?) => {{
        let slice = &$slice;
        let value = $value;
        let left = slice.partition_point(|x| x < &value);
        let right = slice.partition_point(|x| x <= &value);
        (left, right)
    }};
}

macro_rules! count_sorted {
    ($slice:expr, $value:expr $(,)?) => {{
        let (left, right) = equal_range!($slice, $value);
        right - left
    }};
}

macro_rules! lower_bound_by_key {
    ($slice:expr, $key:expr, $key_of:expr $(,)?) => {{
        let key = $key;
        let mut key_of = $key_of;
        ($slice).partition_point(|x| key_of(x) < key)
    }};
}

macro_rules! upper_bound_by_key {
    ($slice:expr, $key:expr, $key_of:expr $(,)?) => {{
        let key = $key;
        let mut key_of = $key_of;
        ($slice).partition_point(|x| key_of(x) <= key)
    }};
}
macro_rules! neighbors4 {
    ($y:expr, $x:expr, $h:expr, $w:expr $(,)?) => {{
        const DY: [isize; 4] = [-1, 0, 1, 0];
        const DX: [isize; 4] = [0, 1, 0, -1];

        (0..4).filter_map(move |dir| {
            let ny = $y as isize + DY[dir];
            let nx = $x as isize + DX[dir];

            if 0 <= ny && ny < $h as isize && 0 <= nx && nx < $w as isize {
                Some((ny as usize, nx as usize))
            } else {
                None
            }
        })
    }};
}
macro_rules! run_length {
    ($iter:expr $(,)?) => {{
        let mut result = Vec::new();

        for value in $iter {
            match result.last_mut() {
                Some((last, count)) if *last == value => {
                    *count += 1usize;
                }
                _ => {
                    result.push((value, 1usize));
                }
            }
        }

        result
    }};
}
// =============================================================================
// Map / Set literals and counters
// =============================================================================

macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = ::std::collections::BTreeMap::new();
        $(map.insert($key, $value);)*
        map
    }};
}

macro_rules! set {
    ($($value:expr),* $(,)?) => {{
        let mut set = ::std::collections::BTreeSet::new();
        $(set.insert($value);)*
        set
    }};
}

macro_rules! count_map {
    ($iter:expr $(,)?) => {{
        let mut count = BTreeMap::new();
        for value in $iter {
            *count.entry(value).or_insert(0usize) += 1;
        }
        count
    }};
}

macro_rules! map_count {
    ($map:expr, $key:expr $(,)?) => {
        ($map).get(&$key).copied().unwrap_or_default()
    };
}

macro_rules! map_add {
    ($map:expr, $key:expr, $delta:expr $(,)?) => {{
        let key = $key;
        let delta = $delta;
        let map = &mut $map;
        *map.entry(key).or_insert(0) += delta;
    }};
}

macro_rules! map_inc {
    ($map:expr, $key:expr $(,)?) => {
        map_add!($map, $key, 1)
    };
}

// delta を引いた後も正なら減算し、0 以下になるならキーごと削除する。
// キーを削除した場合のみ true。
macro_rules! map_sub {
    ($map:expr, $key:expr, $delta:expr $(,)?) => {{
        let key = $key;
        let delta = $delta;
        let map = &mut $map;

        let remove = match map.get_mut(&key) {
            Some(value) => {
                if *value <= delta {
                    true
                } else {
                    *value -= delta;
                    false
                }
            }
            None => false,
        };

        if remove {
            map.remove(&key);
            true
        } else {
            false
        }
    }};
}

macro_rules! map_dec {
    ($map:expr, $key:expr $(,)?) => {
        map_sub!($map, $key, 1)
    };
}

// 未登録なら挿入するため、その場合も true。
macro_rules! map_chmin {
    ($map:expr, $key:expr, $value:expr $(,)?) => {{
        let key = $key;
        let value = $value;
        let map = &mut $map;

        match map.get_mut(&key) {
            Some(current) if value < *current => {
                *current = value;
                true
            }
            Some(_) => false,
            None => {
                map.insert(key, value);
                true
            }
        }
    }};
}

// 未登録なら挿入するため、その場合も true。
macro_rules! map_chmax {
    ($map:expr, $key:expr, $value:expr $(,)?) => {{
        let key = $key;
        let value = $value;
        let map = &mut $map;

        match map.get_mut(&key) {
            Some(current) if value > *current => {
                *current = value;
                true
            }
            Some(_) => false,
            None => {
                map.insert(key, value);
                true
            }
        }
    }};
}

macro_rules! set_toggle {
    ($set:expr, $value:expr $(,)?) => {{
        let value = $value;
        let set = &mut $set;
        if set.remove(&value) {
            false
        } else {
            set.insert(value);
            true
        }
    }};
}

// =============================================================================
// Ordered map / set neighbor queries
// =============================================================================

macro_rules! first_ge {
    ($ordered:expr, $value:expr $(,)?) => {
        ($ordered).range($value..).next()
    };
}

macro_rules! first_gt {
    ($ordered:expr, $value:expr $(,)?) => {
        ($ordered)
            .range((
                std::ops::Bound::Excluded($value),
                std::ops::Bound::Unbounded,
            ))
            .next()
    };
}

macro_rules! last_le {
    ($ordered:expr, $value:expr $(,)?) => {
        ($ordered).range(..=$value).next_back()
    };
}

macro_rules! last_lt {
    ($ordered:expr, $value:expr $(,)?) => {
        ($ordered).range(..$value).next_back()
    };
}

macro_rules! u {
    ($($x:ident),+ $(,)?) => {
        $(
            let mut $x = 0usize;
        )+
    };
}

macro_rules! i {
    ($($x:ident),+ $(,)?) => {
        $(
            let mut $x = 0i64;
        )+
    };
}

fn main() {}
