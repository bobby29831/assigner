mod commands;

use std::fs;

use clap::{Parser, Subcommand};
use colored::Colorize;
use directories::ProjectDirs;

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
    let proj_dirs = get_proj_dirs();
    if !proj_dirs.config_dir().exists() {
        println!("Configuration folder doesn't currently exist... attempting to make one...");
        match fs::create_dir_all(proj_dirs.config_dir()) {
            Ok(_) => {
                println!("{}", format!("Successfully created configuration folder!").green());
                write_credentials().expect("Failed to write credentials :(");
            },
            Err(_) => {
                println!("{}", format!("Could not successfully create configuration folder!").red())
            }
        }
    }

    let args = Args::parse();

    match &args.command {
        Commands::Assignments { course, assignment } => commands::assignments::handle_command(course, assignment),
        Commands::Create { name } => commands::create::handle_command(name),
        Commands::Todo {  } => commands::todo::handle_command(),
        Commands::Courses { search } => commands::courses::handle_command(search)
    }
}

fn prompt_credentials() -> (String, String) {
    let mut base_url = String::new();
    println!("Because of this, please input your base canvas URL:");
    std::io::stdin().read_line(&mut base_url).expect("Failure to read canvas URL from input");

    let mut canvas_token = String::new();
    println!("Please input your canvas token:");
    std::io::stdin().read_line(&mut canvas_token).expect("Failure to read canvas URL from input");

    (base_url, canvas_token)
}

fn write_credentials() -> Result<(), String> {
    let proj_dirs = get_proj_dirs();

    let (base_url, canvas_token) = prompt_credentials();
    let data = format!("base_url = \"{}\"\ncanvas_token = \"{}\"", base_url.trim(), canvas_token.trim());

    if let Some(dir_path) = proj_dirs.config_dir().to_str() {
        let path = dir_path.to_string() + "/config.toml";
        let file = fs::File::create(&path).expect("Unable to find file reference!");
        fs::File::set_len(&file, 0).expect("Unable to clear file!");
        fs::write(path, data).expect("Failed to write to config file.");
        Ok(())
    } else {
        Err("Could not write information to file for some reason...".to_string())
    }
}

fn get_proj_dirs() -> ProjectDirs {
    ProjectDirs::from("com", "bobby29831", "Canvas-CLI").expect("Could not get 'proj_dirs' for some reason...")
}