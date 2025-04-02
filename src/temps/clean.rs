use genpdf::{
    elements::{self, Break, LinearLayout, Paragraph},
    style, Alignment, Document, Element,
};

use crate::temps::default::add_paragraph;
use crate::user::Person;

use super::default::Line;

fn ct_header(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::default().styled_string(
        p.name.to_string(),
        style::Style::new().bold().with_font_size(15),
    ));

    doc.push(Break::new(0.5));
    doc.push(Line);
    doc.push(Paragraph::new(p.contact.to_string()));
    doc.push(Break::new(1));
}

fn ct_summary(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Summary").styled(style::Style::new().bold().with_font_size(14)));
    doc.push(Break::new(0.5));
    doc.push(Line);
    doc.push(Break::new(0.5));
    doc.push(Paragraph::new(p.summary));
    doc.push(Break::new(1));
}

fn ct_we(doc: &mut Document, p: &Person) {
    if let Some(exp) = &p.work_experience {
        doc.push(
            Paragraph::new("Work History").styled(style::Style::new().bold().with_font_size(14)),
        );
        doc.push(Break::new(0.5));
        doc.push(Line);
        doc.push(Break::new(0.5));
        for e in exp {
            let mut achievement_list = elements::UnorderedList::with_bullet("•");
            for ach in &e.achievements {
                achievement_list.push(Paragraph::new(*ach));
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

fn ct_projs(doc: &mut Document, p: &Person) {
    if let Some(projs) = &p.projects {
        doc.push(Paragraph::new("Projects").styled(style::Style::new().bold().with_font_size(14)));
        doc.push(Break::new(0.5));
        doc.push(Line);
        doc.push(Break::new(0.5));

        for proj in projs {
            let mut used_tech = String::new();
            for used in &proj.technologies {
                used_tech.push_str(used);
                used_tech.push(' ');
            }
            doc.push(
                elements::UnorderedList::with_bullet("•").element(
                    LinearLayout::vertical()
                        .element(Paragraph::new(proj.name).styled(style::Effect::Bold))
                        .element(Paragraph::new(proj.description))
                        .element(Paragraph::new(proj.url).styled(style::Effect::Italic))
                        .element(Paragraph::new(format!("Technologies: {}", used_tech))),
                ),
            );
            doc.push(Break::new(1));
        }
    }
}

fn ct_edu(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Education").styled(style::Style::new().bold().with_font_size(14)));
    doc.push(Break::new(0.5));
    doc.push(Line);
    doc.push(Break::new(0.5));
    for e in &p.education {
        let courses = &e.courses.join(", ");
        doc.push(
            LinearLayout::vertical()
                .element(Paragraph::new(e.institution))
                .element(Paragraph::new(e.degree))
                .element(Paragraph::new(format!("GPA: {}", e.gpa)))
                .element(Paragraph::new(format!("Courses: {}", courses)))
                .element(
                    Paragraph::new(format!("{} - {}", e.start_date, e.end_date))
                        .aligned(Alignment::Right),
                ),
        );
    }
}

fn ct_skills(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Skills").styled(style::Style::new().bold().with_font_size(14)));
    doc.push(Break::new(0.5));
    doc.push(Line);

    add_paragraph(doc, "Languages", &p.skills.languages);
    add_paragraph(doc, "Technicals", &p.skills.technical);
    add_paragraph(doc, "Certifications", &p.skills.certifications);
    add_paragraph(doc, "Tools", &p.skills.tools);
    add_paragraph(doc, "Version Control", &p.skills.version_control);
}

pub fn gen_clean_temp(doc: &mut Document, p: &Person) {
    ct_header(doc, p);
    ct_summary(doc, p);
    ct_we(doc, p);
    ct_projs(doc, p);
    ct_edu(doc, p);
    ct_skills(doc, p);
}

// TODO: Make another template
