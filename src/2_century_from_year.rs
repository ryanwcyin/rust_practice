/*
Given a year, return the century it is in. 
The first century spans from the year 1 up to and including the year 100, 
the second - from the year 101 up to and including the year 200, etc.
*/
fn century_from_year(year: i32) -> i32 {
    (year-1)/100 + 1
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_century_from_year() {
        assert_eq!( century_from_year(1700), 17);
        assert_eq!( century_from_year(2020), 21);
    }
}