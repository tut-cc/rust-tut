// loop式と continue/break を組み合わせると実行の流れをより細かく制御できます。

fn main() {
    let ops = ['-', '+', '.']; // この行は変えないでください。

    // output = 9 と表示されるよう入力値を決めてください。
    let input = ???;
    println!("output = {}", interpret(&ops, input));
}

// 命令を解析して実行する関数です。命令は3種類あります。
//
// +  スコアを入力値の分だけ増やす
// -  スコアを1減らす
// .  入力値を1減らし、最初から命令を実行しなおす
//    入力値が0のときは命令解析を終了し、現在のスコアを返す
fn interpret(ops: &[char], mut input: i32) -> i32 {
    let mut score = 0; // スコア0から始める
    let mut ip = 0; // 命令ポインタ
    loop {
        // NOTE: 次の行を実行して命令、入力値、スコアの途中経過を確認できます。
        // eprintln!("'{}' (input: {input}, score: {score:2})", ops[ip]);

        match ops[ip] {
            '+' => score += input, // スコアを増やす
            '-' => score -= 1,     // スコアを減らす
            '.' => {
                if input > 0 {
                    ip = 0; // 命令ポインタを最初に移動させる
                    input -= 1; //  入力を1つ減らす
                    continue;
                } else {
                    break score; // loopブロックを脱出すると同時にスコアを返す
                }
            }
            _ => unreachable!(),
        }

        ip += 1; // 命令ポインタを次に移動させる
    }
}

// for文やwhile文はloop式を用いて書き直すことができます。
//
// 例えば、次のようなwhile文の場合、
//
//     let i = 0;
//     while i < 10 {
//         println!("{i}");
//         i += 1;
//     }
//
// loop式にすると次のようになります。
//
//     let i = 0;
//     loop {
//         if i < 10 {
//           println!("{i}");
//         } else {
//           break;
//         }
//     }
//
// 以下のfor文は、
//
//     for i in 0..10 {
//          println!("{i}");
//     }
//
// loop式で次のように展開できます。
//
//     let mut it = (0..10).into_iter();
//     loop {
//         if let Some(i) = it.next() {
//             println!("i");
//         } else {
//             break;
//         }
//     }
//
// 書き換えた後のelse分岐を見てみると、break文でループを脱出しています。
// どちらも式が省略されていることから、暗黙的にユニット () が返却値となります。
// forとwhileを文と表現していたのは、ユニット値しか返せない式、つまり文にしかなれないからなのです。
