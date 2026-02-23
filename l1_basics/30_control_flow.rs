// if文を使うと条件に基づいて実行する文を切り替えることができます。
//
// 条件とは以下のようなものです。
//
//     a == b   a と b が等しい
//     a <  b   a は b より小さい
//     a <= b   a は b 以下である
//     a >  b   a は b より大きい
//     a >= b   a は b 以上である
//     a != b   a は b は等しくない
//
// Rustのif文は条件として真偽値しか受け付けません。

fn main() {
    deep_thinking(42); // the greatest and bestest of all time in the universe
    deep_thinking(57); // but a prime number according to Grothendieck
}

fn deep_thinking(x: u8) {
    // 条件式を直してください
    if x == ??? {
        println!("The answer to the ultimate question of life, universe, and everything.");
    } else {
        println!("Just a random number.");
    }
}
