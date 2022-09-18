use std::{
    fmt::{Debug, Display},
    mem::size_of,
    num::ParseIntError,
};

use num::{integer::Roots, iter::RangeInclusive, traits::AsPrimitive, PrimInt, ToPrimitive};
use rand::Rng;

//素数判定 3種、書き方違うだけで大体一緒。たぶん3つ目のが一番効率はいい。
pub fn prime<T: Into<u64>>(t: T) -> bool {
    let val = t.into();
    for i in 2..val {
        if val % i == 0 {
            return false;
        }

        if i * i >= val {
            break;
        }
    }
    true
}

pub fn prime2<T>(val: T) -> bool
where
    T: num::PrimInt + num::integer::Roots,
{
    let two = T::one() + T::one();
    num::range_inclusive(two, val.sqrt()).all(|i| val % i != T::zero())
}

pub fn prime3(val: usize) -> bool {
    num::range_inclusive(2, val.sqrt()).all(|i| val % i != 0)
}

//エラトステネスのふるい
pub fn sieve(val: usize) -> Vec<usize> {
    let mut primeList: Vec<usize> = vec![];
    let mut bTbl = vec![true; val];

    for i in 2..val {
        if (i * i >= val) {
            break;
        }
        if bTbl[i] {
            let mut x = 2 * i;
            while x < val {
                bTbl[x] = false;
                x += i;
            }
        }
    }

    for i in 2..val {
        if bTbl[i] {
            primeList.push(i);
        }
    }
    primeList
}

//モンテカルロ法　試行回数によってpiが出る
pub fn montecarlo(val: usize) -> f64 {
    let mut count: usize = 0;

    for i in 0..val {
        //このように型を指定してもいいが、普通に左辺値で設定してもよい。
        let px = rand::thread_rng().gen::<f64>();
        let py = rand::thread_rng().gen::<f64>();

        if (px * px + py * py <= 1.0) {
            count += 1;
        }
    }

    let pi = 4.0 * count as f64 / val as f64;
    pi
}
