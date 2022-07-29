use std::{path::{PathBuf, self}, collections::HashMap, string};

use clap::{Parser, Subcommand};
use config::{Config, File, FileFormat};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
#[clap(name = "subcommand", author, about, version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short = 'c', long = "config", default_value_t = String::from("./mes.json"), value_parser)]
    conf: String,
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    Show,
    Create
}

#[derive(Debug, Deserialize)]
struct CounterConfig{
    ignore_char: Vec<String>
}
#[derive(Debug, Deserialize)]
struct CliConfig{
    name: String,
    counter: CounterConfig
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// MeSをパースしてMedo型のJSON文字列として出力します.
    //Add { x: i64, y: i64 },
    Parse{
        #[clap(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    /// チャット形式で出力します.
    Chat {
        #[clap(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    /// コンフィグ
    Config{
        #[clap(subcommand)]
        conf: ConfigCommand
    }
    //文字数カウンター
    /*
    Counter {
        #
    }
    */
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Chat { path } => {
            do_chat(path);
        },
        Commands::Parse { path } => {
            do_parse(path);
        },
        Commands::Config { conf } => {
            match conf {
                ConfigCommand::Create => {
                    do_config_create(cli.conf);
                },
                ConfigCommand::Show =>{
                    do_config_show(cli.conf);
                    //print!("do_config::show");
                }
            }
        },
        _ => {
            println!("")
        }
    }
}

fn do_config_create(path: String){
    let filepath = PathBuf::from(&path);
    let json = r#"{"name": "sample","counter": {"ignore_char": ["a","b","c"]}}"#;
    std::fs::write(path, "this is config").expect("cannot write config");
}

fn do_config_show(path: String){
    let setting = Config::builder()
        .add_source(File::with_name(&path))
        .build()
        .unwrap();

    println!("{:?}",
        setting.try_deserialize::<CliConfig>().unwrap()
    );
    
    /*
    let config = std::fs::read_to_string(path).unwrap();
    let json: CliConfig = serde_json::from_str(&config).unwrap();
    print!("{:?}", json);
    */


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