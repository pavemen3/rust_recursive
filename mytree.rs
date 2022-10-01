use std::{env, path};
fn main() {
  // コマンドライン引数を得る
  let args: Vec<String> = env::args().collect();
  // カレントディレクトリーを指定
  let mut target_dir = ".";
  if args.len() >= 2 { // パスを指定した場合
    target_dir = &args[1];
  }
  // PathBufに変換
  let target = path::PathBuf::from(target_dir);
  println!("{}", target_dir);
  tree(&target, 0);
}

// 再帰的にファイル一覧を表示
fn tree(target: &path::PathBuf, level: isize) {
  // ファイル一覧を取得
  let files = target.read_dir().expect("存在しないパス");
  // 繰り返し表示
  for ent in files {
    // PathBufを取得
    let path = ent.unwrap().path();
    // level分だけインデント
    for _ in 1..=level {
      print!("|  ");
    }
    // ファイル名を得る
    let fname = path.file_name().unwrap().to_string_lossy();
    // ディレクトリなら再帰的に表示
    if path.is_dir() {
      println!("|-- <{}>", fname);
      tree(&path, level+1);
      continue;
    }
    // ファイル名を表示
    println!("|-- {}", fname);
  }
}

      
