use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let r: f64 = input.trim().parse().unwrap();

    let area = 3.14159 * r * r;

    println!("A={:.4}", area);
}
