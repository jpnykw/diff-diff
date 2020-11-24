use std::env;
use std::path::Path;
use regex::Regex;

mod process;
mod file;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { panic!("You need the path"); }

    let path: &Path = Path::new(&args[1]);
    if let Ok(content) = file::read(path) {
        let mut content = content.split("#SECTION#");
        // マッチに使用する正規表現パターン
        let pattern: &str = &content.next().unwrap().replace("\n", "");
        dbg!(&pattern);
        let pattern = Regex::new(pattern).unwrap();
        // 比較する2つのコンテンツ
        let a: Vec<&str> = content.next().unwrap().split("\n").collect();
        let b: Vec<&str> = content.next().unwrap().split("\n").collect();
        dbg!(&a);
        dbg!(&b);
        // コンテンツからパターンを抽出
        let a = process::extract(&a, &pattern);
        let b = process::extract(&b, &pattern);
        // 差分を表示
        let result = process::diff(&a, &b);
        dbg!(result);
    } else {
        println!("Please enter collect file path.");
    }
}

