use std::fs::File;
use canvasapi::canvas::CanvasInformation;
use canvasapi::prelude::{Assignment, Canvas, Course};
use clap::{Parser, Subcommand};
use colored::Colorize;
use regex::Regex;

// token: 5359~DyffhDbpeX2h7hFTF77owWNgHU6tStx6JZniDoAplFOC8lWCRJqv66rnSXSZ5YfK

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Create {
        name: String
    },
    Assignments {
        course_url: String,
        search: Option<String>
    },
    Todo {  }
}

fn main() {
    let args = Args::parse();

    let base_url = "https://sluh.instructure.com/";
    let canvas_token = "5359~DyffhDbpeX2h7hFTF77owWNgHU6tStx6JZniDoAplFOC8lWCRJqv66rnSXSZ5YfK";

    let canvas = CanvasInformation::new(base_url, canvas_token);

    match &args.command {
        Commands::Create { name } => {
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

        Commands::Assignments { course_url, search } => {
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

        Commands::Todo {  } => {
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
    }
}

fn extract_course_and_assignment_ids(input: &str) -> Option<(u32, u32)> {
    let pattern = Regex::new(r"^https://sluh\.instructure\.com/courses/(\d+)/assignments/(\d+)$").unwrap();
    let captures = pattern.captures(input)?;
    let course_id = captures[1].parse::<u32>().unwrap();
    let assignment_id = captures[2].parse::<u32>().unwrap();
    Some((course_id, assignment_id))
}

fn extract_course_id(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"^https://sluh\.instructure\.com/courses/(\d+)$").unwrap();
    let captures = pattern.captures(input)?;
    let course_id = captures[1].parse::<u32>().unwrap();
    Some(course_id)
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

fn print_courses(courses: Vec<Course>, max: usize) {
    for i in 0..courses.len() {
        if i > (max - 1) { break; }
        let course = courses.get(i).unwrap();
        if let Some(name) = &course.name {
            println!("({}) {}", i + 1, name)
        } else {
            println!("({}) Unknown Course", i + 1)
        }
    }
}