use std::collections::hash_map::*;
extern crate num;

use num::bigint::BigUint;
use num::{Zero, One};
use std::env;
use std::rc::Rc;
use std::time::{Instant};
use num::pow::pow;

fn fib(n: u64, map: &mut HashMap<u64, Rc<BigUint>>) -> Rc<BigUint> {
    assert!(n > 0);
    if let Some(v) = map.get(&n) {
        return Rc::clone(v);
    } else {
        let res = if n % 2 == 0 {
            let m_1 = fib(n  / 2 - 1, map).as_ref().clone();
            let m = fib(n  / 2 , map).as_ref().clone();
            let a_1 = fib(n  / 2 + 1, map).as_ref().clone();
            Rc::new((m_1  + a_1)*m)

        } else {
            let m_1 = fib((n - 1) / 2, map).as_ref().clone();
            let a_1 = fib((n + 1) / 2, map).as_ref().clone();
            Rc::new(pow(m_1,2) + pow(a_1,2))
        };
        map.insert(n, Rc::clone(&res));
        return res;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target: i32 = args[1].parse().unwrap();
    let mut map = HashMap::new();
    map.insert(0, Rc::new(Zero::zero()));
    map.insert(1, Rc::new(One::one()));
    map.insert(2, Rc::new(One::one()));
    let start = Instant::now();
    let res = fib(target as u64, &mut map);
    let duration = start.elapsed();

    println!("Finished calculation in {:?}",duration);

    println!("The {:?}th fibonacci number is {:?}", target,res.to_str_radix(10));
}


