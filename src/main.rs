use std::{path::{PathBuf, Path}, collections::HashMap, string};

use clap::{Parser, Subcommand};
use config::{Config, File, FileFormat};
use mes::mes::{MeSConfig, get_defaultConfig};
use serde::{Deserialize, Serialize, __private::de::FlatInternallyTaggedAccess};

#[derive(Debug, Parser)]
#[clap(name = "mes", author, about, version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,

    #[clap(short = 'c', long = "config", default_value_t = String::from("./mes.json"), value_parser)]
    conf: String,
}

#[derive(Debug, Subcommand)]
enum ConfigCommand {
    /// コンフィグを表示します
    Show,
    /// 初期設定のコンフィグを作成します
    Create
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// MeSをパースしてMedo型のJSON文字列として出力します.
    Parse{
        #[clap(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    /// WIP:チャット形式で出力します.
    Chat {
        #[clap(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    /// コンフィグ関連のサブコマンドです
    Config{
        #[clap(subcommand)]
        conf: ConfigCommand
    },
    /// キャラ毎にセリフの文字数を集計します
    Count {
        #[clap(parse(from_os_str))]
        path: std::path::PathBuf,
    }
}

fn main() {
    let cli = Cli::parse();
    //コンフィグの初期化
    let mesConf: MeSConfig;
    if std::path::Path::exists(std::path::Path::new(&cli.conf)){
        mesConf = Config::builder()
            .add_source(File::with_name(&cli.conf))
            .build()
            .unwrap()
            .try_deserialize::<MeSConfig>().unwrap();
    }else{
        mesConf = get_defaultConfig();
    }

    //サブコマンドの解析   
    match cli.command {
        Commands::Chat { path } => {
            do_chat(path);
        },
        Commands::Parse { path } => {
            do_parse(path);
        },
        Commands::Count { path } => {
            do_count(path,mesConf);
        },
        Commands::Config { conf } => {
            match conf {
                ConfigCommand::Create => {
                    do_config_create(cli.conf);
                },
                ConfigCommand::Show =>{
                    do_config_show(cli.conf, mesConf);
                    //print!("do_config::show");
                }
            }
        },
        _ => {
            println!("subcommand parse error!")
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
    println!("chatはまだ実装されていません.");
}
fn do_count(path: PathBuf, conf: MeSConfig){
    let content = std::fs::read_to_string(path).expect("could not read file");
    let json = mes::countDialogueWordToJsonWithConf(&content, conf);
    println!("{}", json);
}

fn do_config_create(path: String){
    //let filepath = PathBuf::from(&path);
    let defConf = get_defaultConfig();
    //let json = r#"{"name": "sample","counter": {"ignore_char": ["a","b","c"]}}"#;
    let json = serde_json::to_string(&defConf).unwrap();
    //TODO: すでにファイルがある場合は上書きするか確認をする
    std::fs::write(path, &json).expect("cannot write config");
}

fn do_config_show(path: String, mesconf: MeSConfig){
    
    println!("{}", serde_json::to_string(&mesconf).unwrap());

}

