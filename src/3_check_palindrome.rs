/*
Given the string, check if it is a palindrome.
*/

fn check_palindrome(input_string: String) -> bool {
    // rev : reverse
    // collect: iterator to collection/String/Result
    input_string.chars().rev().collect::<String>() == input_string
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_palindrome() {
        // convert &str to String
        assert_eq!(check_palindrome(String::from("abcde")), false);  
        // A syntactic sugar of above
        assert_eq!(check_palindrome("aka".to_string()), true);
    }
}