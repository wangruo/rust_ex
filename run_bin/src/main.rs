fn main() {
    let mut sum = 0u64;

    for i in 1..65 {
        sum += 2u64.pow(i - 1);
    }

    println!("{}", sum);
    println!("{}", 2u128.pow(64) - 1);
}
