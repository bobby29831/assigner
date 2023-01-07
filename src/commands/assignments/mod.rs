use canvasapi::prelude::{Canvas, CanvasInformation};

use crate::commands::print_assignments;

use super::{get_base_url, get_canvas_token, extract_course_id};

pub fn handle_command(course_url: &String, search: &Option<String>) {
    let base_url = &get_base_url().expect("Base URL not populated.");
    let canvas_token = &get_canvas_token().expect("Canvas Token not populated.");
    let canvas = CanvasInformation::new(base_url, canvas_token);
    if let Some(course_id) = extract_course_id(course_url) {
        println!("Searching for a course with the ID {}...", course_id);
        let course = Canvas::get_course(course_id as usize).unwrap().fetch(&canvas).unwrap().inner();
        let assignments = course.get_assignments().unwrap().fetch(&canvas).unwrap().inner();
        match search {
            Some(keyword) => {
                let filtered_assignments = assignments
                            .into_iter()
                            .filter(|a| a.name.is_some())
                            .filter(|s| s.name.clone().unwrap().contains(keyword))
                            .collect();
                print_assignments(filtered_assignments, 10);
            },
            None => {
                print_assignments(assignments, 10);
            }
        }
    }
}