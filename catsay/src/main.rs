/*
page 10

Features:

• Take a message string as the positional argument.4
• Take a -h/--help flag to print a help message.
• Take a -d/--dead flag that makes the cat’s eyes become xx, which is
the comical expression of dead eyes.
• Print in color.
• Print the error message to STDERR for error handling.
• Accept STDIN for piping input and pipe the output to other
programs.
• Perform integration tests.
• Package and publish to crates.io.


Hello I'm a cat
 \
  \
    /\_/\
   ( o o )
   =( I )=

*/

use std::io::{self, Read};

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;

#[derive(Parser)]
struct Options {
    #[clap(default_value = "Meow!")]
    /// What does the cat say?
    message: String,
    #[clap(short = 'd', long = "dead")]
    /// Make the cat appear dead
    dead: bool,
    #[clap(short = 'f', long = "file")]
    /// Load the cat picture from the specified file
    catfile: Option<std::path::PathBuf>,
    #[clap(short = 'i', long = "stdin")]
    /// Read the message from STDIN instead of the argument
    stdin: bool,
}

fn main() -> Result<()> {
    let options = Options::parse();
    let mut message = String::new();
    if options.stdin {
        io::stdin().read_to_string(&mut message)?;
        message = message.trim_end().to_string();
    } else {
        message = options.message;
    }

    let eye = if options.dead { "x" } else { "o" };

    match &options.catfile {
        Some(path) => {
            let cat_template = std::fs::read_to_string(path)
                .with_context(|| format!("Could not read file {:?}", path))?;
            let eye = format!("{}", eye.red().bold());
            let cat_picture = cat_template.replace("{eye}", &eye);
            println!("{}", message.bright_yellow().underline().on_purple());
            println!("{}", cat_picture);
        }
        None => {
            println!("{}", message.bright_yellow().underline().on_purple());
            println!(" \\");
            println!("  \\");
            println!("      /\\_/\\");
            println!("     ( {eye} {eye} )", eye = eye.red().bold());
            println!("     =( I )=");
        }
    }

    Ok(())
}
