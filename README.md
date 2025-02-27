# Rust Shell Implementation

A lightweight Unix-style shell implementation written in Rust that provides essential shell functionality with robust error handling and a clean interface.

## Features

- Interactive command prompt with Unix-style interface (`$`)
- Core built-in commands implementation 
- File and directory operations with robust error handling
- Support for combined flags (e.g. `ls -la`)
- ANSI terminal support for clear screen
- Sorted directory listings
- Proper file permission handling
- File size and timestamp display
- Support for recursive operations
- Path handling for both files and directories

## Built-in Commands

| Command | Description | Usage | Example |
|---------|-------------|--------|---------|
| `ls` | List directory contents | `ls [-l] [-a] [-F] [path]` | `ls -la /home` |
| `cd` | Change directory | `cd <directory>` | `cd /usr/local` |
| `pwd` | Print working directory | `pwd` | `pwd` |
| `mkdir` | Create directories | `mkdir <directory>...` | `mkdir test` |
| `rm` | Remove files/directories | `rm [-r] <path>...` | `rm -r old_dir` |
| `cp` | Copy files | `cp <source> <dest>` | `cp file1.txt file2.txt` |
| `mv` | Move/rename files | `mv <source> <dest>` | `mv old.txt new.txt` |
| `cat` | Display file contents | `cat <file>...` | `cat log.txt` |
| `echo` | Display text | `echo [text...]` | `echo "Hello World"` |
| `clear` | Clear terminal screen | `clear` | `clear` |
| `exit` | Exit the shell | `exit` | `exit` |

### Command Details

#### ls
Lists directory contents with options:
- `-l`: Long format showing permissions, size and timestamps
- `-a`: Show hidden files (starting with .)
- `-F`: Classify entries with indicators:
  - `/` for directories
  - `*` for executables
  - `@` for symbolic links

Output is sorted alphabetically by filename.

#### rm
Remove files and directories:
- `-r`: Recursively remove directories and their contents
- Safeguards against removing directories without -r flag
- Proper error handling for non-existent files

#### cp
Copy files with features:
- Handles both files and directories as destinations
- Preserves original filename when copying to directory
- Reports detailed errors for source/destination issues

#### mv
Move/rename files:
- Supports moving files into directories
- Preserves original filename when moving to directory
- Provides detailed error messages

#### File Operations
Core file operations with robust error handling:
- Create directories with `mkdir` (multiple directories supported)
- Display file contents with `cat` (multiple files supported)
- Change directories with `cd` (with proper error messages)
- Print working directory with `pwd`

## Installation

### Prerequisites
- Rust toolchain (1.56.0 or later)
- Cargo package manager
- Unix-like environment (for file permission support)

### Building from Source
1. Clone the repository:
   ```bash
   git clone https://github.com/K11E3R/rust-shell.git
   cd rust-shell
   ```
2. Build the project:
   ```bash
   cargo build --release
   ```
3. Run the shell:
   ```bash
   cargo run --release
   ```

## Usage
1. Start the shell:
   ```bash
   cargo run --release
   ```

2. Use the shell to execute commands:
   ```bash
   $ ls -la /home
   ```

3. Type `exit` to quit the shell.

## Contributing

Contributions are welcome! Please feel free to submit pull requests.
