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
/// let val1 = helpers::f32::clamp(-1_f32, 0_f32, 1_f32); // val1 = 0_f32
/// let val2 = helpers::f32::clamp(2_f32, 0_f32, 1_f32); // val2 = 1_f32
/// let val3 = helpers::f32::clamp(1_f32, 0_f32, 2_f32); // val3 = 3_f32
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
/// let val1 = helpers::f32::round_to(1.2345_f32, 3_i32); // val1 = 1.235_f32
/// let val2 = helpers::f32::round_to(1.2345_f32, 2_i32); // val2 = 1.23_f32
/// let val3 = helpers::f32::round_to(1.234_f32, 5_i32); // val3 = 1.234_f32
/// ```
pub fn round_to(value: f32, precision: i32) -> f32 {
    let multiplier = 10_f32.powi(precision);
    let value = (value * multiplier).round();
    value / multiplier
}