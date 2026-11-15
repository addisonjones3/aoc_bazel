use rust_2025::dial;

pub(crate) fn main() {
    let d = dial::Dial::new(50, 100);
    println!("{}", d)
}
