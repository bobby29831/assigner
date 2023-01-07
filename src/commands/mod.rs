use canvasapi::prelude::Assignment;
use colored::Colorize;
use config::Config;
use regex::Regex;

pub mod assignments;
pub mod create;
pub mod todo;

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
