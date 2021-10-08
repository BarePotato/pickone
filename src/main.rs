fn main() {
    let boobs = std::process::Command::new("fd").arg("-d").arg("1").arg("-t").arg("d").output().unwrap();
    let to_shuf = boobs.stdout.split(|b| b == &b'\n').map(|b| std::str::from_utf8(b).unwrap()).collect::<Vec<&str>>();
    let shuf = std::process::Command::new("shuf").args(["-n", "1"]).arg("-e").args(to_shuf).output().unwrap();
    println!("{}", std::str::from_utf8(&shuf.stdout).unwrap().trim_end());
}
