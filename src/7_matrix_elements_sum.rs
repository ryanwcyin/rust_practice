/*
Given matrix, a rectangular matrix of integers, 
where each value represents the cost of the room, 
your task is to return the total sum of all rooms 
that are suitable for the CodeBots (ie: add up all the values 
that don't appear below a 0).
*/
fn matrix_elements_sum(matrix: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;
    let mut haunted = Vec::new();
    for i in matrix.iter(){
        for (jdx, j) in i.iter().enumerate(){
            if *j == 0 {
                haunted.push(jdx);
            }
            if !haunted.contains(&jdx) {
                ans+=j;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_matrix_elements_sum(){
        let matrix = vec![vec![1, 1, 1, 0], 
                        vec![0, 5, 0, 1], 
                        vec![2, 1, 3, 10]];
        assert_eq!(matrix_elements_sum(matrix), 9);

    }
}