use chrono::{DateTime, Local};
use shellexpand;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process;

const VERSION: &str = "0.1.0";

fn usage() {
    eprintln!("dtren v{}\n", VERSION);
    eprintln!("Usage: dtren <file>\n\nRename a file by appending its modified timestamp to the name.");
}

fn error_exit<S: AsRef<str>>(msg: S, code: i32) -> ! {
    eprintln!("error: {}", msg.as_ref());
    process::exit(code);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("error: missing file argument");
        usage();
        process::exit(2);
    }

    let arg = &args[1];
    if arg == "-h" || arg == "--help" {
        usage();
        process::exit(1);
    }

    // Expand ~ and environment variables (tilde expansion)
    let expanded = shellexpand::tilde(arg).to_string();
    let path = PathBuf::from(expanded);

    // Check existence and that it's a regular file (not a dir or symlink)
    let meta = match fs::symlink_metadata(&path) {
        Ok(m) => m,
        Err(_) => error_exit(format!("path does not exist: {}", path.display()), 2),
    };

    if meta.file_type().is_symlink() {
        error_exit(format!("path is a symlink (not allowed): {}", path.display()), 2);
    }

    if meta.is_dir() {
        error_exit(format!("path is a directory (not a file): {}", path.display()), 2);
    }

    if !meta.is_file() {
        error_exit(format!("path is not a regular file: {}", path.display()), 2);
    }

    // Resolve absolute path
    let abs_path = match fs::canonicalize(&path) {
        Ok(p) => p,
        Err(e) => error_exit(format!("failed to resolve absolute path {}: {}", path.display(), e), 2),
    };

    // Get modified time
    let metadata = match fs::metadata(&abs_path) {
        Ok(m) => m,
        Err(e) => error_exit(format!("failed to read metadata for {}: {}", abs_path.display(), e), 2),
    };

    let mtime = match metadata.modified() {
        Ok(t) => t,
        Err(e) => error_exit(format!("failed to get modification time for {}: {}", abs_path.display(), e), 2),
    };

    let datetime: DateTime<Local> = DateTime::from(mtime);
    let date_time_str = datetime.format("%Y%m%d_%H%M%S").to_string();

    // Build new file name: stem + '-' + {date_time} + optional .extension
    let stem = abs_path
        .file_stem()
        .and_then(OsStr::to_str)
        .unwrap_or_else(|| error_exit(format!("invalid file name: {}", abs_path.display()), 2));

    let new_name = match abs_path.extension().and_then(OsStr::to_str) {
        Some(ext) if !ext.is_empty() => format!("{}-{}.{}", stem, date_time_str, ext),
        _ => format!("{}-{}", stem, date_time_str),
    };

    let parent = abs_path.parent().unwrap_or(Path::new("."));
    let new_path = parent.join(new_name);

    if new_path.exists() {
        error_exit(format!("destination path already exists: {}", new_path.display()), 2);
    }

    // Perform rename
    if let Err(e) = fs::rename(&abs_path, &new_path) {
        error_exit(format!("failed to rename {} -> {}: {}", abs_path.display(), new_path.display(), e), 2);
    }

    // Success: exit 0
}
