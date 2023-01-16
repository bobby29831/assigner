mod commands;

use std::fs::File;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "view assignments for a course", long_about = None)]
    Assignments {
        course: String,
        assignment: Option<String>
    },
    #[command(about = "create a document to submit as the assignment", long_about = None)]
    Create {
        name: String
    },
    #[command(about = "check assignments in your todo list", long_about = None)]
    Todo {  },
    #[command(about = "see a list of all of your courses & course ids", long_about = None)]
    Courses {
        search: Option<String>
    }
}

fn main() {
    let args = Args::parse();

    if File::open("./assigner-config.toml").is_err() {
        println!("It appears that this is your first time using assigner, or your configuration was deleted.");

        let mut base_url = String::new();
        println!("Because of this, please input your base canvas URL:");
        std::io::stdin().read_line(&mut base_url).expect("Failure to read canvas URL from input");

        let mut canvas_token = String::new();
        println!("Please input your canvas token:");
        std::io::stdin().read_line(&mut canvas_token).expect("Failure to read canvas URL from input");

        match File::create("./assigner-config.toml") {
            Ok(_) => {
                let data = format!("base_url = \"{}\"\ncanvas_token = \"{}\"", base_url.trim(), canvas_token.trim());
                std::fs::write("./assigner-config.toml", data).expect("Failure to write data to file.");
            }
            Err(_) => {
                println!("Failure to create assigner configuration file.")
            }
        }
    }

    match &args.command {
        Commands::Assignments { course, assignment } => commands::assignments::handle_command(course, assignment),
        Commands::Create { name } => commands::create::handle_command(name),
        Commands::Todo {  } => commands::todo::handle_command(),
        Commands::Courses { search } => commands::courses::handle_command(search)
    }
}