/// Clamps a value between min and max.
///
/// # Arguments
///
/// * `value` - the f32 value to be clamped
/// * `min` - returned if values < min
/// * `max` - returned if value is > max
///
/// # Examples
/// ```
/// use rusty_colors::helpers::f32_helper;
///
/// let val1 = f32_helper::clamp(-1_f32, 0_f32, 1_f32); // val1 = 0_f32
/// let val2 = f32_helper::clamp(2_f32, 0_f32, 1_f32); // val2 = 1_f32
/// let val3 = f32_helper::clamp(1_f32, 0_f32, 2_f32); // val3 = 3_f32
/// ```
pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        return min;
    }

    if value > max {
        return max;
    }

    value
}

/// Rounds a number to provided precision.
///
/// # Arguments
///
/// * `value` - the value to be rounded
/// * `precision` - the number of decimal places to be rounded to
///
/// # Examples
/// ```
/// use rusty_colors::helpers::f32_helper;
///
/// let val1 = f32_helper::round_to(1.2345_f32, 3_i32); // val1 = 1.235_f32
/// let val2 = f32_helper::round_to(1.2345_f32, 2_i32); // val2 = 1.23_f32
/// let val3 = f32_helper::round_to(1.234_f32, 5_i32); // val3 = 1.234_f32
/// ```
pub fn round_to(value: f32, precision: i32) -> f32 {
    let multiplier = 10_f32.powi(precision);
    let value = (value * multiplier).round();
    value / multiplier
}

#[cfg(test)]
mod tests {
    use crate::helpers::f32_helper::{clamp, round_to};

    #[test]
    fn clamp_returns_min_when_value_less_than_min() {
        let val = clamp(-1_f32, 0_f32, 1_f32);
        assert_eq!(val, 0_f32);
    }

    #[test]
    fn clamp_returns_max_when_value_greater_than_max() {
        let val = clamp(2_f32, 0_f32, 1_f32);
        assert_eq!(val, 1_f32);
    }

    #[test]
    fn clamp_returns_value_when_value_between_min_and_max() {
        let val = clamp(1_f32, 0_f32, 2_f32);
        assert_eq!(val, 1_f32);
    }

    #[test]
    fn round_to_rounds_up_last_digit_when_expected() {
        let val = round_to(1.2345_f32, 3_i32);
        assert_eq!(val, 1.235_f32);
    }

    #[test]
    fn round_to_rounds_leaves_last_digit_unchanged_when_expected() {
        let val = round_to(1.2344_f32, 3_i32);
        assert_eq!(val, 1.234_f32);
    }

    #[test]
    fn round_to_returns_values_unmodified_when_expected() {
        let val = round_to(1.234_f32, 3_i32);
        assert_eq!(val, 1.234_f32);
    }

    #[test]
    fn round_to_does_not_return_insignificant_digits() {
        let val = round_to(1.23_f32, 10_i32);
        assert_eq!(val, 1.23_f32);
    }
}
