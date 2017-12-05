extern crate crypto;

use crypto::digest::Digest;
use crypto::sha2::Sha256;
use std::io::{stdin,stdout,Write};


fn main() {

    let mut s=String::new();
    let mut sha = Sha256::new();

    print!("Enter input: ");
    let _= stdout().flush();

    stdin().read_line(&mut s).expect("Did not enter a correct string");
    let input = s.trim();//.parse().unwrap();

    sha.input_str(input);

    let hash: &str = &sha.result_str();

    println!("Hash: {}", hash);



    let z = i64::from_str_radix(&hash[..15], 16);
    let y = i64::from_str_radix(&hash[15..30], 16);
    let x = i64::from_str_radix(&hash[30..45], 16);
    let w = i64::from_str_radix(&hash[45..60], 16);
    let v = i64::from_str_radix(&hash[60..64], 16);

    println!("{:?}{:?}{:?}{:?}{:?}", z,y,x,w,v);

    // println!("Hash: {}", a);

}
