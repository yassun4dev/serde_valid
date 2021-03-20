#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FieldName(String);

impl FieldName {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self(name.into())
    }
}

impl<'a> PartialEq<&'a str> for FieldName {
    fn eq(&self, other: &&'a str) -> bool {
        &self.0 == other
    }
}
