pub trait ParseTemplate<T> {
    fn parse_template(&self, template: &T) -> Self;
}
