use crate::helpers::f32_helper::{clamp, round_to};

/// Represents the [sRGB](https://www.w3.org/TR/css-color-4/#numeric-srgb) colorspace, consisting of a
/// triplet of percentage values (red, green, blue) identifying a point in the sRGB color space. It
/// also can include an optional alpha channel percentage.
///
/// If the alpha channel is omitted, it is assumed to be 100%.
///
/// All color channel values are optional to account for color blending. When blended with another
/// color, the omitted channel values are set to the corresponding value of the other color.
pub struct  SRgbSpace {
    /// The red channel represented as either a Some(f32) between 0 and 1 or None. If the value is
    /// Some(f32) and falls outside of the specified range, the value will be clamped to 0 if below
    /// 0, or 100 if above 100.
    pub red: Option<f32>,

    /// The green channel represented as either a Some(f32) between 0 and 1 or None. If the value is
    /// Some(f32) and falls outside of the specified range, the value will be clamped to 0 if below
    /// 0, or 100 if above 100.
    pub green: Option<f32>,

    /// The blue channel represented as either a Some(f32) between 0 and 1 or None. If the value is
    /// Some(f32) and falls outside of the specified range, the value will be clamped to 0 if below
    /// 0, or 1 if above 1.
    pub blue: Option<f32>,

    /// The alpha channel represented as either a Some(f32) between 0 and 1 or None. If the value is
    /// Some(f32) and falls outside of the specified range, the value will be clamped to 0 if below
    /// 0, or 1 if above 1.
    pub alpha: Option<f32>,
}

impl SRgbSpace {
    pub fn new(red: Option<f32>, green: Option<f32>, blue: Option<f32>, alpha: Option<f32>) -> Self {
        let red: Option<f32> = match red {
            Some(red) => Some(clamp(red, 0_f32, 1_f32)),
            None => None,
        };

        let green: Option<f32> = match green {
            Some(green) => Some(clamp(green, 0_f32, 1_f32)),
            None => None,
        };

        let blue: Option<f32> = match blue {
            Some(blue) => Some(clamp(blue, 0_f32, 1_f32)),
            None => None,
        };

        let alpha: Option<f32> = match alpha {
            Some(alpha) => Some(clamp(alpha, 0_f32, 1_f32)),
            None => None,
        };

        SRgbSpace { red, green, blue, alpha}
    }
}

/// Represents an [OKLab](https://www.w3.org/TR/css-color-4/#lab-colors) device-independent color,
/// intended to be perceptually uniform. Lab is a rectangular coordinate system with a central
/// Lightness axis. The a and b axes represent the hue.
///
/// The lightness, a and b values are optional to allow for color blending with another color. If
/// one these values is omitted, the values from the other color are used for the blend. If both
/// sides are not defined, the blended color is not able to be calculated.
///
/// If the alpha channel is not provided, it is treated as a value of 1 (100%).
pub struct LabSpace {

    /// The Lightness axis represented as a percentage. The illuminate of the OKLab color space is
    /// [D65](https://www.w3.org/TR/css-color-4/#d65-whitepoint).
    ///
    /// Values will be clamped between 0 and 4. The precision of the value is rounded to 3 decimal
    /// places of the percentage (0 - 400), creating a precision to the value to 5 places.
    ///
    /// Values above 1 (100%) are permitted for future compatibility with HDR devices.
    pub l: Option<f32>,

    /// Conveys hue red/green axis. Positive numbers are purplish red while negative numbers are the
    /// complementary color green. While theoretically the value is unbounded, in practice they do
    /// not exceed ±0.5 so they will be clamped to those values.
    pub a: Option<f32>,

    /// Conveys yellow/blue hue axis. Positive numbers are yellow while negative numbers are the
    /// complementary color blue. While theoretically the value is unbounded, in practice it does
    /// not exceed ±0.5, so will be clamped to those values.
    pub b: Option<f32>,


    /// The alpha channel represented as either a Some(f32) between 0 and 1 or None. If the value is
    /// Some(f32) and falls outside of the specified range, the value will be clamped to 0 if below
    /// 0, or 1 if above 1.
    pub alpha: Option<f32>,
}

impl LabSpace {
    pub fn new(l: Option<f32>, a: Option<f32>, b: Option<f32>, alpha: Option<f32>) -> Self {
        let l = match l {
            Some(v) => Some(round_to(clamp(v, 0_f32, 4_f32), 5)),
            None => None,
        };
        let a = match a {
            Some(v) => Some(clamp(v, -0.5_f32, 0.5_f32)),
            None => None,
        };
        let b = match b {
            Some(v) => Some(clamp(v, -0.5_f32, 0.5_f32)),
            None => None,
        };

        let alpha = match alpha {
            Some(v) => Some(clamp(v, 0_f32, 1_f32)),
            None => None,
        };

        LabSpace { l, a, b, alpha }
    }
}