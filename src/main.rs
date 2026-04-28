use crate::cli::Cli;

mod cli;
mod command;
fn main() {
    let cli = Cli::parse();
    if let Some(cli) = cli {
        match cli.command {
            command::Commands::Version => {
                let version = env!("CARGO_PKG_VERSION");
                println!("zen-browser-url-to-desktop : {}", version);
            }
            command::Commands::Help => {
                println!(
                    "Usage: zen-browser-url-to-desktop [OPTIONS] COMMAND [ARGS]

Options:

    help        Print this message.
    version     Print the version.

Commands:

    create      Create a new desktop.
    remove      Remove an existing desktop.
                    "
                )
            }
            command::Commands::Create(name, url) => {
                let dir_path = format!("/home/{}/.local/share/applications/", env!("USER"));
                let file_path = format!("{}/{}.desktop", dir_path, name);
                std::fs::create_dir_all(&dir_path).unwrap();
                let content = format!(
                    "[Desktop Entry]
Name={}
Exec=zen-browser --blank-window {}
Icon={}
Type=Application
",
                    name, url, name
                );
                let bytes = command::get_logo_png(url).unwrap();
                let icon_file_path = format!(
                    "/home/{}/.local/share/icons/hicolor/64x64/apps/{}.png",
                    env!("USER"),
                    name
                );
                std::fs::write(&file_path, content).unwrap();
                println!("[Info] 已经创建 {}", file_path);
                std::fs::write(&icon_file_path, bytes).unwrap();
                println!("[Info] 已经创建 {}", icon_file_path);
            }
            command::Commands::Remove(name) => {
                let app_dir_path = format!("/home/{}/.local/share/applications/", env!("USER"));
                let file_path = format!("{}/{}.desktop", app_dir_path, name);

                let icon_dir_path = format!(
                    "/home/{}/.local/share/icons/hicolor/64x64/apps/",
                    env!("USER")
                );
                let icon_file_path = format!("{}/{}.png", icon_dir_path, name);
                std::fs::remove_file(&file_path).unwrap();
                println!("[Info] 已经删除 {}", file_path);
                std::fs::remove_file(&icon_file_path).unwrap();
                println!("[Info] 已经删除 {}", icon_file_path);
            }
        }
    }
}
