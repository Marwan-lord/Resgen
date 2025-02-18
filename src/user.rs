use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Person<'a> {
    pub name: &'a str,
    pub address: &'a str,
    pub contact: Contact<'a>,
    pub summary: &'a str,
    pub work_experience: Option<Vec<Work<'a>>>,
    pub education: Vec<Edu<'a>>,
    pub skills: Skills,
    pub projects: Option<Vec<Project<'a>>>,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Skills {
    pub languages: Option<Vec<String>>,
    pub version_control: Option<Vec<String>>,
    pub certifications: Option<Vec<String>>,
    pub technical: Option<Vec<String>>,
    pub tools: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact<'a> {
    pub email: &'a str,
    pub phone: &'a str,
    pub linkedin: &'a str,
    pub github: &'a str,
}
impl Display for Contact<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            " {} | {} | {} | {} ",
            self.email, self.phone, self.linkedin, self.github
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub technologies: Vec<&'a str>,
    pub url: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Work<'a> {
    pub title: &'a str,
    pub company: &'a str,
    pub start_date: &'a str,
    pub end_date: &'a str,
    pub achievements: Vec<&'a str>,
}

//impl Display for Work {
//    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//        write!(
//            f,
//            "{}, {}  from {} to  {} {}",
//            self.title, self.company, self.start_date, self.end_date, self.achievements
//        )
//    }
//}

#[derive(Serialize, Deserialize, Debug)]
pub struct Edu<'a> {
    pub degree: &'a str,
    pub institution: &'a str,
    pub start_date: &'a str,
    pub end_date: &'a str,
    pub gpa: &'a str,
    pub courses: Vec<&'a str>,
}

impl Display for Person<'_> {
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

impl Display for Edu<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}             from {} to  {}",
            self.institution, self.degree, self.start_date, self.end_date
        )
    }
}
