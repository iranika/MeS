use std::path::{PathBuf, self};

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

    Parse{
        #[clap(parse(from_os_str))]
        path: std::path::PathBuf,   
    },
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
        },
        Commands::Parse { path } => {
            do_parse(path);
        }
    }
}

fn do_parse(path: PathBuf){
    let content = std::fs::read_to_string(path).expect("could not read file.");
    let json = mes::parseMeSToJson(&content);
    print!("{}", json);

}

fn do_chat(path: PathBuf){
    let content = std::fs::read_to_string(path)
        .expect("could not read file");
    println!("{:?}", content);
}