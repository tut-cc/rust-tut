// 次は基本データ型のうち、複合型について見ていきます。

fn main() {
    // タプル型は異なる型の集合です。
    let jack_of_all_trades = (true, -1, std::f32::consts::E, '🦀', "Helloworld");
    // タプル型の要素にアクセスするときは .数字 という記法を用います。
    let crab = jack_of_all_trades.3;
    // 文字列を分割する関数 split_at は、(&str, &str) というタプル型を返します。
    let (left, right) = jack_of_all_trades.4.split_at(5);
    println!("{left} {right} {crab}");

    // 配列型は同じ型 N 個の集合です。
    // [要素0, 要素1, ..., 要素N-1] と並べて定義できます。
    // ここでは [要素; N] という、同じ要素を N 個連続させる記法を紹介します。
    let mut array: [i32; 5] = [20; 5];
    println!("array = {:?}, len = {}", array, array.len()); // array.len() は配列の長さを返します。
    // 配列の要素は 配列[インデックス] でアクセスできます。
    array[0] = -30; // sum = 50 と出力されるようにしてください。
    let mut sum = 0;
    // 配列型はイテレータになれるので、for文で使えます。
    for item in array {
        sum += item;
    }
    println!("sum = {sum}");

    // スライス型はメモリ区間上に連続して配置された要素への参照です。
    // スライスを作る際には、for文の導入で出てきた範囲型 start..end が活躍します。
    let slice: &mut [i32] = &mut array[1..4]; // &mut [i32] は要素が変更可能であることを意味します。
    slice[1] = 100; // slice = [20, 100, 20] と出力されるようにしてください。
    println!("slice = {:?}, len = {}", slice, slice.len());
    println!("array = {:?}", array);
}

// array と slice がメモリ空間でどのように保存されているか可視化してみましょう。
//
//          #0  #1  #2  #3  #4
// array [ ??? -30 100  20  20]
//            |           |
//            |           |
// slice ---> [-30 100  20]
//              #0  #1  #2
//
// slice 自体は参照なので実態がなく、array の一部を指し示しているのみです。
// この図では slice[0] が array[1] に対応しています。
//
// 参照という概念については所有権とライフタイムが出てきたときにもっと詳しく説明します。
