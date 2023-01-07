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
    Assignments {
        course_url: String,
        search: Option<String>
    },
    Create {
        name: String
    },
    Todo {  }
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
        Commands::Assignments { course_url, search } => commands::assignments::handle_command(course_url, search),
        Commands::Create { name } => commands::create::handle_command(name),
        Commands::Todo {  } => commands::todo::handle_command()
    }
}