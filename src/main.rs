

use clap::Parser;
use colored::Colorize;
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Message to display")]
    message: String,
    #[arg(short, long, help = "Is Stormy alive or dead?", default_value = "false")]
    dead: bool,
    #[arg(short = 'f', long = "file", help = "File to read message from")]
    catfile: Option<std::path::PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let args = Args::parse();
    let message = args.message;
    let eye = if args.dead { "x" } else { "o" };
    let d_or_a = if args.dead { "Stormy is dead" } else { "Stormy is alive" };
    let greeting = "Stormy says: ";
        if message.to_lowercase() == "woof" {
            let err: &str = "Cats don't say woof!";
            eprintln!("{} {}", greeting.red().bold(), err.red());
        } else {
        println!("{} {}", greeting.blue().bold(), message.green());
        }

    match &args.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
            .with_context(|| format!("Dude!!! Fix the file location!! {:?}", path))?;
            let eye: String = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!(
                "{}",
                message.bright_yellow().underline().on_purple()
            );
            println!("{}", &cat_picture);
        }, None => {
                println!(" \\");
                println!("  \\");
                println!("     /\\_/\\");
                println!("    ( {eye}.{eye} )", eye = eye);
                println!("    =( I )=");

                println!("\n{}", d_or_a);
                
            }
    }
    Ok(())
}