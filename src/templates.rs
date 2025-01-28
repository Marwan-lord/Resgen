use genpdf::{elements, style, Alignment, Document, Element};

use crate::user::Person;

pub fn gen_default_temp(doc: &mut Document, p: &Person) {
    let header_layout = elements::LinearLayout::vertical()
        .element(
            elements::Paragraph::default()
                .styled_string(&p.name, style::Effect::Bold)
                .aligned(Alignment::Center),
        )
        .element(
            elements::Paragraph::default()
                .styled_string(format!("Address: {}", &p.address), style::Effect::Italic)
                .aligned(Alignment::Center),
        )
        .element(elements::Break::new(1))
        .element(elements::Paragraph::new(&p.contact.to_string()).aligned(Alignment::Center));

    doc.push(header_layout);
    doc.push(elements::Break::new(1));

    doc.push(elements::Paragraph::new("Summary").styled(style::Effect::Bold));
    doc.push(elements::Paragraph::new(&p.summary));
    doc.push(elements::Break::new(1));

    doc.push(elements::Paragraph::new("Education").styled(style::Effect::Bold));

    for e in &p.education {
        let courses = &e.courses.join(", ");
        doc.push(
            elements::LinearLayout::vertical()
                .element(elements::Paragraph::new(&e.degree))
                .element(elements::Paragraph::new(&e.institution))
                .element(elements::Paragraph::new(format!("GPA: {}", &e.gpa)))
                .element(elements::Paragraph::new(format!("Courses: {}", courses)))
                .element(
                    elements::Paragraph::new(format!("{} - {}", &e.start_date, &e.end_date))
                        .aligned(Alignment::Right),
                ),
        );
    }
    doc.push(elements::Break::new(1));

    if let Some(exp) = &p.work_experience {
        doc.push(elements::Paragraph::new("Work Experience").styled(style::Effect::Bold));
        for e in exp {
            let mut achievement_list = elements::UnorderedList::new();
            for ach in &e.achievements {
                achievement_list.push(elements::Paragraph::new(ach));
            }

            doc.push(
                elements::UnorderedList::with_bullet("-").element(
                    elements::LinearLayout::vertical()
                        .element(elements::Paragraph::new(&e.title).styled(style::Effect::Bold))
                        .element(
                            elements::Paragraph::new(format!("At {}", &e.company))
                                .styled(style::Effect::Italic),
                        )
                        .element(elements::Break::new(1))
                        .element(achievement_list)
                        .element(
                            elements::Paragraph::new(format!(
                                "{} - {}",
                                &e.start_date, &e.end_date
                            ))
                            .aligned(Alignment::Right),
                        ),
                ),
            );
            doc.push(elements::Break::new(1));
        }
    }

    if let Some(projs) = &p.projects {
        doc.push(elements::Paragraph::new("Projects").styled(style::Effect::Bold));
        for proj in projs {
            let used_tech = proj.technologies.join(", ");
            doc.push(
                elements::UnorderedList::with_bullet("-").element(
                    elements::LinearLayout::vertical()
                        .element(elements::Paragraph::new(&proj.name).styled(style::Effect::Bold))
                        .element(elements::Paragraph::new(&proj.url).styled(style::Effect::Italic))
                        .element(elements::Paragraph::new(&proj.description))
                        .element(elements::Paragraph::new(format!(
                            "Technologies: {}",
                            used_tech
                        ))),
                ),
            );
            doc.push(elements::Break::new(1));
        }
    }
    doc.push(elements::Paragraph::new("Skills").styled(style::Effect::Bold));

    add_paragraph(doc, "Languages", &p.skills.languages);
    add_paragraph(doc, "Technicals", &p.skills.technical);
    add_paragraph(doc, "Certifications", &p.skills.certifications);
    add_paragraph(doc, "Tools", &p.skills.tools);
    add_paragraph(doc, "Version Control", &p.skills.version_control);
}

fn add_paragraph(doc: &mut Document, label: &str, items: &Option<Vec<String>>) {
    if let Some(item) = items {
        let joined = item.join(", ");
        doc.push(elements::Paragraph::new(format!("{}: {}", label, joined)));
    }
}

pub fn gen_clean_temp(doc: &mut Document, p: &Person) {
    doc.push(
        elements::Paragraph::default().styled_string(format!("{}", &p.name), style::Effect::Bold),
    );
    doc.push(elements::Paragraph::new(&p.contact.to_string()));
    doc.push(elements::Break::new(1));

    doc.push(elements::Paragraph::new("Summary").styled(style::Effect::Bold));
    doc.push(elements::Paragraph::new(&p.summary));
    doc.push(elements::Break::new(1));

    if let Some(exp) = &p.work_experience {
        doc.push(elements::Paragraph::new("Work History").styled(style::Effect::Bold));
        for e in exp {
            let mut achievement_list = elements::UnorderedList::new();
            for ach in &e.achievements {
                achievement_list.push(elements::Paragraph::new(ach));
            }
            doc.push(
                elements::UnorderedList::with_bullet("•").element(
                    elements::LinearLayout::vertical()
                        .element(
                            elements::Paragraph::new(format!("{} at {}", &e.title, &e.company))
                                .styled(style::Effect::Bold),
                        )
                        .element(
                            elements::Paragraph::new(format!(
                                " {} - {}",
                                &e.start_date, &e.end_date
                            ))
                            .aligned(Alignment::Right),
                        )
                        .element(achievement_list),
                ),
            );
            doc.push(elements::Break::new(1));
        }
    }

    if let Some(projs) = &p.projects {
        doc.push(elements::Paragraph::new("Projects").styled(style::Effect::Bold));
        for proj in projs {
            let mut used_tech = String::new();
            for used in &proj.technologies {
                used_tech.push_str(used.as_str());
                used_tech.push(' ');
            }
            doc.push(
                elements::UnorderedList::with_bullet("•").element(
                    elements::LinearLayout::vertical()
                        .element(elements::Paragraph::new(&proj.name).styled(style::Effect::Bold))
                        .element(elements::Paragraph::new(&proj.description))
                        .element(elements::Paragraph::new(&proj.url).styled(style::Effect::Italic))
                        .element(elements::Paragraph::new(format!(
                            "Technologies: {}",
                            used_tech
                        ))),
                ),
            );
            doc.push(elements::Break::new(1));
        }
    }

    doc.push(elements::Paragraph::new("Education").styled(style::Effect::Bold));
    for e in &p.education {
        let courses = &e.courses.join(", ");
        doc.push(
            elements::LinearLayout::vertical()
                .element(elements::Paragraph::new(&e.institution))
                .element(elements::Paragraph::new(&e.degree))
                .element(elements::Paragraph::new(format!("GPA: {}", &e.gpa)))
                .element(elements::Paragraph::new(format!("Courses: {}", courses)))
                .element(
                    elements::Paragraph::new(format!("{} - {}", &e.start_date, &e.end_date))
                        .aligned(Alignment::Right),
                ),
        );
    }

    doc.push(elements::Paragraph::new("Skills").styled(style::Effect::Bold));
    add_paragraph(doc, "Languages", &p.skills.languages);
    add_paragraph(doc, "Technicals", &p.skills.technical);
    add_paragraph(doc, "Certifications", &p.skills.certifications);
    add_paragraph(doc, "Tools", &p.skills.tools);
    add_paragraph(doc, "Version Control", &p.skills.version_control);
}
