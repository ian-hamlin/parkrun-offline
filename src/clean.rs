pub trait Clean {
    fn remove_anchor(&self) -> Self;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn should_return_input() {
        let input = String::from("hello world");
        let output = input.remove_anchor();

        assert_eq!("hello world".to_string(), output);
    }

    #[test]
    fn should_return_input_if_wrong_order() {
        let input = String::from("<hello world>");
        let output = input.remove_anchor();

        assert_eq!("<hello world>".to_string(), output);
    }

    #[test]
    fn should_return_input_if_missing_end() {
        let input = String::from("<a href>hello world");
        let output = input.remove_anchor();

        assert_eq!("<a href>hello world".to_string(), output);
    }

    #[test]
    fn should_return_input_if_missing_start() {
        let input = String::from("hello world</a>");
        let output = input.remove_anchor();

        assert_eq!("hello world</a>".to_string(), output);
    }

    #[test]
    fn should_return_anchor_content() {
        let input = String::from("<a href>hello world</a>");
        let output = input.remove_anchor();

        assert_eq!("hello world".to_string(), output);
    }
}
