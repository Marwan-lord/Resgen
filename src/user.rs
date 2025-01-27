use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person {
    pub name: String,
    pub address: String,
    pub email: String,
    pub phone: String,
    pub summary: String,
    pub work_experience: Option<Vec<Work>>,
    pub education: Vec<Edu>,
    pub skills: Vec<String>,
    pub projects: Option<Vec<Project>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub start_date: String,
    pub end_date: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Work {
    pub title: String,
    pub company: String,
    pub start_date: String,
    pub end_date: String,
    pub description: String,
}

impl Display for Work {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}  from {} to  {} {}",
            self.title, self.company, self.start_date, self.end_date, self.description
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edu {
    pub degree: String,
    pub institution: String,
    pub start_date: String,
    pub end_date: String,
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}             from {} to  {}",
            self.institution, self.degree, self.start_date, self.end_date
        )
    }
}
