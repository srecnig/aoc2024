pub fn blink(stones: Vec<i64>) -> Vec<i64> {
    let mut new_stones: Vec<i64> = Vec::new();
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
        } else {
            let digits = stone.to_string();
            let digit_count = digits.len();
            if digit_count % 2 == 0 {
                let left: i64 = digits[0..digit_count / 2].parse().unwrap();
                new_stones.push(left);
                let right: i64 = digits[digit_count / 2..].parse().unwrap();
                new_stones.push(right);
            } else {
                new_stones.push(stone * 2024);
            }
        }
    }
    new_stones
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_blink_at_0() {
        let mut stones: Vec<i64> = vec![0];
        stones = blink(stones);
        assert_eq!(vec![1], stones);
    }

    #[test]
    fn can_blink_at_even_digits() {
        let mut stones: Vec<i64> = vec![23];
        stones = blink(stones);
        assert_eq!(vec![2, 3], stones);
    }

    #[test]
    fn can_blink_at_other_stoge() {
        let mut stones: Vec<i64> = vec![234];
        stones = blink(stones);
        assert_eq!(vec![473616], stones);
    }

    #[test]
    fn can_blink() {
        let mut stones: Vec<i64> = vec![125, 17];
        stones = blink(stones);
        assert_eq!(vec![253000, 1, 7], stones);
    }
}
