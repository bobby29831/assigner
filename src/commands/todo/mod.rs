use canvasapi::prelude::{Canvas, CanvasInformation};
use colored::Colorize;

use super::{get_base_url, get_canvas_token};

pub fn handle_command() {
    let base_url = &get_base_url().expect("Base URL not populated.");
    let canvas_token = &get_canvas_token().expect("Canvas Token not populated.");
    let canvas = CanvasInformation::new(base_url, canvas_token);
    let todos = Canvas::get_todo_items().unwrap().fetch(&canvas).unwrap().inner();
    if todos.len() < 1 { println!("{}", format!("\u{2713} Nothing in todo right now! #winning").green()) } else {
        for todo in todos {
            let course = match todo.course_id {
                None => { None }
                Some(id) => { Some(Canvas::get_course(id as usize).unwrap().fetch(&canvas).unwrap().inner()) }
            };

            let course_name = course.unwrap().name.unwrap_or("Unknown Course".parse().unwrap());
            let assignment_name = todo.assignment.unwrap().name.unwrap_or("Unknown Assignment".parse().unwrap());

            println!("[{}] {}", course_name, assignment_name);
        }
    }
}