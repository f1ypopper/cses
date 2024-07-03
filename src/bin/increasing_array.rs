use std::io::stdin;
fn main() {
    stdin().lines().next();
    let mut array = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let mut moves = 0;
    for i in 0..array.len() - 1 {
    }
    println!("{moves}");
}
