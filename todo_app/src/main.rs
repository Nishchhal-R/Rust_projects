use std::env;

fn main() {
    let  arguments: Vec<String>  = env::args().collect();
    let command = arguments[1].clone();

    println!("{}",command);

    if command== "get"{
        println!("we got a get")
    }

}