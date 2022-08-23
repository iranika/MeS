use std::{path::{PathBuf}, io::Read};

use clap::{Parser, Subcommand, builder};
use config::{Config, File};
//use mes::mes::{get_default_config};
use mes::mes::builder::{MeSBuilder};
use question::{Question, Answer};
use std::io;

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
    let mes_conf: MeSBuilder;
    if std::path::Path::exists(std::path::Path::new(&cli.conf)){
        mes_conf = Config::builder()
            .add_source(File::with_name(&cli.conf))
            .build()
            .unwrap()
            .try_deserialize::<MeSBuilder>().unwrap();
    }else{
        mes_conf = mes::mes::builder::new();
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
            do_count(path, mes_conf);
        },
        Commands::Config { conf } => {
            match conf {
                ConfigCommand::Create => {
                    do_config_create(cli.conf);
                },
                ConfigCommand::Show =>{
                    do_config_show(cli.conf, mes_conf);
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
    //let content = std::fs::read_to_string(path).expect("could not read file");
    println!("chatはまだ実装されていません.");
}
fn do_count(path: PathBuf, conf: MeSBuilder){
    let content = std::fs::read_to_string(path).expect("could not read file");
    let json = mes::mes::countDialogueWordToJsonWithConf(content, &conf);
    println!("{}", json);
}

fn do_config_create(path: String){
    //let filepath = PathBuf::from(&path);
    let def_conf = mes::mes::builder::new();
    //let json = r#"{"name": "sample","counter": {"ignore_char": ["a","b","c"]}}"#;
    let json = serde_json::to_string_pretty(&def_conf).unwrap();
    //すでにファイルがある場合は上書きするか確認をする
    let filepath = std::path::Path::new(&path);
    if std::path::Path::exists(filepath) {
        let answer = Question::new("すでにファイルが存在します。上書きしますか？")
            .confirm();
        
        if answer == Answer::NO {
            return //早期リターン
        }
    }
    std::fs::write(path, &json).expect("cannot write config");
 

}

fn do_config_show(path: String, mesconf: MeSBuilder){
    
    println!("{}", serde_json::to_string(&mesconf).unwrap());

}

