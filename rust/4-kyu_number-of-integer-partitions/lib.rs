pub fn partitions(n: u32) -> u32 {
    n.partitions().count() as u32
}

trait Partitions {
    fn partitions(self) -> PartitionIterator;
}

impl Partitions for u32 {
    fn partitions(self) -> PartitionIterator {
        PartitionIterator::new(self)
    }
}

struct PartitionIterator {
    n: u32,
    stack: Vec<u32>,
    current_sum: u32,
}

impl PartitionIterator {
    pub fn new(n: u32) -> PartitionIterator {
        PartitionIterator {
            n,
            stack: Vec::new(),
            current_sum: 0,
        }
    }
}

impl Iterator for PartitionIterator {
    type Item = Vec<u32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.stack.is_empty() {
            self.stack.push(self.n);
            self.current_sum = self.n;
            return Some(self.stack.clone());
        }

        while let Some(top) = self.stack.pop() {
            self.current_sum -= top;
            for i in (1..=top - 1).rev() {
                if self.current_sum + i <= self.n {
                    self.current_sum += i;
                    self.stack.push(i);
                    let mut remaining_sum = self.n - self.current_sum;
                    while remaining_sum > 0 {
                        let next_value = remaining_sum.min(i);
                        self.current_sum += next_value;
                        self.stack.push(next_value);
                        remaining_sum -= next_value;
                    }
                    return Some(self.stack.clone());
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::Partitions;

    use super::partitions;

    #[test]
    fn basic_test_01() {
        assert_eq!(partitions(1), 1);
    }

    #[test]
    fn basic_test_05() {
        assert_eq!(partitions(5), 7);
    }

    #[test]
    fn basic_test_10() {
        assert_eq!(partitions(10), 42);
    }

    #[test]
    fn basic_test_25() {
        assert_eq!(partitions(25), 1958);
    }

    #[test]
    fn partitions_test() {
        let cases: Vec<(u32, Vec<Vec<u32>>)> = vec![
            (1, vec![vec![1]]),
            (5, vec![vec![5], vec![4, 1], vec![3, 2], vec![3, 1, 1], vec![2, 2, 1], vec![2, 1, 1, 1], vec![1, 1, 1, 1, 1]]),
            (10, vec![vec![10], vec![9, 1], vec![8, 2], vec![8, 1, 1], vec![7, 3], vec![7, 2, 1], vec![7, 1, 1, 1], vec![6, 4], vec![6, 3, 1], vec![6, 2, 2], vec![6, 2, 1, 1], vec![6, 1, 1, 1, 1], vec![5, 5], vec![5, 4, 1], vec![5, 3, 2], vec![5, 3, 1, 1], vec![5, 2, 2, 1], vec![5, 2, 1, 1, 1], vec![5, 1, 1, 1, 1, 1], vec![4, 4, 2], vec![4, 4, 1, 1], vec![4, 3, 3], vec![4, 3, 2, 1], vec![4, 3, 1, 1, 1], vec![4, 2, 2, 2], vec![4, 2, 2, 1, 1], vec![4, 2, 1, 1, 1, 1], vec![4, 1, 1, 1, 1, 1, 1], vec![3, 3, 3, 1], vec![3, 3, 2, 2], vec![3, 3, 2, 1, 1], vec![3, 3, 1, 1, 1, 1], vec![3, 2, 2, 2, 1], vec![3, 2, 2, 1, 1, 1], vec![3, 2, 1, 1, 1, 1, 1], vec![3, 1, 1, 1, 1, 1, 1, 1], vec![2, 2, 2, 2, 2], vec![2, 2, 2, 2, 1, 1], vec![2, 2, 2, 1, 1, 1, 1], vec![2, 2, 1, 1, 1, 1, 1, 1], vec![2, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]]),
        ];
        for (n, expected) in cases {
            assert_eq!(n.partitions().collect::<Vec<Vec<u32>>>(), expected);
        }
    }
}
