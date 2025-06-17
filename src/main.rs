use clap::Parser;
use regex::Regex;
use serde::Serialize;
use std::{
    fs::{self, File},
    io::{self, Write},
    path::{Path}
};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Directory to search files (defaults to current directory)
    #[arg(default_value = ".")]
    dir: String,

    /// Regex pattern to filter file names
    #[arg(long)]
    regex: Option<String>,

    /// Filter for file extensions (eg, "rs", "py", "R", "txt")
    #[arg(long)]
    ext: Option<String>,

    /// Output in JSON format (otherwise default format in .out text file)
    #[arg(long)]
    json: bool,
}

#[derive(Serialize)]
struct FileEntry {
    file: String,
    content: String,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let dir_path = Path::new(&args.dir);
    let regex = match &args.regex {
        Some(pattern) => Some(Regex::new(pattern).expect("Invalid regex")),
        None => None,
    };
    
    let mut entries = vec![];
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filename = match path.file_name().and_then(|n| n.to_str()) {
                Some(name) => name,
                None => continue,
            };
            if let Some(ref ext) = args.ext {
                if !path.extension().map_or(false, |e| *e == **ext) {
                    continue;
                }
            }
            if let Some(ref r) = regex {
                if !r.is_match(filename) {
                    continue;
                }
            }

            let content = fs::read_to_string(&path)?;
            entries.push(FileEntry {
                file: filename.to_string(),
                content,
            });
        }
    }

    let out_name = format!(
        "{}.out",
        dir_path.file_name().and_then(|s| s.to_str()).unwrap_or("output")
    );

    if args.json {
        let json = serde_json::to_string_pretty(&entries)?;
        fs::write(out_name, json)?;
    } else {
        let mut out = File::create(out_name)?;
        for entry in entries {
            writeln!(out, "#// {}\n{}", entry.file, entry.content)?;
        }
    }
    Ok(())
}
