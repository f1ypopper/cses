use std::io::stdin;
fn main() {
    stdin().lines().next();
    let mut array = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let mut moves: usize = 0;
    for i in 0..array.len() - 1 {
        if array[i] > array[i+1]{
            moves+=array[i]-array[i+1];
            array[i+1]+=array[i]-array[i+1];
        }
    }
    println!("{moves}");
}
