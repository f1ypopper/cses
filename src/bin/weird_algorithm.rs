fn main(){
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.strip_suffix('\n').unwrap();
    let mut n = input.parse::<usize>().unwrap();
    while n > 1{
        print!("{n} ");
        if n % 2 == 0{
            n /= 2;
        }else{
            n=n*3+1;
        }
    }
    println!("{n}");
}