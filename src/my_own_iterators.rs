struct Counter {
    count: u32
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 5 {
            return None
        }
        self.count += 1;
        Some(self.count - 1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate_a_vector_sum() {
        let counter = Counter{ count: 0 };
        assert_eq!(counter.into_iter().collect::<Vec<_>>(), vec![0, 1, 2, 3, 4]);
    }
}