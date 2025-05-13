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
    if left > right {
        (half - left) / half
    } else {
        (right - half) / half
    }
}

const PERCENT_I: i32 = 100; // 100%.
pub fn calc_i32(price: i32, count: i32, discount: i32) -> i32 {
    (price * count * (discount + PERCENT_I)) / PERCENT_I
}

const PERCENT_F: f64 = 1.0; // 100%.
const KOP: f64 = 0.01; // Cents.
pub fn calc_f64(price: f64, count: f64, discount: f64) -> f64 {
    let sum = price * count * (discount + PERCENT_F);
    sum - (sum % KOP)
}

pub fn float_to_integer(v: f64, tick: f64) -> i32 {
    // TODO: binary split float
    (v / tick) as i32
}

pub fn integer_to_float(v: i32, tick: f64) -> f64 {
    // TODO: binary join float
    let res = v as f64 * tick;
    res - (res % tick)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam_scales() {
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

    #[test]
    fn test_calc_i32() {
        let current = calc_i32(12345, 12, 5);
        let expected = 155547;
        assert_eq!(current, expected)
    }

    #[test]
    fn test_calc_f64() {
        let current = calc_f64(123.45, 12.0, 0.05);
        let expected = 1555.47;
        assert_eq!(current, expected)
    }

    #[test]
    fn test_float_to_integer() {
        let cases = vec![
            (0.00123456, 0.00000001, 123456),
            (0.0123456, 0.0000001, 123456),
            (0.123456, 0.000001, 123456),
            (1.23456, 0.00001, 123456),
            (12.3456, 0.0001, 123455), // <=
            (123.456, 0.001, 123456),
            (1234.56, 0.01, 123455), // <=
            (12345.6, 0.1, 123456),
            (123456.0, 1.0, 123456),
            (1234560.0, 10.0, 123456),
            (12345600.0, 100.0, 123456),
        ];

        cases.iter().for_each(|(v, tick, expected)| {
            let current = float_to_integer(*v, *tick);
            assert_eq!(current, *expected, "v:{v}, tick:{}", *tick)
        });
    }

    #[test]
    fn test_integer_to_float() {
        let cases = vec![
            (123456, 0.00000001, 0.00123455),
            (123456, 0.0000001, 0.0123456),
            (123456, 0.000001, 0.123456),
            (123456, 0.00001, 1.23456),
            (123456, 0.0001, 12.345600000000001), // <=
            (123456, 0.001, 123.456),
            (123456, 0.01, 1234.55), // <=
            (123456, 0.1, 12345.5),  // <=
            (123456, 1.0, 123456.0),
            (123456, 10.0, 1234560.0),
            (123456, 100.0, 12345600.0),
        ];

        cases.iter().for_each(|(v, tick, expected)| {
            let current = integer_to_float(*v, *tick);
            assert_eq!(current, *expected, "v:{v}, tick:{}", *tick)
        });
    }
}
