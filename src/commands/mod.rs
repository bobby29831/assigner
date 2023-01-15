use canvasapi::canvas::CanvasInformation;
use canvasapi::prelude::{Assignment, Course};
use colored::Colorize;
use config::Config;
use regex::Regex;

pub mod assignments;
pub mod create;
pub mod todo;
pub mod courses;

fn get_settings_config() -> Config {
    Config::builder()
        .add_source(config::File::with_name("./assigner-config"))
        .build()
        .unwrap()
}

fn get_base_url() -> Option<String> {
    match get_settings_config().get_string("base_url") {
        Ok(url) => { Some(url) }
        Err(_) => { None }
    }
}

fn get_canvas_token() -> Option<String> {
    match get_settings_config().get_string("canvas_token") {
        Ok(url) => { Some(url) }
        Err(_) => { None }
    }
}

fn extract_course_id(input: &str) -> Option<u32> {
    if let Some(base_url) = get_base_url() {
        let pattern = Regex::new(r"^courses/(\d+)$").unwrap();
        let replaced = input.replace(&*base_url, "");
        let captures = pattern.captures(&*replaced)?;
        let course_id = captures[1].parse::<u32>().unwrap();
        return Some(course_id);
    }
    None
}

fn extract_course_and_assignment_ids(input: &str) -> Option<(u32, u32)> {
    if let Some(base_url) = get_base_url() {
        let pattern = Regex::new(r"^courses/(\d+)/assignments/(\d+)$").unwrap();
        let replaced = input.replace(&*base_url, "");
        let captures = pattern.captures(&*replaced)?;
        let course_id = captures[1].parse::<u32>().unwrap();
        let assignment_id = captures[2].parse::<u32>().unwrap();
        return Some((course_id, assignment_id));
    }
    None
}

fn print_assignments(assignments: Vec<Assignment>, max: usize) {
    for i in 0..assignments.len() {
        if i > (max - 1) { break; }
        let assignment = assignments.get(i).unwrap();
        let submitted = if let Some(submissions) = assignment.has_submitted_submissions { submissions } else { false };
        if let Some(name) = &assignment.name {
            if submitted {
                println!("({}) {} {}", i + 1, name, format!("\u{2713}").green());
            } else {
                println!("({}) {}", i + 1, name);
            }
        } else {
            if submitted {
                println!("({}) Unknown Assignment {}", i + 1, format!("\u{2713}").green());
            } else {
                println!("({}) Unknown Assignment", i + 1);
            }
        }
    }
}

fn search_courses(search: Option<String>) -> Vec<Course> {
    let base_url = &get_base_url().expect("Base URL not populated.");
    let canvas_token = &get_canvas_token().expect("Canvas Token not populated.");
    let canvas = CanvasInformation::new(base_url, canvas_token);
    let courses = Course::courses().unwrap().fetch(&canvas).unwrap().inner();
    let pattern = search.unwrap_or("".parse().unwrap());

    let mut filtered: Vec<Course> = Vec::new();
    for course in courses {
        if let Some(name) = &course.name {
            if name.contains(&pattern) { filtered.push(course) }
        } else {
            continue;
        }
    };
    return filtered;
}

// fn print_in_box(lines: Vec<&str>, padding: Option<usize>) {
//     let extra = padding.unwrap_or(1) * 2;
//     let longest = lines.iter().fold(lines[0], |acc, &item| {
//         if item.len() > acc.len() { item } else { acc }
//     });
//     let length = longest.len() + extra;
//     println!("{}{}{}", "\u{250C}", "\u{2500}".repeat(length), "\u{2510}");
//     lines.iter().for_each(|l| println!("{}{:^length$}{}", "\u{2502}", l, "\u{2502}"));
//     println!("{}{}{}", "\u{2514}", "\u{2500}".repeat(length), "\u{2518}");
// }