pub fn two_knights(n: i128) -> i128 {
    // (n - 1) * (n + 4) * (4 + n * n - 3 * n) / 2
    match n {
        ..1 => 0,
        1.. => {
            // 3 4 5 5 5 ...
            // 4 5 7 7 7 ...
            // 5 7 9 9 9 ...
            // 5 7 9 9 9 ...
            // 5 7 9 9 9 ...
            // ...
            // trick formatter
            // (n * n - 3) * 4 / 2
            //     + (n * n - 4) * 8 / 2
            //     + (n * n - 5) * (n - 3) * 4 / 2
            //     + (n * n - 7) * (n - 4) * 4 / 2
            //     + (n * n - 9) * (n - 4) * (n - 4) / 2

            // 2 * n * n - 6 + 4 * n * n - 16
            //     + 2 * (n * n - 5) * (n - 3)
            //     + 2 * (n * n - 7) * (n - 4)
            //     + (n * n - 9) * (n - 4) * (n - 4) / 2

            (n.pow(4) - 9 * n.pow(2)) / 2 + 12 * n - 8
        }
    }
}

#[no_mangle]
pub fn a(n: i128) -> i128 {
    if n < 1 {
        return 0;
    }
    0
    // trick formatter
        + (n * n - 3) * 4 / 2
        + (n * n - 4) * 8 / 2
        + (n * n - 5) * (n - 3) * 4 / 2
        + (n * n - 7) * (n - 4) * 4 / 2
        + (n * n - 9) * (n - 4) * (n - 4) / 2
}

#[no_mangle]
pub fn b(n: i128) -> i128 {
    if n < 1 {
        return 0;
    }
    (n * n * n * n - 9 * n * n) / 2 + 12 * n - 8
}

#[cfg(test)]
mod tests {
    use rayon::iter::{IntoParallelIterator, ParallelIterator};

    use super::*;

    #[test]
    fn test_two_knights() {
        // assert_eq!(two_knights(0), 0);
        assert_eq!(two_knights(1), 0);
        assert_eq!(two_knights(2), 6);
        assert_eq!(two_knights(3), 28);
        assert_eq!(two_knights(4), 96);
        assert_eq!(two_knights(5), 252);
        assert_eq!(two_knights(6), 550);
        assert_eq!(two_knights(7), 1056);
        assert_eq!(two_knights(8), 1848);
        assert_eq!(two_knights(1e5 as i128), 49999999955001199992);
    }

    #[test]
    fn test_a_b() {
        (-100_000..=2_100_000_000)
            .into_par_iter()
            .for_each(|n| assert_eq!(a(n), b(n)));
    }
}
