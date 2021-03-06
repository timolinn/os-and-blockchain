#![allow(dead_code)]
extern crate rand;
extern crate nanoid;
extern crate hex;

mod goofycoin;

use goofycoin::{Person, GoofyCoin};

fn main() {
    let goofy = Person::new("goofy");
    let mut coin = GoofyCoin::new(goofy.clone());
    println!("name={:?}\nPublic Key={:?}\nsignature={:?}", goofy.name(), goofy.pk, coin.signature());

    println!("{}", coin.verify(&goofy));

    let alice = Person::new("Alice");
    println!("{}", coin.verify(&alice));

    // let res = match goofy.transfer_coin(coin, &alice) {
    //     Ok(c) => c,
    //     Err(e) => e
    // }
    coin = goofy.transfer_coin(coin, &alice).unwrap();
    println!("{}", coin.verify(&goofy));
    println!("{}", coin.verify(&alice));
}
