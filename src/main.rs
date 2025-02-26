use std::io::{self, Write};
use std::fs;

fn main() -> io::Result<()> {
    println!("=== PREFIX REMOVAL TOOL ===");
    println!("This script will remove prefixes such as 'spotidownloader.com - ' from your music files.");
    println!();

    println!("Type the prefix: ");
    let mut prefix = String::new();
    io::stdin()
        .read_line(&mut prefix)
        .expect("Failed to read prefix");
    let prefix = prefix.trim();

    // Get all files in current directory
    let current_dir = std::env::current_dir()?;
    let entries = fs::read_dir(&current_dir)?;

    // Filter and collect files with prefix
    let files: Vec<_> = entries
        .filter_map(Result::ok)
        .filter(|entry| {
            entry.file_type().map(|ft| ft.is_file()).unwrap_or(false)
                && entry
                    .file_name()
                    .to_string_lossy()
                    .starts_with(prefix)
        })
        .collect();

    let count = files.len();
    if count == 0 {
        println!("No files with '{}' prefix found in the current directory.", prefix);
        return Ok(());
    }

    println!("Found {} files to rename.", count);
    println!();

    // Show preview
    println!("Preview of changes:");
    for file in &files {
        let filename = file.file_name();
        let old_name = filename.to_string_lossy();
        let new_name = old_name.replace(prefix, "");
        println!("\"{}\" -> \"{}\"", old_name, new_name);
    }
    println!();

    // Ask for confirmation
    print!("Do you want to proceed with renaming these files? (y/n): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    if !["y", "Y"].contains(&input.trim()) {
        println!("Operation cancelled.");
        return Ok(());
    }

    // Rename files
    let mut success = 0;
    for file in files {
        let old_path = file.path();
        let filename = file.file_name();
        let old_name = filename.to_string_lossy();
        let new_name = old_name.replace(prefix, "");
        let new_path = old_path.with_file_name(&new_name);

        match fs::rename(&old_path, &new_path) {
            Ok(_) => {
                println!("Renamed '{}' -> '{}'", old_name, new_name);
                success += 1;
            }
            Err(e) => println!("Error renaming '{}': {}", old_name, e),
        }
    }

    println!();
    println!("=== SUMMARY ===");
    println!("Successfully renamed {} of {} files.", success, count);
    println!("Operation complete!");

    Ok(())
}
