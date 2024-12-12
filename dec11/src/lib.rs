use std::collections::HashMap;

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

pub fn blink_efficiently(initial_stones: &Vec<i64>, blink_count: i32) -> i64 {
    let mut stones: HashMap<i64, i64> = HashMap::new();
    for stone in initial_stones {
        stones.insert(*stone, 1);
    }

    for _ in 0..blink_count {
        let mut new_stones: HashMap<i64, i64> = HashMap::new();
        let stone_values: Vec<i64> = stones.keys().cloned().collect();

        for stone_value in stone_values {
            let current_count = stones.remove(&stone_value).unwrap();

            if stone_value == 0 {
                // 0 turns into 1
                let new_value: i64 = 1;
                new_stones
                    .entry(new_value)
                    .and_modify(|e| *e += current_count)
                    .or_insert(current_count);
            } else {
                let digits = stone_value.to_string();
                let digit_count = digits.len();
                if digit_count % 2 == 0 {
                    let left: i64 = digits[0..digit_count / 2].parse().unwrap();
                    new_stones
                        .entry(left)
                        .and_modify(|e| *e += current_count)
                        .or_insert(current_count);

                    let right: i64 = digits[digit_count / 2..].parse().unwrap();
                    new_stones
                        .entry(right)
                        .and_modify(|e| *e += current_count)
                        .or_insert(current_count);
                } else {
                    let new_value = stone_value * 2024;
                    new_stones
                        .entry(new_value)
                        .and_modify(|e| *e += current_count)
                        .or_insert(current_count);
                }
            };
        }
        stones = new_stones;
    }
    stones.values().sum()
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

    #[test]
    fn can_blink_efficiently() {
        let stones: Vec<i64> = vec![125, 17];
        assert_eq!(9, blink_efficiently(&stones, 4));
        assert_eq!(22, blink_efficiently(&stones, 6));
    }
}
