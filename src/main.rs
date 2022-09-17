#![allow(non_snake_case)]
//mod ForTest;
mod Math;
//mod Algorithm;
use rand::Rng;

fn main() {
    //number list の場合
    // let number_list = vec![34, 50, 25, 100, 65];
    // let result = ForTest::largest(&number_list); //ここで借用
    // println!("The largest number is {}", result);

    // //char listの場合
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = ForTest::largest(&char_list);
    // println!("The largest char is {}", result);


    let s = Math::prime3(10);
    println!("{:?}", s);

    let list = Math::sieve(10);
    println!("{:?}", list);

    let ss: f32 = rand::thread_rng().gen();
    println!("{}", ss);

    println!("{}", Math::montecarlo(10000));
}