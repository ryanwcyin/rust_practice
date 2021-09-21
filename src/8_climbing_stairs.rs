/*
You are climbing a staircase that has n steps. 
You can take the steps either 1 or 2 at a time. 
Calculate how many distinct ways you can climb to the top of the staircase.
*/

fn climbing_stairs(n: i32) -> i32 {
    let n: usize = n as usize;
    let mut table = vec![0; n+1];
    table[0] = 1;
    table[1] = 1;
    for i in 2..n+1 {
        table[i] = table[i-1] + table[i-2];
    }
    table[n]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_climbing_stairs(){
        assert_eq!(climbing_stairs(13), 377);
    }
}