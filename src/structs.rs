use std::path::PathBuf;
use clap::Parser;
use crate::hosts::enums::Host;

#[derive(Parser)]
#[command(name = "uploader.rs")]
pub struct Args {
    #[clap(short, long, num_args = 1..)]
    pub directories: Vec<PathBuf>,

    #[clap(short, long, num_args = 1..)]
    pub file_paths: Vec<PathBuf>,

    #[clap(long, required = true, num_args = 1..)]
    pub hosts: Vec<Host>,

    #[clap(short, long, help = "Output template path.")]
    pub out_path: Option<PathBuf>,

    #[clap(short, long, help = "Include subdirectories.")]
    pub recursive: bool,

    #[clap(short, long, default_value = "<url><newline>", help = "Output template. Vars: filename, file_path, host, newline, url.")]
    pub template: String,

    #[clap(short, long, help = "Wipe output template on startup.")]
    pub wipe: bool,
}