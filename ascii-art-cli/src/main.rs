use std::{io, path::PathBuf, u32};

use anyhow::{Result, ensure};
use ascii_art::image_to_ascii;
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{Generator, Shell, generate};
use colored::{ColoredString, Colorize};

#[derive(Debug, Parser)]
#[command(version,about,long_about=None,subcommand_required(true),arg_required_else_help(true))]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand, Debug)]
enum Commands {
    #[cfg(target_os = "linux")]
    Completion { shell: Shell },
    Transform {
        path: PathBuf,
        #[arg(short = 'H', long = "height", help = "Maximum characters in a line")]
        max_height: Option<u32>,
        #[arg(short = 'W', long = "width", help = "Maximum rows")]
        max_width: Option<u32>,
        #[arg(short = 'c', long = "color", help = "Color pixels")]
        colored: bool,
    },
}

#[cfg(target_os = "linux")]
fn print_completions<G: Generator>(generator: G, cmd: &mut Command) {
    generate(generator, cmd, "todo", &mut io::stdout());
}

fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        #[cfg(target_os = "linux")]
        Some(Commands::Completion { shell }) => cmd_completion(shell),
        Some(Commands::Transform {
            path,
            max_height,
            max_width,
            colored,
        }) => cmd_transform(path, max_height, max_width, colored),
        None => todo!(),
    }
}

#[cfg(target_os = "linux")]
fn cmd_completion(shell: &Shell) -> Result<()> {
    print_completions(*shell, &mut Args::command());
    Ok(())
}
fn cmd_transform(
    path: &PathBuf,
    max_height: &Option<u32>,
    max_width: &Option<u32>,
    colored: &bool,
) -> Result<()> {
    ensure!(path.is_file(), "Invalid path. Path must point to an image");
    let mut image = image::open(path)?;
    if max_height.is_some() || max_width.is_some() {
        image = image.resize(
            max_width.unwrap_or(u32::MAX),
            max_height.unwrap_or(u32::MAX),
            image::imageops::FilterType::Lanczos3,
        );
    }
    let image = image.to_rgba8();
    let res = image_to_ascii(&image);
    for row in res {
        for (character, color) in row {
            let mut c: ColoredString = character.to_string().repeat(2).into();
            if *colored {
                c = c.truecolor(color[0], color[1], color[2]);
            }
            print!("{}", c);
        }
        println!();
    }
    Ok(())
}
