/*
Write a function that returns the sum of two numbers.

Example

For param1 = 1 and param2 = 2, the output should be
add(param1, param2) = 3
*/

fn add(param1: i32, param2: i32) -> i32 {
    param1 + param2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!( add(1, 2), 3);
    }
}
