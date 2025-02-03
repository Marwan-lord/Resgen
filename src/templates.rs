use genpdf::{elements::{self, Break, LinearLayout, Paragraph, }, style, Alignment, Document, Element};

use crate::user::Person;

fn dt_header(doc: &mut Document, p: &Person) {
    let header_layout = LinearLayout::vertical()
        .element(
            Paragraph::default()
            .styled_string(&p.name, style::Effect::Bold)
            .aligned(Alignment::Center),
        )
        .element(
            Paragraph::default()
            .styled_string(format!("Address: {}", &p.address), style::Effect::Italic)
            .aligned(Alignment::Center),
        )
        .element(Break::new(1))
        .element(Paragraph::new(&p.contact.to_string()).aligned(Alignment::Center));

    doc.push(header_layout);
    doc.push(Break::new(1));
}

fn dt_summary(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Summary").styled(style::Effect::Bold));
    doc.push(Paragraph::new(&p.summary));
    doc.push(Break::new(1));
}

fn dt_edu(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Education").styled(style::Effect::Bold));

    for e in &p.education {
        let courses = &e.courses.join(", ");
        doc.push(
            LinearLayout::vertical()
            .element(Paragraph::new(&e.degree))
            .element(Paragraph::new(&e.institution))
            .element(Paragraph::new(format!("GPA: {}", &e.gpa)))
            .element(Paragraph::new(format!("Courses: {}", courses)))
            .element(
                Paragraph::new(format!("{} - {}", &e.start_date, &e.end_date))
                .aligned(Alignment::Right),
            ),
        );
    }
    doc.push(Break::new(1));
}


fn dt_we(doc: &mut Document, p: &Person) {
    if let Some(exp) = &p.work_experience {
        doc.push(Paragraph::new("Work Experience").styled(style::Effect::Bold));
        for e in exp {
            let mut achievement_list = elements::UnorderedList::new();
            for ach in &e.achievements {
                achievement_list.push(Paragraph::new(ach));
            }

            doc.push(
                elements::UnorderedList::new().element(
                    LinearLayout::vertical()
                    .element(Paragraph::new(&e.title).styled(style::Effect::Bold))
                    .element(
                        Paragraph::new(format!("At {}", &e.company))
                        .styled(style::Effect::Italic),
                    )
                    .element(Break::new(1))
                    .element(achievement_list)
                    .element(
                        Paragraph::new(format!(
                                "{} - {}",
                                &e.start_date, &e.end_date
                        ))
                        .aligned(Alignment::Right),
                    ),
                ),
            );
            doc.push(Break::new(1));
        }
    }
}

fn dt_projs(doc: &mut Document, p: &Person) {
    if let Some(projs) = &p.projects {
        doc.push(Paragraph::new("Projects").styled(style::Effect::Bold));
        for proj in projs {
            let used_tech = proj.technologies.join(", ");
            doc.push(
                elements::UnorderedList::new().element(
                    LinearLayout::vertical()
                    .element(Paragraph::new(&proj.name).styled(style::Effect::Bold))
                    .element(Paragraph::new(&proj.url).styled(style::Effect::Italic))
                    .element(Paragraph::new(&proj.description))
                    .element(Paragraph::new(format!(
                                "Technologies: {}",
                                used_tech
                    ))),
                ),
            );
            doc.push(Break::new(1));
        }
    }

}

fn dt_skills(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Skills").styled(style::Effect::Bold));
    add_paragraph(doc, "Languages", &p.skills.languages);
    add_paragraph(doc, "Technicals", &p.skills.technical);
    add_paragraph(doc, "Certifications", &p.skills.certifications);
    add_paragraph(doc, "Tools", &p.skills.tools);
    add_paragraph(doc, "Version Control", &p.skills.version_control);
}

fn add_paragraph(doc: &mut Document, label: &str, items: &Option<Vec<String>>) {
    if let Some(item) = items {
        let joined = item.join(", ");
        doc.push(Paragraph::new(format!("{}: {}", label, joined)));
    }
}

fn dt_init(doc: &mut Document, p: &Person) {
    dt_header(doc, p);
    dt_summary(doc, p);
    dt_edu(doc, p);
    dt_we(doc, p);
    dt_projs(doc, p);
    dt_skills(doc, p);
}

pub fn gen_default_temp(doc: &mut Document, p: &Person) {
    dt_init(doc, p);
}

fn ct_header(doc: &mut Document, p: &Person) {
    doc.push(
        Paragraph::default().styled_string(format!("{}", &p.name), style::Effect::Bold),
    );
    doc.push(Paragraph::new(&p.contact.to_string()));
    doc.push(Break::new(1));
}

fn ct_summary(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Summary").styled(style::Effect::Bold));
    doc.push(Paragraph::new(&p.summary));
    doc.push(Break::new(1));
}

fn ct_we(doc: &mut Document, p: &Person) {
    if let Some(exp) = &p.work_experience {
        doc.push(Paragraph::new("Work History").styled(style::Effect::Bold));
        for e in exp {
            let mut achievement_list = elements::UnorderedList::new();
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
                        Paragraph::new(format!(
                                " {} - {}",
                                &e.start_date, &e.end_date
                        ))
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
        doc.push(Paragraph::new("Projects").styled(style::Effect::Bold));
        for proj in projs {
            let mut used_tech = String::new();
            for used in &proj.technologies {
                used_tech.push_str(used.as_str());
                used_tech.push(' ');
            }
            doc.push(
                elements::UnorderedList::with_bullet("•").element(
                    LinearLayout::vertical()
                    .element(Paragraph::new(&proj.name).styled(style::Effect::Bold))
                    .element(Paragraph::new(&proj.description))
                    .element(Paragraph::new(&proj.url).styled(style::Effect::Italic))
                    .element(Paragraph::new(format!(
                                "Technologies: {}",
                                used_tech
                    ))),
                ),
            );
            doc.push(Break::new(1));
        }
    }
}

fn ct_edu(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Education").styled(style::Effect::Bold));
    for e in &p.education {
        let courses = &e.courses.join(", ");
        doc.push(
            LinearLayout::vertical()
            .element(Paragraph::new(&e.institution))
            .element(Paragraph::new(&e.degree))
            .element(Paragraph::new(format!("GPA: {}", &e.gpa)))
            .element(Paragraph::new(format!("Courses: {}", courses)))
            .element(
                Paragraph::new(format!("{} - {}", &e.start_date, &e.end_date))
                .aligned(Alignment::Right),
            ),
        );
    }
}

fn ct_skills(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Skills").styled(style::Effect::Bold));
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
