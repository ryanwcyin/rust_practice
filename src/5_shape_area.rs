/*
Below we will define an n-interesting polygon. 
Your task is to find the area of a polygon for a given n.

A 1-interesting polygon is just a square with a side of length 1. 
An n-interesting polygon is obtained by taking the n - 1-interesting 
polygon and appending 1-interesting polygons to its rim, side by side. 
*/
fn shape_area(n: i32) -> i32 {
    // create a range iter with i..j
    (0..n).map(|i| i*4).sum::<i32>() + 1
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_shape_area() {
        assert_eq!(shape_area(3), 13);
    }
}