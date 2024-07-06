fn modpow(n: usize, m: usize)->usize{
    if n == 0{return 1 % m}
    let mut u = modpow(n/2, m);
    u = (u*u)%m;
    if n % 2 == 1{u=(u*2)%m};
    u
}

fn main(){
    let n = std::io::stdin().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
    let m = (10 as usize).pow(9) + 7;
    println!("{}", modpow(n, m));
}