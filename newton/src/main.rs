extern crate num;

use num::rational::BigRational;
use num::bigint::{BigInt};
use std::env;
use num::pow::pow;


fn main() {
    let args: Vec<String> = env::args().collect();
    let _target: i32 = args[1].parse().unwrap();
    assert!(_target > 0);
    let _precision:usize = args[2].parse().unwrap();
    let mut precision = 2;
    assert!(precision > 0);
    let iter_times = (_precision as f32).log2() as usize + 3;
    let mut approximate = 1;
    loop {
        let power = pow(approximate, 2);
        if power == _target {
            println!("{}",approximate);
            return;
        } else if power > _target {
            break;
        } else {
            approximate += 1;
        }
    }
    approximate -= 1;
    let mut pre = pow(BigInt::from(10_i32),precision);
    let target = BigRational::new(BigInt::from(_target), BigInt::from(1_i32));
    let two = BigRational::new(BigInt::from(2_i32), BigInt::from(1_i32));
    let mut x = BigRational::new(BigInt::from(approximate+1), BigInt::from(1_i32));
    print!("{}.",approximate);
    let mut prev = vec!['0';precision];
    let mut index:usize = 0;
    for i in 0..iter_times{
        /*
        Newton iteration approach
        Calculate the root of formula y = x^2 - target;
        Iterate using x1 = x0 - f(x0)/f'(x0)
        Which is x1 = 2x0 - target* x0^2 / 2 * x0
        */
        x = &(&x +  &target / &x) / &two;
        let cur = (&x.fract() * &pre).to_integer().to_str_radix(10).chars().collect::<Vec<char>>();
        while index < _precision && index < prev.len() && index < cur.len(){
            if prev[index] == cur[index]{
                print!("{}",cur[index]);
                index += 1;
            } else {
                prev = cur;
                break;
            }
        }
        if precision < _precision{
            pre = pow(pre,2);
            precision *= 2;
            // println!("{} {}",precision, _precision);

        }
    }
}


