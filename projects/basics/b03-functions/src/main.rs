fn main() {
    println!("本関数からの出力です。");

    // == 関数の宣言 ==
    println!("== 01 ==");
    another_function();
    
    // == 関数の引数 ==
    println!("== 02 ==");
    another_function2(0b0101);

    // == 複数の引数 ==
    println!("== 03 ==");
    another_function3(5, 6);
    
    // == 文と式 ==
    println!("== 04 ==");
    let x = 6; // これ全体は文であるが、イコールの右側には6の値を返す式である。
    println!("xの値は{}です。", x);
    // let y = (let x = 6); // エラー発生、`let y = 6`は文であって、値を返さないので、xに代入できない。
    let y = {
        let x = 3;
        x + 1
    }; // 波括弧の中には式である。最後にx+1の値を返す。最後にセミコロンを付ける必要がない。付けたら文になってしまって値を返さなくなる。
    println!("yの値は{}です。", y);

    // == 値を返す関数 ==
    println!("== 05 ==");
    let x = get_five();
    println!("xの値は{}です。", x);
    let y = plus_one(get_five());
    println!("yの値は{}です。", y);
}

fn another_function() { // 関数の宣言は呼び出す関数の前も後も宣言できる。
    println!("別の関数からの出力です。");
}

fn another_function2(x: i32) {  // 定義する関数の引数は「仮引数(parameter)」、実行時に値を持つ引数は「実引数(argument)」と呼ばれる。
                                // 各仮引数は型を宣言しなければならない。
    println!("xの値は{}です。", x);
}

fn another_function3(x: i32, y: i32) { // 複数の引数
    println!("xの値は{}です。", x);
    println!("yの値は{}です。", y);
}

fn get_five() -> i32 { // 矢印`->`の後に、この関数が返す型を定義する。
    5 // セミコロン不要。最後に式の5を返すと表す。returnを付けたら、同じだ多が、returnは関数の式の中にあったら早期に返す。
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
