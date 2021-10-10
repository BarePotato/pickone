use std::fs;

use rand::{thread_rng, Rng};

fn main() {
    let dirs = match get_dirs() {
        Ok(d) => d,
        Err(e) => return print_error(&e.to_string()),
    };

    if dirs.len().lt(&1) {
        return print_error("There needs to be at least one directory in target location.");
    }

    let mut rng = thread_rng();
    let i = rng.gen_range(0..dirs.len());

    println!("\x1b[1;35m{}\x1b[0m", dirs[i]);
}

fn get_dirs() -> Result<Vec<String>, std::io::Error> {
    Ok(fs::read_dir(".")?
        .filter_map(|p| p.ok())
        .filter(|p| p.path().is_dir())
        .map(|p| p.file_name().into_string())
        .filter_map(|p| p.ok())
        .collect::<Vec<String>>())
}

fn print_error(e: &str) {
    println!("\x1b[1;31m{}\x1b[0m", e);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dir() {
        // This is going to fail anywhere other than my original cargo crate
        assert_eq!(get_dirs().unwrap(), vec!["src", ".git", "target"]);
    }
}
