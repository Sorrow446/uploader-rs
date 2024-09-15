use std::collections::HashMap;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use clap::Parser;

mod utils;
mod structs;
mod client;
mod hosts;

use structs::Args;
use crate::client::UploaderClient;
use crate::hosts::enums::Host;
use crate::hosts::errors::{SectionKeyMissingError, SectionValueEmptyError};

type RunFuncType = fn(
    &mut UploaderClient, &PathBuf,
    usize, &HashMap<String, HashMap<String, String>>,
    &mut HashMap<String, HashMap<String, String>>
) -> Result<String, Box<dyn Error>>;
pub fn make_func_map() -> HashMap<Host, RunFuncType> {
    let mut m: HashMap<Host, RunFuncType> = HashMap::new();

    m.insert(Host::Fileio, hosts::fileio::run);
    m.insert(Host::Gofile, hosts::gofile::run);
    m.insert(Host::Pixeldrain, hosts::pixeldrain::run);
    m
}

fn template_file_setup(path: &PathBuf, wipe: bool) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    let f = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)?;

    if wipe {
        f.set_len(0)?;
    }

    Ok(())
}

fn parse_args() -> Result<Args, Box<dyn Error>> {
    let mut args = Args::parse();

    if args.file_paths.len() == 0 && args.directories.len() == 0 {
        return Err("file path and/or directory required".into())
    }

    if args.directories.len() > 0 {
        utils::process_dirs(&mut args)?;
    }
    let paths = utils::filter_paths(args.file_paths)?;
    if paths.len() == 0 {
        return Err("all paths were filtered".into())
    }

    args.file_paths = paths;

    if let Some(out_path) = &args.out_path {
        if !out_path.is_absolute() {
            let abs_path = utils::get_abs_path(out_path)?;
            args.out_path = Some(abs_path);
        }
    }
    Ok(args)
}

// Too many args, clean up.
fn generate_template(mut template: String, filename: &str, file_path: &PathBuf, file_url: &str, host: &str) -> Vec<u8> {
    template = template.replace("<filename>", filename);
    template = template.replace("<file_path>", &file_path.to_string_lossy());
    template = template.replace("<url>", file_url);
    template = template.replace("<host>", host);

    let newline = if cfg!(windows) {
        "\r\n"
    } else {
        "\n"
    };

    let template = template.replace("<newline>", newline);
    template.into_bytes()
}

fn write_template(template: String, out_path: &PathBuf, file_path: &PathBuf, filename: &str, url: &str, host: &str) -> Result<(), io::Error>  {
    let data = generate_template(template, &filename, file_path, &url, &host);
    let canon_path = utils::normalise_path_case(out_path)?;

    let mut f = OpenOptions::new()
        .append(true)
        .write(true)
        .open(canon_path)?;

    f.write_all(&data)?;

    Ok(())
}

fn read_config() -> Result<HashMap<String, HashMap<String, String>>, Box<dyn Error>> {
    let exe_path = utils::get_exe_path()?;
    let config_path = exe_path.join("config.toml");
    let data = fs::read_to_string(config_path)?;
    let config: HashMap<String, HashMap<String, String>> = toml::from_str(&data)?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut sessions_map: HashMap<String, HashMap<String, String>> = HashMap::new();
    let func_map = make_func_map();

    let args = parse_args()?;
    let file_total = args.file_paths.len();

    let mut uploader_client = UploaderClient::new()?;

    if let Some(out_path) = &args.out_path {
        template_file_setup(out_path, args.wipe)?;
    }

    let cfg = read_config()?;

    let host_total =  args.hosts.len();
    'outer: for (mut host_num, host) in args.hosts.iter().enumerate() {
        host_num += 1;
        println!("Host {} of {}:", host_num, host_total);
        println!("{}", host.to_string());

        for (mut file_num, p) in args.file_paths.iter().enumerate() {
            file_num += 1;

            println!("File {} of {}:", file_num, file_total);
            let filename = utils::get_fname_string_from_path(p)?;
            let file_size = utils::get_file_size(&p)?;

            utils::print_path_no_prefix(p);
            let url = match func_map[&host](&mut uploader_client, p, file_size, &cfg, &mut sessions_map) {
                Ok(url) => url,
                Err(err) => {
                    println!("Upload failed.\n{:?}", err);
                    if err.downcast_ref::<SectionKeyMissingError>().is_some() ||
                        err.downcast_ref::<SectionValueEmptyError>().is_some() {
                        break 'outer
                    }
                    continue;
                },
            };

            println!("{}", url);
            if let Some(out_path) = &args.out_path {
                write_template(args.template.clone(), out_path, p, &filename, &url, &host.to_string())?;
            }

        }
    }

    Ok(())
}
