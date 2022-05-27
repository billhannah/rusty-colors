use crate::color_space::SRgbSpace;
use crate::helpers::f32_helper::as_percentage;

pub struct Rgb {
    color_space: SRgbSpace,
}

impl Rgb {
    pub fn new(r: Option<f32>, g: Option<f32>, b: Option<f32>, a: Option<f32>) -> Self {
        Rgb {
            color_space: SRgbSpace::new(r, g, b, a),
        }
    }

    pub fn css_as_percentage(&self) -> String {
        let r = match self.color_space.red {
            None => "none".to_string(),
            Some(value) => as_percentage(value),
        };

        let g = match self.color_space.green {
            None => "none".to_string(),
            Some(value) => as_percentage(value),
        };

        let b = match self.color_space.blue {
            None => "none".to_string(),
            Some(value) => as_percentage(value),
        };

        let a = match self.color_space.alpha {
            None => "".to_string(),
            Some(value) => format!(" {}", as_percentage(value)),
        };

        format!("rgb({} {} {}{})", r, g, b, a)
    }
}

#[cfg(test)]
mod tests {
    use crate::colors::rgb::Rgb;

    #[test]
    fn rgb_with_all_values_formats_as_percentage() {
        let rgb = Rgb::new(Some(0.5), Some(0.5), Some(0.5), Some(1_f32));
        let css = rgb.css_as_percentage();

        assert_eq!(css, "rgb(50% 50% 50% 100%)");
    }

    #[test]
    fn rgb_with_no_red_formats_as_percentage() {
        let rgb = Rgb::new(None, Some(0.5), Some(0.5), Some(1_f32));
        let css = rgb.css_as_percentage();

        assert_eq!(css, "rgb(none 50% 50% 100%)");
    }

    #[test]
    fn rgb_with_no_green_formats_as_percentage() {
        let rgb = Rgb::new(Some(0.5), None, Some(0.5), Some(1_f32));
        let css = rgb.css_as_percentage();

        assert_eq!(css, "rgb(50% none 50% 100%)");
    }

    #[test]
    fn rgb_with_no_blue_formats_as_percentage() {
        let rgb = Rgb::new(Some(0.5), Some(0.5), None, Some(1_f32));
        let css = rgb.css_as_percentage();

        assert_eq!(css, "rgb(50% 50% none 100%)");
    }

    #[test]
    fn rgb_with_no_alpha_formats_as_percentage() {
        let rgb = Rgb::new(Some(0.5), Some(0.5), Some(0.5), None);
        let css = rgb.css_as_percentage();

        assert_eq!(css, "rgb(50% 50% 50%)");
    }
}