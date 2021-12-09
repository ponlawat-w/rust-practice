fn main() {
    {
        // == 01 - 変数のスコープ ==
        println!("== 01 ==");
        {
            let s = "こんにちは。"; // ここからsが有効になる。
            println!("{}", s);
        }   // ここの波括弧まで、sが有効。
        {
            // println!("{}", s); // エラー発生、sがもう無効。
        }
    } {
        // == 02 - データの相互作用法：ムーブ ==
        println!("== 02 ==");
        let x = 5;
        let y = x;   // 値の代入
        println!("yの値は{}です。", y);
        println!("xの値は{}です。", x);     // 問題なし
        
        let s1 = String::from("こんにちは");
        let s2 = s1; // 参照の代入。この時点で、s1が無効。
        println!("s2の値は「{}」です。", s2);
        // println!("s1の値は「{}」です。", s1); // 問題発生、s1はs2に代入された時点で、コンパイラーによって無効化されてしまったから。
    } {
        // == 03 - データの相互作用法：クローン ==
        println!("== 03 ==");
        let s1 = String::from("こんにちは");
        let s2 = s1.clone();
        println!("s2の値は「{}」です。", s2);
        println!("s1の値も「{}」です。", s1);
    } {
        // == 04 - 関数と所有権 ==
        println!("== 04 ==");
        let s1 = String::from("こんにちは");
        print_string(s1);
        // print_string(s1); // ここエラー発生。関数の引数に渡す時もムーブ代入されるので、無効化されてしまった。
    }
}

fn print_string(s: String) {
    println!("{}", s);
}
