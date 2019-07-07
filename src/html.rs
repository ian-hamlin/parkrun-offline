pub trait Html {
    fn remove_anchor(&self) -> Self;
}

impl Html for String {
    fn remove_anchor(&self) -> Self {
        let mut result = String::new();

        if let Some(left) = self.find('>') {
            if let Some(right) = self.rfind('<') {
                result.push_str(&self[left + 1..right]);
            }
        }

        if result.is_empty() {
            result.push_str(self);
        }

        result
    }
}
