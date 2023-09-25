// https://stackoverflow.com/questions/34733811/what-is-the-difference-between-iter-and-into-iter
// `into_iter`, vs `iter` vs `iter_mut`

// TL;DR:
// The iterator returned by into_iter may yield any of T, &T or &mut T, depending on the context.
// The iterator returned by iter will yield &T, by convention.
// The iterator returned by iter_mut will yield &mut T, by convention

// https://dev.to/wrongbyte/implementing-iterator-and-intoiterator-in-rust-3nio
// Implementing your own Iterator

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
    fn call_next_on_iterator() {
        // Note that the counter must be mutable, since `next` mutates it.
        let mut counter = Counter{ count: 0 };
        assert_eq!(counter.next(), Some(0));
    }

    #[test]
    fn call_next_on_iterator2() {
        // Here counter does not need to be mutable since `into_iter` returns iterator 
        // that yields back references to the item.
        let counter = Counter{ count: 0 };
        let mut iter = counter.into_iter();
        assert_eq!(iter.next(), Some(0));
    }

    #[test]
    fn collect_iterator() {
        let counter = Counter{ count: 0 };
        assert_eq!(counter.into_iter().collect::<Vec<_>>(), vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn take_some_from_iterator() {
        let counter = Counter{ count: 0 };
        assert_eq!(counter.into_iter().take(3).collect::<Vec<_>>(), vec![0, 1, 2]);
    }
}
