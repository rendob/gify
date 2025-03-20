use std::error::Error;

use regex::Regex;

#[derive(Debug, Clone)]
pub struct Color(String);

impl Color {
    pub fn from(color: &str) -> Result<Self, Box<dyn Error>> {
        let color_code = match color {
            "red" => "#E7000BFF",
            "green" => "#00A63EFF",
            "blue" => "#3EA8FFFF",
            _ => color,
        };

        let color_code_pattern = Regex::new(r"^#[0-9a-fA-F]{8}$")?;
        if !color_code_pattern.is_match(color_code) {
            return Err(Box::from("invalid color code!"));
        }

        Ok(Color(color_code.to_string()))
    }

    pub fn rgba(&self) -> [u8; 4] {
        let red = u8::from_str_radix(&self.0[1..3], 16).unwrap();
        let green = u8::from_str_radix(&self.0[3..5], 16).unwrap();
        let blue = u8::from_str_radix(&self.0[5..7], 16).unwrap();
        let alpha = u8::from_str_radix(&self.0[7..9], 16).unwrap();

        [red, green, blue, alpha]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case::red("red")]
    #[case::green("green")]
    #[case::blue("blue")]
    #[case::upper_case("#3EA8FFFF")]
    #[case::lower_case("#89e633ff")]
    fn test_instantiation_success(#[case] color: &str) {
        let result = Color::from(color);

        assert!(result.is_ok());
    }

    #[rstest]
    #[case::invalid_name("pink")]
    #[case::out_of_range("#00A8G7FF")]
    #[case::without_hash("89e633ff")]
    #[case::short("#89e63f")]
    #[case::long("#89e63df3ff")]
    fn test_instantiation_failure(#[case] color: &str) {
        let result = Color::from(color);

        assert!(result.is_err());
    }

    #[rstest]
    #[case::upper_case("#3EA8FFFF", [62, 168, 255, 255])]
    #[case::lower_case("#89e633e0", [137, 230, 51, 224])]
    fn test_rgba(#[case] color: &str, #[case] expected: [u8; 4]) {
        let sut = Color::from(color).expect("failed to instantiate!");

        let result = sut.rgba();

        assert_eq!(result, expected);
    }
}
