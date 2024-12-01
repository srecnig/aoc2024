pub fn difference_vector(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut sorted_left = left.clone();
    let mut sorted_right = right.clone();
    let mut result = Vec::new();
    sorted_left.sort();
    sorted_right.sort();

    for i in 0..sorted_left.len() {
        let difference = sorted_left[i] - sorted_right[i];
        result.push(difference.abs());
    }

    result
}

pub fn similarity_score(number: i32, vector: &[i32]) -> i32 {
    let frequency: i32 = vector
        .iter()
        .filter(|&e| *e == number)
        .count()
        .try_into()
        .expect("Weird things going on!");
    frequency * number
}

pub fn sum_up_vector(vector: &[i32]) -> i32 {
    vector.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_difference_vector() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        let result = vec![2, 1, 0, 1, 2, 5];
        assert_eq!(result, difference_vector(&left, &right));
    }

    #[test]
    fn can_sum_up_vector() {
        let vector = vec![5, 0, 1, 2];
        assert_eq!(8, sum_up_vector(&vector));
    }

    #[test]
    fn can_get_similarity_score() {
        let vector = vec![4, 3, 5, 3, 9, 3];
        assert_eq!(9, similarity_score(3, &vector));
        assert_eq!(4, similarity_score(4, &vector));
        assert_eq!(0, similarity_score(2, &vector));
        assert_eq!(0, similarity_score(1, &vector));
    }
}
