fn main() {
    // == 01: if式 ==
    println!("== 01 ==");
    let number = 3;

    if number < 5 {
        println!("条件は真です。");
    } else {
        println!("条件は偽です。");
    }

    // == 02: 条件に付く型 ==
    
    // if (number) { ← エラー発生。なぜかというと、if式の中にbool型でなければならないからだ。
    //     println!("条件は真です。");
    // }

    // == 03: else if ==
    println!("== 03 ==");

    let number = 6;
    if number % 4 == 0 {
        println!("この数値は4で割り切れます。");
    } else if number % 3 == 0 {
        println!("この数値は3で割り切れます。");
    } else if number % 2 == 0 {
        println!("この数値は2で割り切れます。");
    } else {
        println!("この数値は4、3、2で割り切れません。");
    }

    // == 04: if中で値を返す ==
    println!("== 04 ==");

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("numberの値は{}である。", number);
    
    // let number = if condition { 5 } else { "6" }; ← エラー発生。返す値の型が異なる。
    // println!("numberの値は{}である。", number);

    // == 05: 永遠ループ ==
    println!("== 05 ==");
    let mut number = 1;
    
    loop {
        println!("{}回目の繰り返し。", number);
        number += 1;
        if number > 5 {
            break;
        }
    }

    // == 06: whileループ ==
    println!("== 06 ==");
    let mut number = 3;

    while number != 0 {
        println!("値は{}です。", number);
        number -= 1;
    }

    // == 07: コレクション要素のループ ==
    println!("== 07 ==");

    let arr = [10, 20, 30, 40, 50];
    for element in arr.iter() {
        println!("配列の要素の値は{}です。", element);
    }

    // == 練習1: 摂氏華氏変換 ==
    println!("== 練習1: 摂氏華氏変換 ==");

    let f: f64 = 72.0;
    println!("華氏{}度は摂氏{}度です。", f, f2c(f));

    let c: f64 = 24.5;
    println!("摂氏{}度は華氏{}度です。", c, c2f(c));

    // == 練習2: フィボナッチ数列 ==
    println!("== 練習2: フィボナッチ数列 ==");

    let mut n: i64 = 0;
    while n < 16 {
        println!("{}番目のフィボナッチ数列は{}です。", n, fiboc(n));
        n += 1;
    }

    // == 練習3: クリスマスの12日 ==
    println!("== 練習3: クリスマスの12日 ==");

    let sequence = ["first", "second", "third", "forth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelveth"];
    let presents = [
        "a patridge in a pear tree",
        "two turtle doves",
        "three french hens",
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eigth maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];
    let mut i: usize = 0;
    while i < 12 {
        print!("On the {} day of Christmas my true love sent to me", sequence[i]);
        let mut j: usize = i;
        loop {
            let connection: &str = if i > 0 && j == 0 { ", and" } else { "," };
            print!("{} {}", connection, presents[j]);
            if j < 1 {
                break;
            }
            j -= 1;
        }
        i += 1;
        println!("");
    }
}

fn c2f(c: f64) -> f64 {
    return (1.8 * c) + 32.0;
}

fn f2c(f: f64) -> f64 {
    return (f - 32.0) / 1.8;
}

fn fiboc(n: i64) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    return fiboc(n - 1) + fiboc(n - 2);
}
