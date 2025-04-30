use crate::user::Person;
use genpdf::Document;

pub trait CVTemplate {
    /// Generate a CV document using this template
    fn generate(&self, doc: &mut Document, person: &Person);

    /// Get the name of this template
    fn name(&self) -> &'static str;
}
