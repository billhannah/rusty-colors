//! Crate for declaring, converting and transforming colors as defined in the [CSS Color Module Level
//! 4](https://www.w3.org/TR/css-color-4/) Working Draft. The colors will be stored in memory as
//! either an sRGB or OKLab color space to simplify converting between color spaces. sRGB contains
//! 3 channels representing Red, Green, and Blue values as a percentage. OKLab is a perceptually
//! uniform color space using a rectangular coordinate system for hue and an axis to specify
//! lightness.
//!
//! The OKLab color space is used for internally representing LAB colors, as it's lightness range is
//! greater. Lightness values greater than 100% are permitted to allow for forwards compatibility
//! with HDR devices. If the Lightness value is converted to a CIE Lab color and the value
//! is greater than 1 (100%), it will be clamped to 1.

pub mod color_space;
pub mod helpers;