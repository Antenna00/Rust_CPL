use std::mem::size_of;

use num::{ToPrimitive, traits::AsPrimitive};


pub fn prime<T>(val: T) -> bool 
    where T: ToPrimitive 
{
    let val = val.to_i64().unwrap();
    for i in 2..=val {
        if val % i == 0 {
            return false;
        }
        
        if i*i >= val {
            break;
        }
    }
    true
} 

fn main() {
    let v: bool = prime(444444444444444);
    println!("{}", v);
}