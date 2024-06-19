mod median;
use crate::median::Median;

mod partitions;
use crate::partitions::Partitions;

pub fn part(n: i64) -> String {
    let mut partitions: Vec<u32> = (n as u32)
        .partitions()
        .map(|partition| partition.into_iter().product())
        .collect::<Vec<u32>>();
    partitions.sort();
    partitions.dedup();

    let (max, min, sum, count) = partitions.clone().into_iter()
        .fold(
            (u32::MIN, u32::MAX, 0u32, 0u32),
            |(max, min, sum, count), element| (
                max.max(element),
                min.min(element),
                sum + element,
                count + 1,
            )
        );

    let med = partitions.clone().into_iter()
        .map(|n| n as f64)
        .median()
        .unwrap();

    format!(
        "Range: {} Average: {:.*} Median: {:.*}",
        max - min,
        2, sum as f64 / count as f64,
        2, med
    )
}

#[cfg(test)]
mod tests {
    use super::part;

    fn testequal(ans: &str, sol: &str) {
        assert!(ans == sol, "Expected \"{}\", got \"{}\".", sol, ans);
    }

    #[test]
    fn returns_expected() {
        testequal(&part(1), "Range: 0 Average: 1.00 Median: 1.00");
        testequal(&part(2), "Range: 1 Average: 1.50 Median: 1.50");
        testequal(&part(3), "Range: 2 Average: 2.00 Median: 2.00");
        testequal(&part(4), "Range: 3 Average: 2.50 Median: 2.50");
        testequal(&part(5), "Range: 5 Average: 3.50 Median: 3.50");
    }
}
