pub fn beam_scales(left: f64, right: f64) -> f64 {
    if left == 0.0 && right == 0.0 {
        return 0.0;
    };
    if left == right {
        return 0.0;
    };
    if left == 0.0 {
        return 1.0;
    };
    if right == 0.0 {
        return -1.0;
    };

    let half = (left + right) / 2.0;
    return if left > right {
        (half - left) / half
    } else {
        (right - half) / half
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn beam_scales_works() {
        // Arrange.
        let cases = vec![
            (0.0, 0.0, 0.0),
            (1.0, 1.0, 0.0),
            (1.0, 0.0, -1.0),
            (0.0, 1.0, 1.0),
            (15.0, 5.0, -0.5),
            (5.0, 15.0, 0.5),
            (7.0, 1.0, -0.75),
            (1.0, 7.0, 0.75),
            (5.0, 3.0, -0.25),
            (3.0, 5.0, 0.25),
        ];

        cases.iter().for_each(|(left, right, expected)| {
            // Action.
            let current = beam_scales(*left, *right);

            // Assert.
            assert_eq!(current, *expected)
        });
    }
}
