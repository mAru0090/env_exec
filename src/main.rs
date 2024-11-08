/*
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    // コマンドライン引数の確認
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <env_file> <shell> [command...]", args[0]);
        std::process::exit(1);
    }

    // 環境変数ファイルのパスとシェルを取得
    let env_file = &args[1];
    let shell = &args[2];
    let command_args = &args[3..];

    // 現在の Path 環境変数を取得
    let current_path = env::var("Path").unwrap_or_default();
    let mut new_path = current_path.clone();

    // 環境変数ファイルを読み込み、Path を更新
    if let Ok(lines) = read_lines(env_file) {
        for line in lines {
            if let Ok(path_var) = line {
                new_path.push(';');  // セパレータとして `;` を追加
                new_path.push_str(&path_var);
            }
        }
    }

    // 新しい Path を設定
    env::set_var("Path", new_path);

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

// ファイルを行ごとに読み込む関数
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
*/
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    // コマンドライン引数で環境変数ファイルのパスを取得
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <env_file>", args[0]);
        std::process::exit(1);
    }
    let env_file = &args[1];

    // 現在の Path 環境変数を取得
    let current_path = env::var("Path").unwrap_or_default();
    let mut new_path = current_path.clone();

    // 環境変数ファイルを読み込み、Path を更新
    if let Ok(lines) = read_lines(env_file) {
        for line in lines {
            if let Ok(path_var) = line {
                if !path_var.is_empty() {
                    new_path.push(';');  // セパレータとして `;` を追加
                    new_path.push_str(&path_var);
                }
            }
        }
    }

    // エクスポート用のバッチスクリプトを作成
    let mut output_file = File::create("set_env_vars.bat")?;
    writeln!(output_file, "@echo off")?;
    writeln!(output_file, "set Path=\"{}\"", new_path)?;

    println!("Generated set_env_vars.bat script. Run it in your cmd to apply the new Path.");

    Ok(())
}

// ファイルを行ごとに読み込む関数
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



