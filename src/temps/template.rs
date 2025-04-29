use crate::user::Person;
use genpdf::Document;

pub trait CVTemplate {
    /// Generate a CV document using this template
    fn generate(&self, doc: &mut Document, person: &Person);

    /// Get the name of this template
    fn name(&self) -> &'static str;
}

// Helper function available to all templates
pub fn add_paragraph(doc: &mut Document, label: &str, items: &Option<Vec<String>>) {
    if let Some(item) = items {
        use genpdf::elements::Paragraph;
        let joined = item.join(", ");
        doc.push(Paragraph::new(format!("{}: {}", label, joined)));
    }
}
