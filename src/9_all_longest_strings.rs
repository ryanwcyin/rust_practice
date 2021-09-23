/*
Given an array of strings, return another array containing all of its longest strings.
*/

fn all_longest_strings(input_array: Vec<String>) -> Vec<String> {
    let max_len = input_array.iter().map(|string| string.len()).max().unwrap();
    input_array.into_iter().filter(|string| string.len().eq(&max_len)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_all_longest_strings(){
        assert_eq!(all_longest_strings(vec!["apple".to_string(), 
                                            "orange".to_string(), 
                                            "kiwi".to_string()]), 
                                        vec!["orange".to_string()]);
    }
}