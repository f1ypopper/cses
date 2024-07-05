//sets DON'T have to be of equal size in case of even n

fn print_vec(v: &Vec<usize>){
    println!("{}", v.len());
    println!("{}",v.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(" "));
}
fn main() {
    let mut n = std::io::stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let n_sum = n * (n + 1) / 2;
    if n_sum % 2 != 0 {
        println!("NO");
        return;
    }
    println!("YES");
    let total = n_sum / 2;
    let mut set1 = Vec::new();
    let mut set2 = Vec::new();
    let mut sum = 0;
    while n != 0{
        if sum+n <= total{
            sum+=n;
            set1.push(n);
        }else{
            set2.push(n);
        }
        n-=1;
    }
    print_vec(&set1);
    print_vec(&set2);
}
