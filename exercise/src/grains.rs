#[allow(dead_code)]
pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");

    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    // 2^(s-1)
    2u64.pow(s - 1)
}


#[allow(dead_code)]
pub fn total() -> u64 {
    // todo!();

    // 2^64 - 1
    (2u128.pow(64) - 1) as u64

    // let mut sum = 0u64;
    //
    // for i in 1..65 {
    //     sum += square(i);
    // }
    //
    // sum
}
