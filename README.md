# nedit - Name Editor

A simple command-line tool for batch renaming files by removing specified prefixes.

## Overview

`nedit` is a utility that helps you clean up file names by removing unwanted prefixes. It's particularly useful for cleaning up downloaded files that may have website names or other unwanted text at the beginning of their filenames.

## Features

- Interactive prefix removal
- Preview of changes before execution
- Confirmation prompt before making changes
- Summary report of operations
- Safe operation with error handling
- Works in the current directory

## Installation

### From Source

1. Make sure you have Rust installed on your system. If not, install it from [rustup.rs](https://rustup.rs/)

2. Clone the repository:
```bash
git clone https://github.com/toghroltp/nedit.git
cd nedit
```

3. Build the project:
```bash
cargo build --release
```

4. The executable will be available in `target/release/nedit`

## Usage

1. Navigate to the directory containing the files you want to rename:
```bash
cd path/to/your/files
```

2. Run nedit:
```bash
nedit
```

3. When prompted, enter the prefix you want to remove from the filenames

4. Review the preview of changes

5. Confirm the operation by typing 'y' when prompted

### Example

```
=== PREFIX REMOVAL TOOL ===
This script will remove prefixes such as 'spotidownloader.com - ' from your music files.

Type the prefix:
spotidownloader.com -

Found 3 files to rename.

Preview of changes:
"spotidownloader.com - Song1.mp3" -> "Song1.mp3"
"spotidownloader.com - Song2.mp3" -> "Song2.mp3"
"spotidownloader.com - Song3.mp3" -> "Song3.mp3"

Do you want to proceed with renaming these files? (y/n):
```

## Safety Features

- Preview of all changes before execution
- Confirmation required before making changes
- Error handling for file operations
- Operation summary showing success/failure counts

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the GPL3 License - see the LICENSE file for details.

## Notes

- The tool only operates on files in the current directory
- It does not process files in subdirectories
- Make sure you have write permissions in the directory where you're running the tool

## Support

If you encounter any issues or have suggestions, please open an issue on the GitHub repository.
