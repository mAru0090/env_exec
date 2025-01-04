use std::env;
use std::fs::File;
use std::process::{Command, Stdio};
use std::io::{self, BufRead};
use toml;
use std::path::Path;
use serde::Deserialize;
use serde::de::Error;
use anyhow::Result;
#[derive(Debug, Deserialize)]
struct Config {
    version: f32,
    paths: Vec<String>,
    envs: Vec<Vec<String>>,
}

fn main() -> Result<()> {
    // コマンドライン引数の確認
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <config_file> <shell> [command...]", args[0]);
        std::process::exit(1);
    }

    // 設定ファイルとシェルの取得
    let config_file = &args[1];
    let shell = &args[2];
    let command_args = &args[3..];

    // 設定ファイルを読み込む
    let config: Config = read_toml(config_file)?;

    // 現在の Path 環境変数を取得
    let current_path = env::var("Path").unwrap_or_default();
    let mut new_path = current_path.clone();

    // TOMLの paths を Path 環境変数に追加
    for path in config.paths {
        if !path.trim().is_empty() {
            new_path.push(';');
            new_path.push_str(&path);
        }
    }

    // 新しい Path を設定
    env::set_var("Path", new_path);

    // 環境変数の設定
    for env in config.envs {
        if env.len() == 2 {
            let (key, value) = (&env[0], &env[1]);
            if !key.is_empty() && !value.is_empty() {
                env::set_var(key, value);
            }
        }
    }

    // コマンドを構築して実行
    let mut command = Command::new(shell);
    command.args(command_args);
    // 標準入出力の設定（オプション）
    command.stdin(Stdio::inherit())
           .stdout(Stdio::inherit())
           .stderr(Stdio::inherit());

    // コマンド実行
    let status = command.status()?;
    if !status.success() {
        eprintln!("Command failed with status: {:?}", status);
    }

    Ok(())
}

// TOMLファイルを読み込む関数
fn read_toml<P>(filename: P) -> Result<Config, toml::de::Error>
where
    P: AsRef<Path>,
{
    let mut file = File::open(filename).map_err(|e| toml::de::Error::custom(e.to_string()))?;
    let mut contents = String::new();
    io::Read::read_to_string(&mut file, &mut contents).unwrap();
    toml::de::from_str(&contents)
}
