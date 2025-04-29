use genpdf::{
    elements::{self, Break, LinearLayout, Paragraph},
    style, Alignment, Document, Element,
};

use super::line::Line;
use crate::temps::template::CVTemplate;
use crate::user::Person;

pub struct CleanTemplate;

impl CleanTemplate {
    pub fn new() -> Self {
        Self {}
    }

    fn ct_header(&self, doc: &mut Document, p: &Person) {
        doc.push(Paragraph::default().styled_string(
            p.name.to_string(),
            style::Style::new().bold().with_font_size(15),
        ));

        doc.push(Break::new(0.5));
        doc.push(Line);
        doc.push(Paragraph::new(p.contact.to_string()));
        doc.push(Break::new(1));
    }

    fn ct_summary(&self, doc: &mut Document, p: &Person) {
        doc.push(Paragraph::new("Summary").styled(style::Style::new().bold().with_font_size(14)));
        doc.push(Break::new(0.5));
        doc.push(Line);
        doc.push(Break::new(0.5));
        doc.push(Paragraph::new(p.summary.clone()));
        doc.push(Break::new(1));
    }

    fn ct_we(&self, doc: &mut Document, p: &Person) {
        if let Some(exp) = &p.work_experience {
            doc.push(
                Paragraph::new("Work History")
                    .styled(style::Style::new().bold().with_font_size(14)),
            );
            doc.push(Break::new(0.5));
            doc.push(Line);
            doc.push(Break::new(0.5));
            for e in exp {
                let mut achievement_list = elements::UnorderedList::with_bullet("•");
                for ach in &e.achievements {
                    achievement_list.push(Paragraph::new(ach));
                }
                doc.push(
                    elements::UnorderedList::with_bullet("•").element(
                        LinearLayout::vertical()
                            .element(
                                Paragraph::new(format!("{} at {}", &e.title, &e.company))
                                    .styled(style::Effect::Bold),
                            )
                            .element(
                                Paragraph::new(format!(" {} - {}", &e.start_date, &e.end_date))
                                    .aligned(Alignment::Right),
                            )
                            .element(achievement_list),
                    ),
                );
                doc.push(Break::new(1));
            }
        }
    }

    fn ct_projs(&self, doc: &mut Document, p: &Person) {
        if let Some(projs) = &p.projects {
            doc.push(
                Paragraph::new("Projects").styled(style::Style::new().bold().with_font_size(14)),
            );
            doc.push(Break::new(0.5));
            doc.push(Line);
            doc.push(Break::new(0.5));

            for proj in projs {
                let used_tech = proj.technologies.join(", ");
                doc.push(
                    elements::UnorderedList::with_bullet("•").element(
                        LinearLayout::vertical()
                            .element(Paragraph::new(proj.name.clone()).styled(style::Effect::Bold))
                            .element(Paragraph::new(proj.description.clone()))
                            .element(Paragraph::new(proj.url.clone()).styled(style::Effect::Italic))
                            .element(Paragraph::new(format!("Technologies: {}", used_tech))),
                    ),
                );
                doc.push(Break::new(1));
            }
        }
    }

    fn ct_edu(&self, doc: &mut Document, p: &Person) {
        doc.push(Paragraph::new("Education").styled(style::Style::new().bold().with_font_size(14)));
        doc.push(Break::new(0.5));
        doc.push(Line);
        doc.push(Break::new(0.5));
        for e in &p.education {
            let courses = &e.courses.join(", ");
            doc.push(
                LinearLayout::vertical()
                    .element(Paragraph::new(e.institution.clone()))
                    .element(Paragraph::new(e.degree.clone()))
                    .element(Paragraph::new(format!("GPA: {}", e.gpa)))
                    .element(Paragraph::new(format!("Courses: {}", courses)))
                    .element(
                        Paragraph::new(format!("{} - {}", e.start_date, e.end_date))
                            .aligned(Alignment::Right),
                    ),
            );
        }
    }

    fn ct_skills(&self, doc: &mut Document, p: &Person) {
        doc.push(Paragraph::new("Skills").styled(style::Style::new().bold().with_font_size(14)));
        doc.push(Break::new(0.5));
        doc.push(Line);

        self.add_paragraph(doc, "Languages", &p.skills.languages);
        self.add_paragraph(doc, "Technicals", &p.skills.technical);
        self.add_paragraph(doc, "Certifications", &p.skills.certifications);
        self.add_paragraph(doc, "Tools", &p.skills.tools);
        self.add_paragraph(doc, "Version Control", &p.skills.version_control);
    }

    fn add_paragraph(&self, doc: &mut Document, label: &str, items: &Option<Vec<String>>) {
        if let Some(item) = items {
            let joined = item.join(", ");
            doc.push(Paragraph::new(format!("{}: {}", label, joined)));
        }
    }
}

impl CVTemplate for CleanTemplate {
    fn generate(&self, doc: &mut Document, p: &Person) {
        self.ct_header(doc, p);
        self.ct_summary(doc, p);
        self.ct_we(doc, p);
        self.ct_projs(doc, p);
        self.ct_edu(doc, p);
        self.ct_skills(doc, p);
    }

    fn name(&self) -> &'static str {
        "clean"
    }
}
