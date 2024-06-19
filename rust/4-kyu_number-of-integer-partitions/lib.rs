pub fn partitions(n: u32) -> u32 {
    let mut partitions = vec![0; n as usize + 1];
    partitions[0] = 1;

    for i in 1..=n as usize {
        for j in i..=n as usize {
            partitions[j] += partitions[j - i];
        }
    }

    partitions[n as usize]
}

#[cfg(test)]
mod tests {
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
}
