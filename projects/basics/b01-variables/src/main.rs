fn main() {
    {   // == 不変変数 ==
        println!("== 01 ==");
        let x = 5; // 不変変数を作成する。
        println!("xの値は{}です。", x);
        // x = 6; // ここにコンパイルエラー発生、不変変数は代入できない。
        // println!("xの新しい値は{}です。", x);
    } { // == 可変変数 ==
        println!("== 02 ==");
        let mut x = 5; // 可変変数を作成する。
        println!("xの値は{}です。", x);
        x = 6;
        println!("xの新しい値は{}です。", x);
    } { // == 定数 ==
        println!("== 03 ==");
        const MAX_POINTS: u32 = 100000; // 定数と不変変数の基本の違い、定数は関数から返された値や、実行時に評価される値にセットできない。
        println!("定数MAX_POINTSの値は{}です。", MAX_POINTS);
    } { // == シャドーイング ==
        println!("== 04 ==");
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("xの値は(5+1)×2 = {}", x);

        // let spaces = "     ";
        // spaces = spaces.len(); // ここにコンパイルエラー発生、不変変数は代入できない。

        let spaces = "     ";
        let spaces = spaces.len();
        println!("空白数は{}です。", spaces);
    }
}
