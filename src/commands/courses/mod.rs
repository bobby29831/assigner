use colored::Colorize;
use crate::commands::search_courses;

pub fn handle_command(search: &Option<String>) {
    let courses = search_courses(search.to_owned());
    let largest = courses.iter().map(|c| c.id).max();
    for course in courses {
        if let Some(name) = course.name {
            let length = largest.unwrap_or(1000).to_string().len();
            let id = if course.id.to_string().len() < length { course.id.to_string() + &*" ".repeat(length - course.id.to_string().len()) } else { course.id.to_string() };
            println!("{}", format!("{}{}{}", id.bright_black(), " | ".dimmed(), name.bright_yellow()));
        }
    }
}