pub fn iterator_sum_of_vector(nums: Vec<u32>) -> u32 {
    let mut counter = 0;

    // Needs to be mutable as `next` changes internal state to keep track where
    // iteration is at
    let mut nums_iter = nums.iter();

    counter += nums_iter.next().unwrap();
    counter += nums_iter.next().unwrap();
    counter += nums_iter.next().unwrap();

    counter
}

pub fn sum_of_vector(nums: Vec<u32>) -> u32 {
    nums.iter().sum()
}

pub fn double_vector_elements(nums: Vec<u32>) -> Vec<u32>{
    nums.iter().map(|x| x*2).collect()
}

pub fn get_even_elements(nums: Vec<u32>) -> Vec<u32>{
    nums.into_iter().filter(|x| x % 2 == 0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_a_vector_sum() {
        let nums = vec![1, 2, 3];
        assert_eq!(iterator_sum_of_vector(nums), 6);
    }

    #[test]
    fn vector_sum() {
        let nums = vec![1, 2, 3];
        assert_eq!(sum_of_vector(nums), 6);
    }

    #[test]
    fn double_vector() {
        let nums = vec![1, 2, 3];
        assert_eq!(double_vector_elements(nums), vec![2, 4, 6]);
    }
}
