fn main() {
    let input = std::io::stdin().lines().next().unwrap().unwrap();
    let mut longest = 1;
    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        let mut l = 1;
        while chars.next_if_eq(&c).is_some() {
            l += 1;
        }
        longest = longest.max(l);
    }
    println!("{longest}");
}
