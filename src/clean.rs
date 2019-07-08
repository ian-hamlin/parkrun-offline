pub trait Clean {
    fn remove_anchor(&self) -> Self;
    fn remove_percentage(&self) -> Self;
    fn normalise_age_grade(&self) -> Self;
    fn find_athlete_number(&self) -> Self;
}

impl Clean for String {
    fn remove_anchor(&self) -> Self {
        let mut result = String::new();
        // Find the bits to chop.
        let left = self.find('>');
        let right = self.rfind('<');

        if left.is_some() && right.is_some() {
            // If both are some, we can enter and unwrap safely.
            let left = left.unwrap();
            let right = right.unwrap();

            if left < right {
                result.push_str(&self[left + 1..right]);
            }
        }
        if result.is_empty() {
            // Else just keep the original input.
            result.push_str(self);
        }

        result
    }

    fn remove_percentage(&self) -> Self {
        let mut result = String::new();
        // Find the bit to chop.result
        if let Some(marker) = self.find('%') {
            result.push_str(&self[..marker].trim());
        } else {
            // Else just keep the original input.
            result.push_str(self);
        }

        result
    }

    fn normalise_age_grade(&self) -> Self {
        let mut result = String::new();

        match self.parse::<f64>() {
            Ok(x) if x >= 100f64 => result.push_str("World Record"),
            Ok(x) if x >= 90f64 && x < 100f64 => result.push_str("World Class"),
            Ok(x) if x >= 80f64 && x < 90f64 => result.push_str("National Class"),
            Ok(x) if x >= 70f64 && x < 80f64 => result.push_str("Regional Class"),
            Ok(x) if x >= 60f64 && x < 70f64 => result.push_str("Local Class"),
            _ => {}
        };

        result
    }

    /// athleteNumber=5799061
    fn find_athlete_number(&self) -> Self {
        let mut result = String::new();

        if let Some(left) = self.find("athleteNumber=") {
            if let Some(right) = self[left..].find('"') {
                result.push_str(&self[left + 14..right + left]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn find_athlete_number_empty() {
        let output = "no number".to_string().find_athlete_number();
        assert_eq!(String::from(""), output);
    }

    #[test]
    fn find_athlete_number() {
        let output =
            "<a href=\"athletehistory?athleteNumber=5799061\" target=\"_top\">Ian HAMLIN</a>"
                .to_string()
                .find_athlete_number();
        assert_eq!(String::from("5799061"), output);
    }

    #[test]
    fn normalise_age_grade_world_record() {
        let lower = "100".to_string().normalise_age_grade();

        assert_eq!("World Record".to_string(), lower);
    }

    #[test]
    fn normalise_age_grade_world_class() {
        let lower = "90".to_string().normalise_age_grade();
        let upper = "99.99".to_string().normalise_age_grade();

        assert_eq!("World Class".to_string(), lower);
        assert_eq!("World Class".to_string(), upper);
    }

    #[test]
    fn normalise_age_grade_national_class() {
        let lower = "80".to_string().normalise_age_grade();
        let upper = "89.99".to_string().normalise_age_grade();

        assert_eq!("National Class".to_string(), lower);
        assert_eq!("National Class".to_string(), upper);
    }

    #[test]
    fn normalise_age_grade_regional_class() {
        let lower = "70.00".to_string().normalise_age_grade();
        let upper = "79.99".to_string().normalise_age_grade();

        assert_eq!("Regional Class".to_string(), lower);
        assert_eq!("Regional Class".to_string(), upper);
    }

    #[test]
    fn normalise_age_grade_localclass() {
        let lower = "60.00".to_string().normalise_age_grade();
        let upper = "69.99".to_string().normalise_age_grade();

        assert_eq!("Local Class".to_string(), lower);
        assert_eq!("Local Class".to_string(), upper);
    }

    #[test]
    fn remove_percentage_should_return_input() {
        let input = String::from("hello world");
        let output = input.remove_percentage();

        assert_eq!("hello world".to_string(), output);
    }

    #[test]
    fn remove_percentage_should_return_section_before_percent() {
        let input = String::from(" hello  %world");
        let output = input.remove_percentage();

        assert_eq!("hello".to_string(), output);
    }

    #[test]
    fn remove_anchor_should_return_input() {
        let input = String::from("hello world");
        let output = input.remove_anchor();

        assert_eq!("hello world".to_string(), output);
    }

    #[test]
    fn remove_anchor_should_return_input_if_wrong_order() {
        let input = String::from("<hello world>");
        let output = input.remove_anchor();

        assert_eq!("<hello world>".to_string(), output);
    }

    #[test]
    fn remove_anchor_should_return_input_if_missing_end() {
        let input = String::from("<a href>hello world");
        let output = input.remove_anchor();

        assert_eq!("<a href>hello world".to_string(), output);
    }

    #[test]
    fn remove_anchor_should_return_input_if_missing_start() {
        let input = String::from("hello world</a>");
        let output = input.remove_anchor();

        assert_eq!("hello world</a>".to_string(), output);
    }

    #[test]
    fn remove_anchor_should_return_anchor_content() {
        let input = String::from("<a href>hello world</a>");
        let output = input.remove_anchor();

        assert_eq!("hello world".to_string(), output);
    }
}
