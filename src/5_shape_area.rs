/*
Below we will define an n-interesting polygon. 
Your task is to find the area of a polygon for a given n.

A 1-interesting polygon is just a square with a side of length 1. 
An n-interesting polygon is obtained by taking the n - 1-interesting 
polygon and appending 1-interesting polygons to its rim, side by side. 
*/
fn shape_area(n: i32) -> i32 {
    let mut sum = 1;
    for i in 0..n {
        sum += i*4;
    }
    return sum;
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_shape_area() {
        assert_eq!(shape_area(3), 13);
    }
}