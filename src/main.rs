use std::fs::File;
use std::io::prelude::*;

mod store;

fn main() {
    let x = store::encoder::enc_string(String::from("Hello World"));
    println!("{:?}", x);

    let mut file = File::create("ralph.db").unwrap();

    file.write_all(&x).unwrap();
}
