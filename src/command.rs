use std::{env::Args, iter::Skip, process::Command};

pub enum Commands {
    Version,
    Help,
    Create(String, String),
    Remove(String),
}
impl Commands {
    pub fn from(value: &str, args: &mut Skip<Args>) -> Option<Self> {
        Some(match value {
            "version" => Commands::Version,
            "help" => Commands::Help,
            "create" => Commands::Create(args.next().unwrap(), args.next().unwrap()),
            "remove" => Commands::Remove(args.next().unwrap()),
            _ => return None,
        })
    }
}

pub fn get_logo_png(url: String) -> Option<Vec<u8>> {
    let ico_url = url.to_owned() + "/favicon.ico";
    let status = Command::new("curl")
        .arg("-ILs")
        .arg(&ico_url)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap();
    if status.contains("HTTP/2 404") {
        let bytes = Command::new("curl")
            .arg("-L")
            .arg("-s")
            .arg(format!("https://favicon.run/favicon?domain={}&sz=64", url))
            .output()
            .map(|o| o.stdout)
            .unwrap();
        println!("123");
        return Some(bytes);
    } else {
        let bytes = Command::new("curl")
            .arg("-L")
            .arg("-s")
            .arg(&ico_url)
            .output()
            .map(|o| o.stdout)
            .unwrap();
        println!("456");
        return Some(bytes);
    }
}
