pub mod user;
use crate::user::Person;

use genpdf::{elements, fonts, style, Alignment, Document, Element, SimplePageDecorator};

fn gen_default_temp(doc: &mut Document, p: &Person) {
    let header_layout = elements::LinearLayout::vertical()
        .element(
            elements::Paragraph::default()
                .styled_string(&p.name, style::Effect::Bold)
                .aligned(Alignment::Center),
        )
        .element(
            elements::Paragraph::default()
                .styled_string(&p.address, style::Effect::Italic)
                .aligned(Alignment::Center),
        )
        .element(elements::Paragraph::new(p.contact.to_string()).aligned(Alignment::Center));

    doc.push(header_layout);
    doc.push(elements::Paragraph::new(""));

    doc.push(elements::Paragraph::new("Summary").styled(style::Effect::Bold));
    doc.push(elements::Paragraph::new(&p.summary));
    doc.push(elements::Paragraph::new(""));

    doc.push(elements::Paragraph::new("Education").styled(style::Effect::Bold));
    for e in &p.education {
        doc.push(
            elements::LinearLayout::vertical()
                .element(elements::Paragraph::new(&e.degree))
                .element(elements::Paragraph::new(&e.institution))
                .element(
                    elements::Paragraph::new(format!("-from {} to {}", &e.start_date, &e.end_date))
                        .aligned(Alignment::Right),
                ),
        );
    }
    doc.push(elements::Paragraph::new(""));

    if let Some(exp) = &p.work_experience {
        doc.push(elements::Paragraph::new("Work Experience").styled(style::Effect::Bold));
        for e in exp {
            doc.push(
                elements::UnorderedList::new().element(
                    elements::LinearLayout::vertical()
                        .element(elements::Paragraph::new(&e.title).styled(style::Effect::Bold))
                        .element(
                            elements::Paragraph::new(format!("At {}", &e.company))
                                .styled(style::Effect::Italic),
                        )
                        .element(elements::Paragraph::new(&e.description))
                        .element(
                            elements::Paragraph::new(format!(
                                "-from {} to {}",
                                &e.start_date, &e.end_date
                            ))
                            .aligned(Alignment::Right),
                        ),
                ),
            );
            doc.push(elements::Paragraph::new(""));
        }
    }

    if let Some(projs) = &p.projects {
        doc.push(elements::Paragraph::new("Projects").styled(style::Effect::Bold));
        for proj in projs {
            let mut used_tech = String::new();
            for used in &proj.technologies {
                used_tech.push_str(used.as_str());
                used_tech.push_str(", ");
            }
            doc.push(
                elements::UnorderedList::new().element(
                    elements::LinearLayout::vertical()
                        .element(elements::Paragraph::new(&proj.name).styled(style::Effect::Bold))
                        .element(elements::Paragraph::new(&proj.url).styled(style::Effect::Italic))
                        .element(elements::Paragraph::new(&proj.description))
                        .element(elements::Paragraph::new(format!("Used: {}", used_tech))),
                ),
            )
        }
    }
    doc.push(elements::Paragraph::new("Skills").styled(style::Effect::Bold));

    let mut string_of_skills = String::new();
    for s in &p.skills {
        string_of_skills.push_str(s.as_str());
        string_of_skills.push_str(", ");
    }

    doc.push(elements::Paragraph::new(string_of_skills));
}

fn main() {
    let data = include_str!("../resume2.json");
    let p: Person = serde_json::from_str(data).expect("Unable to read json from file");

    let font = fonts::from_files("./fonts", "LiberationSans", None).expect("Failed to load font");
    let mut doc = Document::new(font);

    doc.set_font_size(12);
    doc.set_title("Demo document");

    let mut deco = SimplePageDecorator::new();
    deco.set_margins(10);
    doc.set_page_decorator(deco);

    gen_default_temp(&mut doc, &p);

    doc.render_to_file("demo.pdf")
        .expect("Error Rendering file to output");
}
