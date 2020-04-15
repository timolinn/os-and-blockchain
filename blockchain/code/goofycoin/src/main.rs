#![allow(dead_code)]
extern crate rand;
extern crate nanoid;
extern crate hex;

mod goofycoin;

fn main() {
    let goofy = goofycoin::Person::new("goofy");
    let coin = goofycoin::GoofyCoin::new(goofy.clone());
    println!("name={:?}\nPublic Key={:?}\nsignature=", goofy.name(), goofy.pk);
}
