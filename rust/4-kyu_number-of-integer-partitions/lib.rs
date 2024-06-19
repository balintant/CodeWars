mod partitions;
use crate::partitions::Partitions;

pub fn partitions(n: u32) -> u32 {
    n.partitions().count() as u32
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
