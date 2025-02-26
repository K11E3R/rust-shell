use std::env;
use std::path::Path;
use std::fs::{create_dir};
use std::io::{self, Write, Read, copy};
use std::fs::{self, File};
use std::time::UNIX_EPOCH;
use std::os::unix::fs::PermissionsExt;


fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap(); // Assure que le prompt $ est affiché avant de bloquer pour l'entrée

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break, // EOF ou Ctrl+D
            Ok(_) => {
                // Traitement de la commande
                execute_command(input.trim());
            },
            Err(error) => eprintln!("Error: {}", error),
        }
    }
}

fn mv(source: &Path, destination: &Path) -> io::Result<()> {
    // Construit le nouveau chemin de destination pour inclure le répertoire source
    let final_destination = if destination.is_dir() {
        destination.join(source.file_name().ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid source name"))?)
    } else {
        destination.to_path_buf()
    };

    // Déplace le répertoire source vers le nouvel emplacement
    fs::rename(source, &final_destination)?;

    println!("Moved {:?} to {:?}", source, final_destination);

    Ok(())
}




fn execute_command(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    // Handle empty input
    if parts.is_empty() {
        return;
    }

    let command = parts[0];  // Safe now since we checked parts is not empty
    let args = &parts[1..]; // This is now safe too

    match command {
        "mv" => {
            if args.len() != 2 {
                eprintln!("Usage: mv <source> <destination>");
            } else {
                let source = Path::new(args[0]);
                let destination = Path::new(args[1]);
                if let Err(e) = mv(source, destination) {
                    eprintln!("mv error: {}", e);
                }
            }
        },
        "cd" => cd(args),
        "echo" => echo(args),
        "pwd" => pwd(),
        "ls" => {
            let combined_args = combine_flags(args);
            ls(&combined_args)
        },
        "mkdir" => mkdir(args),
        "cat" => cat(args),
        "rm" => {
            if let Err(e) = rm(args) {
                eprintln!("rm error: {}", e);
            }
        },
        "cp" => cp(args),
        "clear" => clear(),
        "exit" => std::process::exit(0),
        _ => eprintln!("Command '{}' not found", command),
    }
}

fn mkdir(args: &[&str]) {
    if args.is_empty() {
        eprintln!("mkdir: missing operand");
        return;
    }

    for path in args {
        if let Err(e) = create_dir(path) {
            eprintln!("mkdir: cannot create directory '{}': {}", path, e);
        }
    }
}


fn cat(args: &[&str]) {
    if args.is_empty() {
        eprintln!("cat: missing operand");
        return;
    }

    for path in args {
        let mut file = match File::open(path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("cat: {}: {}", path, e);
                continue;
            },
        };

        let mut contents = String::new();
        if let Err(e) = file.read_to_string(&mut contents) {
            eprintln!("cat: error reading {}: {}", path, e);
            continue;
        }

        print!("{}", contents);
    }
}

fn rm(args: &[&str]) -> io::Result<()> {
    if args.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "rm: missing operand"));
    }

    let mut recursive = false;
    let mut files = Vec::new();

    for arg in args {
        if *arg == "-r" {
            recursive = true;
        } else {
            files.push(*arg);
        }
    }

    for path in files {
        let path = Path::new(path);
        if path.is_dir() {
            if recursive {
                fs::remove_dir_all(path)?;
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, 
                    "rm: cannot remove directory without -r flag"));
            }
        } else {
            fs::remove_file(path)?;
        }
    }

    Ok(())
}

fn cp(args: &[&str]) {
    if args.len() != 2 {
        eprintln!("cp: missing operand");
        return;
    }

    let source_path = Path::new(&args[0]);
    let mut destination_path = Path::new(&args[1]).to_path_buf();

    if let Ok(metadata) = fs::metadata(&destination_path) {
        if metadata.is_dir() {
            if let Some(filename) = source_path.file_name() {
                destination_path.push(filename);
            } else {
                eprintln!("cp: invalid source path");
                return;
            }
        }
    }

    match (File::open(&source_path), File::create(&destination_path)) {
        (Ok(mut src), Ok(mut dst)) => {
            if let Err(e) = copy(&mut src, &mut dst) {
                eprintln!("cp: error copying from {:?} to {:?}: {}", source_path, destination_path, e);
            }
        },
        (Err(e), _) => eprintln!("cp: error opening source file '{:?}': {}", source_path, e),
        (_, Err(e)) => eprintln!("cp: error creating destination file '{:?}': {}", destination_path, e),
    }
}



fn cd(args: &[&str]) {
    if args.len() > 0 {
        if let Err(e) = env::set_current_dir(&Path::new(args[0])) {
            eprintln!("cd: {}", e);
        }
    } else {
        eprintln!("cd: missing argument");
    }
}

fn echo(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn pwd() {
    if let Ok(path) = env::current_dir() {
        println!("{}", path.display());
    } else {
        eprintln!("pwd: failed to get current directory");
    }
}

fn clear() {
    // ANSI escape sequence to clear screen and move cursor to top-left
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

fn combine_flags(args: &[&str]) -> Vec<String> {
    let mut combined_args = Vec::new();
    
    for arg in args {
        if arg.starts_with('-') && arg.len() > 2 {
            // Split combined flags (e.g., -la into -l -a)
            let flags = arg[1..].chars().map(|c| format!("-{}", c));
            for flag in flags {
                combined_args.push(flag);
            }
        } else {
            combined_args.push((*arg).to_string());
        }
    }
    
    combined_args
}

fn ls(args: &[String]) {
    let mut path = ".";
    let mut show_all = false;
    let mut long_format = false;
    let mut classify = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "-l" => long_format = true,
            "-a" => show_all = true,
            "-F" => classify = true,
            arg if arg.starts_with('-') => {
                eprintln!("ls: invalid option -- '{}'", arg);
                return;
            }
            _ => path = args[i].as_str(),
        }
        i += 1;
    }

    match fs::read_dir(path) {
        Ok(entries) => {
            let mut entries: Vec<_> = entries.filter_map(Result::ok).collect();
            entries.sort_by_key(|e| e.file_name());

            for entry in entries {
                let file_name = entry.file_name().to_string_lossy().into_owned();
                if !show_all && file_name.starts_with('.') {
                    continue;
                }

                if let Ok(metadata) = entry.metadata() {
                    if long_format {
                        let permissions = format_permissions(&metadata);
                        let size = metadata.len();
                        let modified = metadata.modified()
                            .ok()
                            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
                            .map(|duration| duration.as_secs())
                            .unwrap_or(0);
                        println!("{} {:8} {:8} {}", permissions, size, modified, file_name);
                    } else {
                        let type_suffix = if classify {
                            if metadata.is_dir() { "/" }
                            else if metadata.is_symlink() { "@" }
                            else if metadata.permissions().mode() & 0o111 != 0 { "*" }
                            else { "" }
                        } else { "" };
                        print!("{}{}\t", file_name, type_suffix);
                    }
                }
            }
            if !long_format {
                println!();
            }
        },
        Err(e) => eprintln!("ls: cannot access '{}': {}", path, e),
    }
}

fn format_permissions(metadata: &fs::Metadata) -> String {
    let mode = metadata.permissions().mode();
    let mut result = String::with_capacity(10);
    
    result.push(if metadata.is_dir() { 'd' } else { '-' });
    result.push(if mode & 0o400 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o200 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o100 != 0 { 'x' } else { '-' });
    result.push(if mode & 0o040 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o020 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o010 != 0 { 'x' } else { '-' });
    result.push(if mode & 0o004 != 0 { 'r' } else { '-' });
    result.push(if mode & 0o002 != 0 { 'w' } else { '-' });
    result.push(if mode & 0o001 != 0 { 'x' } else { '-' });
    
    result
}
