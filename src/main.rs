mod collatz;
use collatz::Collatz;

fn main() {
    let c = Collatz::new(100);
    for i in c {
        print!("{} ", i);
    }
    println!("");
}
