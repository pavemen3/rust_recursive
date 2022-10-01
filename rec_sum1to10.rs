// 再帰的に呼び出す関数sumを定義
fn sum(n:i32) -> i32 {
  if n <= 0 { return 0; } // 再起の終了条件
  return sum (n-1) + n; //再帰的に呼び出す
}

fn main() {
  println!("{}", sum(10));
}

