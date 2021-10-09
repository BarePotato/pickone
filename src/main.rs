use rand::{seq::SliceRandom, thread_rng};

fn main() {
    let boobs = std::process::Command::new("fd").arg("-d").arg("1").arg("-t").arg("d").output().unwrap();
    let mut to_shuf =
        boobs.stdout.split(|b| b == &b'\n').map(|b| std::str::from_utf8(b).unwrap()).collect::<Vec<&str>>();

    let mut rng = thread_rng();
    to_shuf.shuffle(&mut rng);

    // We could just grab a random index, but where is the fun in that?
    let shuf = std::process::Command::new("shuf").args(["-n", "1"]).arg("-e").args(to_shuf).output().unwrap();
    println!("{}", std::str::from_utf8(&shuf.stdout).unwrap().trim_end());
}
