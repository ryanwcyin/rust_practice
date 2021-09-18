/*
Given a sequence of integers as an array, determine whether it is possible 
to obtain a strictly increasing sequence by removing no more than one element from the array.
*/

/* my original algorithm
let mut prev_max: Option<i32>= None;
let mut max_val: Option<i32> = None;
let mut is_removed: bool = false;

for s in sequence.iter() {
    if max_val.is_none() || Some(*s) > max_val {
        prev_max = max_val;
        max_val = Some(*s);
    } else if prev_max.is_none() || Some(*s) > prev_max{
        if is_removed == true {
            return false;
        } else {
            is_removed = true;
            max_val = Some(*s);
        }
    } else {
        if is_removed == true {
            return false;
        } else {
            is_removed = true;
        }
    }
}
return true
*/

fn almost_increasing_sequence(sequence: Vec<i32>) -> bool {
    let win_2 = sequence.windows(2).filter(|v| v[0] >= v[1]).count();
    let win_3 = sequence.windows(3).filter(|v| v[0] >= v[2]).count();
    win_2 <= 1 && win_3 <= 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_almost_increasing_sequence(){
        assert_eq!(almost_increasing_sequence(vec![1,2,1,2]), false);
        assert_eq!(almost_increasing_sequence(vec![1,2,3,4]), true);
    }
}