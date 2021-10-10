use rand::{thread_rng, Rng};

fn main() {
    let boobs = std::process::Command::new("fd").arg("-d").arg("1").arg("-t").arg("d").output().unwrap();

    let mut dirs = boobs.stdout.split(|b| b == &b'\n').map(|b| std::str::from_utf8(b).unwrap()).collect::<Vec<&str>>();

    if let Some(last) = dirs.last() {
        if last.is_empty() {
            dirs.pop();
        }
    }

    if dirs.len().lt(&1) {
        return println!("There needs to be at least one directory in target location.");
    }

    let mut rng = thread_rng();
    let i = rng.gen_range(0..dirs.len());

    println!("{}", dirs[i]);
}
