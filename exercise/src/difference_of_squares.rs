#[allow(dead_code)]
pub fn square_of_sum(n: u32) -> u32 {
    //todo!("square of sum of 1...{n}")

    let mut sum = 0;
    for i in 0..=n {
        sum += i;
    }

    sum *= sum;

    sum
}

#[allow(dead_code)]
pub fn sum_of_squares(n: u32) -> u32 {
    // todo!("sum of squares of 1...{n}")

    let mut sum = 0;

    for i in 0..=n {
        sum += i * i;
    }

    sum
}

#[allow(dead_code)]
pub fn difference(n: u32) -> u32 {
    // todo!("difference between square of sum of 1...{n} and sum of squares of 1...{n}")

    square_of_sum(n) - sum_of_squares(n)
}
