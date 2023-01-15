use canvasapi::canvas::CanvasInformation;
use canvasapi::prelude::{Course};
use colored::Colorize;
use crate::commands::{get_base_url, get_canvas_token};

pub fn handle_command() {
    let base_url = &get_base_url().expect("Base URL not populated.");
    let canvas_token = &get_canvas_token().expect("Canvas Token not populated.");
    let canvas = CanvasInformation::new(base_url, canvas_token);

    let courses = Course::courses().unwrap().fetch(&canvas).unwrap().inner();
    let largest = courses.iter().map(|c| c.id).max();
    for course in courses {
        if let Some(name) = course.name {
            let length = largest.unwrap_or(1000).to_string().len();
            let id = if course.id.to_string().len() < length { course.id.to_string() + &*" ".repeat(length - course.id.to_string().len()) } else { course.id.to_string() };
            println!("{}", format!("{}{}{}", id.bright_black(), " | ".dimmed(), name.bright_yellow()));
        }
    }
}