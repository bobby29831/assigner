use std::fs::File;
use canvasapi::prelude::{Canvas, CanvasInformation};

use super::{get_base_url, get_canvas_token, extract_course_and_assignment_ids};

pub fn handle_command(name: &String) {
    let base_url = &get_base_url().expect("Base URL not populated.");
    let canvas_token = &get_canvas_token().expect("Canvas Token not populated.");
    let canvas = CanvasInformation::new(base_url, canvas_token);
    if let Some((course_id, assignment_id)) = extract_course_and_assignment_ids(name) {
        println!("Searching for an assignment with the course ID {} and the assignment ID {}...", course_id, assignment_id);
        let course = Canvas::get_course(course_id as usize).unwrap().fetch(&canvas).unwrap().inner();
        let assignment = course.get_assignment(assignment_id as usize).unwrap().fetch(&canvas).unwrap().inner();

        if let Some(name) = assignment.name {
            println!("Found assignment with name {}, attempting to create a file called {}.txt", name, name);
            File::create(name + ".txt").expect("Failed to create the file.");
        } else {
            println!("Assignment not found.");
        }
    }
}