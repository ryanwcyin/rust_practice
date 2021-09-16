/*
Given an array of integers, find the pair of adjacent elements 
that has the largest product and return that product.
*/

fn adjacent_elements_product(input_array: Vec<i32>) -> i32 {
    // window(): to iter with specific windows
    let product = input_array.windows(2)
                            .map(|w| w[0]*w[1])
                            .reduce(i32::max)
                            .unwrap();
    return product;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_adjacent_elements_product() {
        assert_eq!(adjacent_elements_product(vec![3,6,-2,7,3]), 21);
    }
}