fn main() {
    let n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let mut count = vec![0usize; (n + 1) as usize];
    count[0] = 1;

    let modulus = 10usize.pow(9)+7;
    for x in 1..n + 1 {
        for o in 1..7 {
            if x - o >= 0 {
                count[x as usize] += count[(x - o) as usize] % modulus;
            }
        }
    }

    println!("{}", count[n as usize] % modulus);
}
