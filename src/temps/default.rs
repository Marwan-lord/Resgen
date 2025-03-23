use genpdf::{
    elements::{self, Break, LinearLayout, Paragraph},
    style, Alignment, Document, Element, Position, RenderResult, Size,
};

use crate::user::Person;

pub struct Line;

impl Element for Line {
    fn render(
        &mut self,
        _: &genpdf::Context,
        area: genpdf::render::Area<'_>,
        style: style::Style,
    ) -> Result<genpdf::RenderResult, genpdf::error::Error> {
        area.draw_line(
            vec![
                Position {
                    x: 0.into(),
                    y: 0.into(),
                },
                Position {
                    x: area.size().width,
                    y: 0.into(),
                },
            ],
            style.with_color(style::Color::Rgb(0, 0, 0)),
        );

        Ok(RenderResult {
            size: Size {
                width: area.size().width,
                height: 1.into(),
            },
            has_more: false,
        })
    }
}

fn dt_header(doc: &mut Document, p: &Person) {
    let header_layout = LinearLayout::vertical()
        .element(
            Paragraph::default()
                .styled_string(p.name, style::Style::new().bold().with_font_size(15))
                .aligned(Alignment::Center),
        )
        .element(
            Paragraph::default()
                .styled_string(format!("Address: {}", p.address), style::Effect::Italic)
                .aligned(Alignment::Center),
        )
        .element(Break::new(1))
        .element(Paragraph::new(p.contact.to_string()).aligned(Alignment::Center));

    doc.push(header_layout);
    doc.push(Break::new(1));
}

fn dt_summary(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Summary").styled(style::Style::new().bold().with_font_size(14)));
    doc.push(Break::new(0.5));
    doc.push(Paragraph::new(p.summary));
    doc.push(Break::new(1));
}

fn dt_edu(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Education").styled(style::Style::new().bold().with_font_size(14)));

    doc.push(Break::new(0.5));
    for e in &p.education {
        let courses = &e.courses.join(", ");
        doc.push(
            LinearLayout::vertical()
                .element(Paragraph::new(e.degree))
                .element(
                    Paragraph::new(format!("{} - {}", e.start_date, e.end_date))
                        .aligned(Alignment::Right),
                )
                .element(Paragraph::new(format!(
                    "Graduated from {} with GPA {}",
                    e.institution, e.gpa
                )))
                .element(Paragraph::new(format!("Courses: {}", courses))),
        );
    }
    doc.push(Break::new(1));
}

fn dt_we(doc: &mut Document, p: &Person) {
    if let Some(exp) = &p.work_experience {
        doc.push(
            Paragraph::new("Work Experience").styled(style::Style::new().bold().with_font_size(14)),
        );

        doc.push(Break::new(0.5));
        for e in exp {
            let mut achievement_list = elements::UnorderedList::with_bullet("•");
            for ach in &e.achievements {
                achievement_list.push(Paragraph::new(*ach));
            }

            doc.push(
                elements::UnorderedList::with_bullet("•").element(
                    LinearLayout::vertical()
                        .element(Paragraph::new(e.title).styled(style::Effect::Bold))
                        .element(Paragraph::new(format!("At {}", &e.company)))
                        .element(Break::new(1))
                        .element(achievement_list)
                        .element(
                            Paragraph::new(format!("{} - {}", &e.start_date, &e.end_date))
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
        doc.push(Paragraph::new("Projects").styled(style::Style::new().bold().with_font_size(14)));
        doc.push(Break::new(0.5));
        for proj in projs {
            let used_tech = proj.technologies.join(", ");
            doc.push(
                elements::UnorderedList::with_bullet("•").element(
                    LinearLayout::vertical()
                        .element(Paragraph::new(proj.name).styled(style::Effect::Bold))
                        .element(Paragraph::new(proj.url).styled(style::Effect::Italic))
                        .element(Paragraph::new(proj.description))
                        .element(Paragraph::new(format!("Technologies: {}", used_tech))),
                ),
            );
            doc.push(Break::new(1));
        }
    }
}

fn dt_skills(doc: &mut Document, p: &Person) {
    doc.push(Paragraph::new("Skills").styled(style::Style::new().bold().with_font_size(14)));
    doc.push(Break::new(0.5));
    add_paragraph(doc, "Languages", &p.skills.languages);
    add_paragraph(doc, "Technicals", &p.skills.technical);
    add_paragraph(doc, "Certifications", &p.skills.certifications);
    add_paragraph(doc, "Tools", &p.skills.tools);
    add_paragraph(doc, "Version Control", &p.skills.version_control);
}

pub fn add_paragraph(doc: &mut Document, label: &str, items: &Option<Vec<String>>) {
    if let Some(item) = items {
        let joined = item.join(", ");
        doc.push(Paragraph::new(format!("{}: {}", label, joined)));
    }
}

fn dt_init(doc: &mut Document, p: &Person) {
    dt_header(doc, p);
    doc.push(Line);
    dt_summary(doc, p);
    dt_edu(doc, p);
    doc.push(Line);
    dt_we(doc, p);
    doc.push(Line);
    dt_projs(doc, p);
    doc.push(Line);
    dt_skills(doc, p);
}

pub fn gen_default_temp(doc: &mut Document, p: &Person) {
    dt_init(doc, p);
}
