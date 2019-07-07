pub trait Html {
    fn remove_anchor(&self) -> Self;
}

impl Html for String {
    fn remove_anchor(&self) -> Self
    {
        String::from("yo")
    }
}
