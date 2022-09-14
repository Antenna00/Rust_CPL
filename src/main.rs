#![allow(non_snake_case)]

use crate::Math::prime;
mod Math;
mod Algorithm;
fn main() {

    let s = String::from("hello");
    let d = String::from("world");

    let str = d + &s;
    println!("{}",str);
    
    let mut num = 10;

    // for i in (2..=num).step_by(2) {
    //     println!("{}", i);
    // }
        let o = 5;
    let t = o.to_string();
    let v: bool = Math::prime(444444444444444);
    println!("{}", v);


    let t: i32 = t.parse().unwrap();


}
