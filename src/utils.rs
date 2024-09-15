use std::collections::HashSet;
use std::error::Error;
use std::{env, fs};
use std::io::{self, Error as IoError};
use std::path::PathBuf;
use crate::structs::Args;

pub fn get_exe_path() -> Result<PathBuf, Box<dyn Error>> {
    let exe_path = env::current_exe()?;
    let parent_dir = exe_path.parent()
        .ok_or("failed to get path of executable")?;
    let exe_path_buf = PathBuf::from(parent_dir);
    Ok(exe_path_buf)
}

pub fn get_file_size(file_path: &PathBuf) -> io::Result<usize> {
    let metadata = fs::metadata(file_path)?;
    Ok(metadata.len() as usize)
}

pub fn get_fname_string_from_path(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .ok_or_else(|| "Path does not have a filename.".into())
}

fn populate_dirs(dir: &PathBuf, recursive: bool) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut files: Vec<PathBuf> = Vec::new();

    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() && recursive {
                let child_dirs = populate_dirs(&path, true)?;
                files.extend(child_dirs);
            } else if path.is_file() {
                files.push(path.clone());
            }
        }
    }

    Ok(files)
}

fn check_exists(file_path: &PathBuf, is_dir: bool) -> Result<bool, IoError> {
    match fs::metadata(file_path) {
        Ok(meta) => {
            Ok(meta.is_dir() == is_dir)
        },
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(err)
            }
        }
    }
}

pub fn process_dirs(args: &mut Args) -> Result<(), Box<dyn Error>> {
    let mut all_dirs: HashSet<PathBuf> = HashSet::new();
    let mut pop_paths: Vec<PathBuf>;

    for dir in &args.directories {
        let exists = check_exists(dir, true)?;

        if exists {
            if !all_dirs.contains(dir) {
                all_dirs.insert(dir.clone());

                pop_paths = populate_dirs(dir, args.recursive)?;

                args.file_paths.extend(pop_paths);
            } else {
                println!("Filtered duplicate directory: {}", dir.display());
            }
        } else {
            println!("Filtered non-existent directory: {}", dir.display());
        }
    }

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn normalise_path_case(path: &PathBuf) -> Result<PathBuf, io::Error> {
    let canon_path = fs::canonicalize(path)?;
    Ok(canon_path)
}

#[cfg(not(target_os = "windows"))]
pub fn normalise_path_case(path: &PathBuf) -> Result<PathBuf, io::Error> {
    Ok(path.to_path_buf())
}

pub fn get_abs_path(path: &PathBuf) -> io::Result<PathBuf> {
    let cd =  env::current_dir()?;
    let abs_path = cd.join(path);
    Ok(abs_path)
}

pub fn filter_paths(paths: Vec<PathBuf>) -> Result<Vec<PathBuf>, io::Error> {
    let mut filtered_paths: Vec<PathBuf> = Vec::new();

    let mut seen_paths: HashSet<PathBuf> = HashSet::new();
    for mut path in paths {
        if !path.is_absolute() {
            path = get_abs_path(&path)?;
        }

        if check_exists(&path, false)? {
            let norm_path = match normalise_path_case(&path) {
                Ok(p) => p,
                Err(e) => {
                    println!("Failed to normalise path; dropped.\n{:?}", e);
                    continue;
                }
            };
            if !seen_paths.contains(&norm_path) {
                seen_paths.insert(norm_path.clone());
                filtered_paths.push(norm_path);
            } else {
                println!("Filtered duplicate file: {:?}", path.display());
            }
        } else {
            println!("Filtered non-existent file: {:?}", path.display());
        }
    }
    Ok(filtered_paths)
}

pub fn print_path_no_prefix(path: &PathBuf) {
    let p = path.to_string_lossy().to_string();
    let trimmed_path = if p.starts_with(r"\\?\") {
        &p[4..]
    } else {
        &p
    };
    println!("{}", trimmed_path);
}