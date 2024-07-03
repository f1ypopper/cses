fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    if n == 2 || n == 3 {
        println!("NO SOLUTION");
        return;
    }
    for i in (2..n + 1).step_by(2) {
        print!("{i} ");
    }
    for i in (1..n + 1).step_by(2) {
        print!("{i} ");
    }

    println!()
}
