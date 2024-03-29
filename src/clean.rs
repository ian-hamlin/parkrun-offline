use std::fmt::Write;

pub trait Clean {
    fn remove_anchor(&self) -> Self;
    fn remove_percentage(&self) -> Self;
    fn normalise_age_grade(&self) -> Self;
    fn find_athlete_number(&self) -> Self;
    fn normalise_first_timer(&self) -> Self;
    fn normalise_personal_best(&self) -> Self;
    fn normalise_time(&self) -> Self;
}

impl Clean for String {
    fn remove_anchor(&self) -> Self {
        let mut result = String::with_capacity(100);
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
        let mut result = String::with_capacity(10);
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
        let mut result = String::with_capacity(15);

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
        let mut result = String::with_capacity(15);

        if let Some(left) = self.find("athleteNumber=") {
            if let Some(right) = self[left..].find('"') {
                result.push_str(&self[left + 14..right + left]);
            }
        }

        result
    }

    fn normalise_first_timer(&self) -> Self {
        if self.to_lowercase().contains("first timer") {
            String::from("Yes")
        } else {
            String::from("")
        }
    }

    fn normalise_personal_best(&self) -> Self {
        if self.to_lowercase().contains("new pb") {
            String::from("Yes")
        } else {
            String::from("")
        }
    }

    fn normalise_time(&self) -> Self {
        let mut result = String::with_capacity(8);

        let parts: Vec<&str> = self.split(':').collect();

        if parts.len() == 3 {
            let _ = write!(&mut result, "{:0>2}:", parts[0]);
            let _ = write!(&mut result, "{:0>2}:", parts[1]);
            let _ = write!(&mut result, "{:0>2}", parts[2]);
        } else if parts.len() == 2 {
            result.push_str("00:");
            let _ = write!(&mut result, "{:0>2}:", parts[0]);
            let _ = write!(&mut result, "{:0>2}", parts[1]);
        } else {
            result.push_str(self);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn normalise_time_not_human() {
        let output = "9:22".to_string().normalise_time();
        assert_eq!(String::from("00:09:22"), output);
    }

    #[test]
    fn normalise_time_sub_hour() {
        let output = "18:22".to_string().normalise_time();
        assert_eq!(String::from("00:18:22"), output);
    }

    #[test]
    fn normalise_time_single_hour() {
        let output = "1:20:22".to_string().normalise_time();
        assert_eq!(String::from("01:20:22"), output);
    }

    #[test]
    fn normalise_time_full_hours() {
        let output = "21:20:22".to_string().normalise_time();
        assert_eq!(String::from("21:20:22"), output);
    }

    #[test]
    fn normalise_personal_best_yes() {
        let output = "New PB!".to_string().normalise_personal_best();
        assert_eq!(String::from("Yes"), output);
    }

    #[test]
    fn normalise_personal_best_no() {
        let output = "Pew NB!".to_string().normalise_personal_best();
        assert_eq!(String::from(""), output);
    }

    #[test]
    fn normalise_first_timer_yes() {
        let output = "First Timer!".to_string().normalise_first_timer();
        assert_eq!(String::from("Yes"), output);
    }

    #[test]
    fn normalise_first_timer_no() {
        let output = "Tirst Fimer!".to_string().normalise_first_timer();
        assert_eq!(String::from(""), output);
    }

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
