#![allow(dead_code)]
extern crate rand;
extern crate nanoid;
extern crate hex;

mod goofycoin;

fn main() {
    let goofy = goofycoin::Person::new("goofy");
    println!("name={:}\nPublic Key={:?}\nSecret Key={:?}", goofy.name(), goofy.public_key(), goofy.secret_key());
}
