use std::io;
//sum of first n natural numbers - sum of given numbers
fn main() {
    let n = io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let sum = io::stdin()  
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(|i| i.parse::<usize>().unwrap())
        .sum::<usize>();

    let n_sum = n*(n+1)/2;
    let missing_number=  n_sum-sum;
    println!("{missing_number}");
}
