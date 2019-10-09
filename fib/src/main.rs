use std::collections::hash_map::*;
extern crate num;

use num::bigint::BigUint;
use num::{Zero, One};
use std::env;
use std::rc::Rc;
use std::time::{Instant};
use num::pow::pow;

fn fib(n: u64, map: &mut HashMap<u64, Rc<BigUint>>) -> Rc<BigUint> {

    if let Some(r) = map.get(&n) {
        Rc::clone(r)
    } else {
        let _res:BigUint = if n % 2 == 0 {
            let m_1_rc = fib(n  / 2 - 1, map);
            let m_1 = m_1_rc.as_ref();
            let a_1_rc = fib(n  / 2 + 1, map);
            let a_1 = a_1_rc.as_ref();
            let m_rc = fib(n  / 2 , map);
            let m = m_rc.as_ref();
            (m_1  + a_1)*m
        } else {
            let m_1_rc = fib(n  / 2 - 1, map);
            let m_1 = m_1_rc.as_ref();
            let a_1_rc = fib(n  / 2 + 1, map);
            let a_1 = a_1_rc.as_ref();
            m_1 * m_1 + a_1 * a_1
        };
        let res = Rc::new(_res);
        map.insert(n, Rc::clone(&res));
        res
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target: i32 = args[1].parse().unwrap();
    assert!(target > 0);

    let mut map = HashMap::new();
    map.insert(0, Rc::new(Zero::zero()));
    map.insert(1, Rc::new(One::one()));
    map.insert(2, Rc::new(One::one()));
    let start = Instant::now();
    let res = fib(target as u64, &mut map);
    let duration = start.elapsed();

    println!("Finished calculation in {:?}",duration);

//    println!("The {:?}th fibonacci number is {:?}", target,res.to_str_radix(10));
}


