use std::fmt::{self, Display, Formatter};

use genpdf::{elements, fonts, style, Alignment, Document, Element, SimplePageDecorator};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    address: String,
    email: String,
    phone: String,
    summary: String,
    work_experience: Option<Vec<Work>>,
    education: Vec<Edu>,
    skills: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Work {
    title: String,
    company: String,
    start_date: String,
    end_date: String,
    description: String,
}

impl Display for Work {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}             from {} to  {} {}",
            self.title, self.company, self.start_date, self.end_date, self.description
        )
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct Edu {
    degree: String,
    institution: String,
    start_date: String,
    end_date: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let strings = self
            .education
            .iter()
            .map(|s| format!("{}", s))
            .collect::<Vec<String>>()
            .join(",");
        write!(f, "{}", strings)
    }
}

impl Display for Edu {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, {}             from {} to  {}",
            self.institution, self.degree, self.start_date, self.end_date
        )
    }
}

fn main() {
    let data = include_str!("../resume.json");
    let p: Person = serde_json::from_str(data).expect("Unable to read json from file");

    let font = fonts::from_files("./fonts", "LiberationSans", None).expect("Failed to load font");
    let mut doc = Document::new(font);

    doc.set_font_size(15);
    doc.set_title("Demo document");

    let mut deco = SimplePageDecorator::new();
    deco.set_margins(10);
    doc.set_page_decorator(deco);

    let header_layout = elements::LinearLayout::vertical()
        .element(
            elements::Paragraph::default()
                .styled_string(p.name, style::Effect::Bold)
                .aligned(Alignment::Center),
        )
        .element(
            elements::Paragraph::default()
                .styled_string(format!("{}\t{}", p.address, p.email), style::Effect::Italic)
                .aligned(Alignment::Center),
        );

    doc.push(header_layout);
    doc.push(elements::Paragraph::new(""));

    doc.push(elements::Paragraph::new("Summary").styled(style::Effect::Bold));
    doc.push(elements::Paragraph::new(p.summary));
    doc.push(elements::Paragraph::new(""));

    doc.push(elements::Paragraph::new("Education").styled(style::Effect::Bold));
    for e in &p.education {
        doc.push(elements::Paragraph::new(e.to_string()));
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
                                "from {} to {}",
                                &e.start_date, &e.end_date
                            ))
                            .aligned(Alignment::Right),
                        ),
                ),
            );
            doc.push(elements::Paragraph::new(""));
        }
    }
    doc.push(elements::Paragraph::new("Skills").styled(style::Effect::Bold));

    let mut string_of_skills = String::new();
    for s in &p.skills {
        string_of_skills.push_str(s.as_str());
        string_of_skills.push_str(", ");
    }

    doc.push(elements::Paragraph::new(string_of_skills));

    doc.render_to_file("demo.pdf")
        .expect("Error Rendering file to output");
}
