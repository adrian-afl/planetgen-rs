use glam::{DVec4};
use std::cmp::Ordering;

fn main() {
    let a = DVec4::new(1.0, 2.0, 3.0, 4.0);
    let b = DVec4::new(3.0, 3.0, 33.0, 3.0);

    match a.x.partial_cmp(&b.x) {
        None => panic!("nanananana"),
        Some(x) => match x {
            Ordering::Less => println!("actually less"),
            Ordering::Equal => println!("actually more!"),
            Ordering::Greater => println!(" uh oh eq")
        }
    }
    println!("{}", a + b);
}
