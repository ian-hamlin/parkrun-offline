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
