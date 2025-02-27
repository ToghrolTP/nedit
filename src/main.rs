use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use colored::Colorize;

// UI Functions
mod ui {
    use super::*;

    pub fn show_header() {
        println!("{} PREFIX REMOVAL TOOL {}", "--===".yellow(), "===--".yellow());
        println!("This program will remove prefixes such as 'spotidownloader.com - ' from your music files.");
        println!();
    }

    pub fn get_prefix() -> io::Result<String> {
        println!("Type the prefix: ");
        let mut prefix = String::new();
        io::stdin()
            .read_line(&mut prefix)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Failed to read prefix: {}", e)))?;

        Ok(prefix.trim().to_string())
    }

    pub fn show_preview(files: &[fs::DirEntry], prefix: &str) {
        println!("Preview of changes:");
        for file in files {
            let (old_name, new_name, _) = file_ops::generate_new_path(file, prefix);
            println!("\"{}\" -> \"{}\"", old_name, new_name);
        }
        println!();
    }

    pub fn confirm_operation() -> io::Result<bool> {
        print!("Do you want to proceed with renaming these files? (y/n): ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        Ok(["y", "Y"].contains(&input.trim()))
    }

    pub fn show_summary(success: usize, total: usize) {
        println!();
        println!("=== SUMMARY ===");
        println!("Successfully renamed {} of {} files.", success, total);
        println!("Operation complete!");
    }
}




mod file_ops {
    use super::*;

    // File Operation Functions
    pub fn find_files_with_prefix(dir: &Path, prefix: &str) -> io::Result<Vec<fs::DirEntry>> {
        let entries = fs::read_dir(dir)?;

        let files: Vec<_> = entries
            .filter_map(Result::ok)  // This was missing in your code
            .filter(|entry| {
                entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
                    && entry
                        .file_name()
                        .to_string_lossy()
                        .starts_with(prefix)
            })
            .collect();

        Ok(files)
    }

    pub fn generate_new_path(file: &fs::DirEntry, prefix: &str) -> (String, String, PathBuf) {
        let old_path = file.path();
        let old_name = file.file_name().to_string_lossy().to_string();
        let new_name = old_name.replace(prefix, "");
        let new_path = old_path.with_file_name(&new_name);

        (old_name, new_name, new_path)
    }

    pub fn rename_file(file: &fs::DirEntry, prefix: &str) -> io::Result<(String, String)> {
        let (old_name, new_name, new_path) = generate_new_path(file, prefix);

        fs::rename(&file.path(), &new_path)?;

        Ok((old_name, new_name))
    }
}


// Main Function
fn main() -> io::Result<()> {
    ui::show_header();

    // Get prefix from user
    let prefix = ui::get_prefix()?;

    // Get files with prefix
    let current_dir = env::current_dir()?;
    let files = file_ops::find_files_with_prefix(&current_dir, &prefix)?;

    let count = files.len();
    if count == 0 {
        println!("No files with '{}' prefix found in the current directory.", prefix);
        return Ok(());
    }

    println!("Found {} files to rename.", count);
    println!();

    // Show preview
    ui::show_preview(&files, &prefix);

    // Confirm operation
    if !ui::confirm_operation()? {
        println!("Operation cancelled.");
        return Ok(());
    }

    // Rename files
    let mut success = 0;
    for file in files {
        match file_ops::rename_file(&file, &prefix) {
            Ok((old_name, new_name)) => {
                println!("Renamed '{}' -> '{}'", old_name, new_name);
                success += 1;
            }
            Err(e) => println!("Error renaming '{}': {}", file.file_name().to_string_lossy(), e),
        }
    }

    ui::show_summary(success, count);

    Ok(())
}
