// use crate::add::add;

mod add;

fn main() {
    let c = a();
    match c {
        Some(o) => println!("{}", o),
        None => println!("hei"),
    }
}

fn a() -> Option<i32> {
    // println!("{}", add(1, 2));

    let a: Option<i32> = None;
    let b = a?;
    Some(b)
}
