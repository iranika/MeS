use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "subcommand", author, about, version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// コマンドライン引数をその場で指定しているサブコマンド。
    //Add { x: i64, y: i64 },
    //チャット形式
    Chat {
        #[clap(parse(from_os_str))]
        path: std::path::PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Chat { path } => {
            do_chat(path);
        }
    }
}

fn do_chat(path: PathBuf){
    let content = std::fs::read_to_string(path)
        .expect("could not read file");
    println!("{:?}", content);
}