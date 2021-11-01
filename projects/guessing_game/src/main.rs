use std::io; // 標準ライブラリーから入出力ioライブラリーをスコープに導入する。
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("数を当ててごらん！");

    // == 乱数を生成する ==
    let secret_number = rand::thread_rng().gen_range(1, 101);
        // rand::thread_rng() => 乱数生成器を返す。
        // gen_range() => 含んでいる下限値と含んでいない上限値の範囲で乱数を生成する。
    
    loop { // 無限ループ
        // == 値を変数に保持する ==
        println!("予想を入力してください。");
        let mut guess = String::new();
            // let => 変数を生成する。
            // mut => 変数を可変にする。
            // ということで guess は可変変数である。
            // String型、標準ライブラリーにある。サイズ可変、UTF-8エンコード。
            // ::は、関連関数（静的メソッド）を表す。
            // new()は、新規値を生成する関数である。
            // まとめて、新たにからのString型オブジェに束縛されている可変変数guessを作る。
    
        io::stdin().read_line(&mut guess).expect("行の読み込みに失敗しました。");
            // 冒頭で`use std::io`がなければ、この関数は`std::io::stdin`で呼ばなければならない。
            // read_lineメソッド、(参照渡しの)可変変数の引数はメソッドの中でユーザが入力した内容に変えられる。
            // read_lineメソッドの返しは列挙Result型である。Resultの列挙子はOKかErr。
            // Result.expectメソッド、Ok値であればスルー、Err値であれば引数の文字列を表示してプログラムを中止させる。
    
        // == 予想した数字を比較する ==
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // 変換成功の場合、Okアームに格納された値(引数)を渡す。
            Err(_) => continue  // 変換失敗の場合、ループの次にスキップする。
        };
        // u32 => 符号なし32ビット整数型(非負)。
        // trim => 両端の空白を除去する。
        
        println!("{}を予想しました。", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小さすぎます！"),
            Ordering::Greater => println!("大きすぎます！"),
            Ordering::Equal => {
                println!("やりましたね！");
                break;
            }
        }
        // match式は、冒頭で与えた値が照合するアームに実行する。
    }
}
