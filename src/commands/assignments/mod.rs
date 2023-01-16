use canvasapi::prelude::CanvasInformation;
use colored::Colorize;

use crate::commands::{print_assignments, search_courses};

use super::{get_base_url, get_canvas_token};

pub fn handle_command(course: &String, assignment: &Option<String>) {
    let base_url = &get_base_url().expect("Base URL not populated.");
    let canvas_token = &get_canvas_token().expect("Canvas Token not populated.");
    let canvas = CanvasInformation::new(base_url, canvas_token);

    let search = course.as_str();
    let binding = search_courses(Some(search.parse().unwrap()));
    let course = binding
        .first()
        .expect("Could not find any course with that search! Please try again.");

    let name = if let Some(name) = &course.name { name } else { "Unknown" };
    println!("{}", format!("{}{}{}", "Found a course with the name '".yellow(), name.green(), "'. Searching for assignments...".yellow()));

    let unfiltered = course
        .get_assignments()
        .unwrap()
        .fetch(&canvas)
        .unwrap()
        .inner();

    let assignments = match assignment {
        Some(keyword) => {
            unfiltered
                .into_iter()
                .filter(|a| a.name.is_some())
                .filter(|s| s.name.clone().unwrap().contains(keyword))
                .collect()
        },
        None => {
            unfiltered
        }
    };

    if assignments.len() < 1 {
        println!("{}", format!("{}", "No assignments found!".red()));
    } else {
        print_assignments(assignments, 10);
    }
}