#[allow(dead_code)]
pub fn is_leap_year(year: u64) -> bool {
    // todo!("true if {year} is a leap year")

    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}