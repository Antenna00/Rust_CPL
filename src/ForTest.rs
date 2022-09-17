pub fn largest<T:>(list: &[T]) -> T {
    let mut largest = list[0]; //この時点で型が確定していないとおそらくスコープ外になる

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    //number list の場合
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list); //ここで借用
    println!("The largest number is {}", result);

    //char listの場合
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}