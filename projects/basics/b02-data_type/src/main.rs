fn main() {
    {   // 静的型性
        // let guess = "42".parse().expect("数字ではありません。"); // エラー発生。コンパイル時にその変数はどの型か設定しなければならない。
        // 上記の場合はどの型にparse(変換)すればいいのかわからないので。
    } { // 静的型性（続）
        println!("== 01 ==");
        let guess: u32 = "42".parse().expect("数字でしゃありません。"); // 文字列を32ビット整数に変換する。
        println!("guessの値は{}です。", guess);
    } { // 整数
        // 符号無し: u8, u16, u32, u64, usize
        // 符号付き: i8, i16. i32, i64, isize
        // usizeとisizeは端末のアーキテクチャーによってビット数が決まること。
    } { // n進数で整数を表す方法
        println!("== 02 ==");
        let mut x: u32 = 98_222; // 10進の9万8,222;
        println!("xの値は{}です。", x);
        x = 98222;          // 同じく10進の9万8,222
        println!("xの値は{}です。", x);
        x = 0x1aff;         // 16進の1aff
        println!("xの値は{}です。", x);
        x = 0o72;           // 8進の72
        println!("xの値は{}です。", x);
        x = 0b1100_0100;    // 2進の11000100
        println!("xの値は{}です。", x);
    } { // 浮動小数点型
        println!("== 03 ==");
        let x: f64 = 2.5;
        println!("xの値は{}です。", x);
        let x: f32 = 2.5;
        println!("xの値は{}です。", x);
    } { // 数値演算
        println!("== 04 ==");

        let x = 5;
        let y = 10;

        println!("{} + {} = {}", x, y, x + y);
        println!("{} - {} = {}", x, y, x - y);
        println!("{} × {} = {}", x, y, x * y);
        println!("{} ÷ {} = {}", x, y, x / y);
        println!("{} ÷ {} の余り = {}", x, y, x % y);
    } { // 論理値
        println!("== 05 ==");
        
        let p: bool = true;
        
        println!("p = {}", p);
    } { // 文字
        println!("== 06 ==");

        let a: char = 'a';
        let b: char = 'あ';
        let c: char = 'ก';
        let d: char = '🤔';
        // let e: char = '🇹🇭'; エラー発生、国旗の絵文字は2つの文字から組み合わせたものなので。
        
        println!("{}{}{}{}", a, b, c, d);
    } { // タプル型
        println!("== 07 ==");
        
        let t: (i32, f64, u8) = (500, 3.2, 0xfe); // 複数の型が変数に
        println!("t.0 = {}", t.0);
        println!("t.1 = {}", t.1);
        println!("t.2 = {}", t.2);
        
        let (x, y, z) = t;
        println!("x = {}", x);
        println!("y = {}", y);
        println!("z = {}", z);
    } { // 配列
        println!("== 08 ==");

        let arr = [1, 2, 3, 4, 5];
        println!("arr[0] = {}", arr[0]);
        println!("arr[1] = {}", arr[1]);
        println!("arr[2] = {}", arr[2]);
        println!("arr[3] = {}", arr[3]);
        println!("arr[4] = {}", arr[4]);

        let days = ["月曜日", "火曜日", "水曜日", "木曜日", "金曜日", "土曜日", "日曜日"];
        println!("今日は{}です。", days[3]);
    }
}
