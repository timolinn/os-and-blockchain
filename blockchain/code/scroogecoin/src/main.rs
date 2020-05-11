
mod scroogecoin;
use scroogecoin::Scroogecoin;
fn main() {
    let scrooge  = Scroogecoin::new();
    println!("Hello, {:?}", scrooge);
}
