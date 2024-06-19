pub trait Median: Iterator {
    fn median(self) -> Option<Self::Item>;
}

impl <I> Median for I
where
    I: Iterator<Item = f64>
{
    fn median(self) -> Option<Self::Item> {
        let mut vec = self.collect::<Vec<_>>();
        let len = vec.len();
        if len == 0 {
            return None
        }
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mid = len / 2;
        if len % 2 == 0 {
            Some((vec[mid - 1] + vec[mid]) / 2f64)
        } else {
            Some(vec[mid])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Median;

    #[test]
    fn median_test() {
        let cases: Vec<(Option<f64>, Vec<f64>)> = vec![
            (None, vec![]),
            (Some(2.0), vec![1.0, 2.0, 3.0]),
            (Some(2.5), vec![1.0, 2.0, 3.0, 4.0]),
        ];
        for (expected, input) in cases {
            assert_eq!(input.into_iter().median(), expected)
        }
    }
}
