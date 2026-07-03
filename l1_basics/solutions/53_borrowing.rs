// 参照には定義された場所と最後に使われた場所までのライフタイム (生存期間) があります。
// 前回出てきた v_owned_mut_slice の簡易版を見てみましょう。
//
//     let mut x = 42;
//     let x_mut_ref = &mut x;
//     *x_mut_ref = 57;
//     dbg!(&x);
//     dbg!(x_mut_ref);
//
// このプログラムには占有参照 x_mut_ref と一時的な共有参照 &x があります。
// 2つの参照のライフタイム 'a, 'b を導入して擬似的に可視化してみましょう。
// 本来は型宣言のときにしか使えない記法ですが、ここでは説明のために普通の文中でも使います。
//
//     let mut x = 42;
//     'a: {
//         let x_mut_ref = &'a mut x;
//         *x_mut_ref = 57;
//         'b: {
//            let x_tmp_ref = &'b x;
//             dbg!(x_tmp_ref);
//         }
//         dbg!(x_mut_ref);
//     }
//
// 'a スコープの中に 'b スコープが入れ子になっていることが分かります。
// どちらも変数 x への参照区間なのですが、問題なのは 'a が占有区間であるということです。
// dbg!(x_mut_ref); の行を消した場合も見てみましょう。
//
//     let mut x = 42;
//     'a: {
//         let x_mut_ref = &'a mut x;
//         *x_mut_ref = 57;
//     }
//     'b: {
//        let x_tmp_ref = &'b x;
//         dbg!(x_tmp_ref);
//     }
//
// 'b が 'a の外に移動したのは、x_mut_refが最後に使われた場所が 'b スコープの前だからです。
// この場合は占有区間と共有区間は重なりあっておらず、借用ルールが守られたことになります。
// このように、Rustでは参照区間がどこから始まりどこで終わるのか意識する必要があります。

// 長い方の文字列を返す関数です。
//
// ライフタイムは省略できないこともあります。
// 以下の関数では、引数 x, y と返り値が全て同じライフタイムであることを明示しています。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 以下はコンパイルが通らない関数定義です。
// コメントアウトしてエラーメッセージを確認してみましょう。
//fn longest_bad0(x: &str, y: &str) -> &str {
//    if x.len() > y.len() { x } else { y }
//}
//fn longest_bad1<'a>(x: &'a str, y: &str) -> &'a str {
//    if x.len() > y.len() { x } else { y }
//}
//fn longest_bad2<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//    if x.len() > y.len() { x } else { y }
//}
//fn longest_bad3<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
//    if x.len() > y.len() { x } else { y }
//}

#[allow(dead_code)]
// より冗長になりますが、この定義でもコンパイルが通ります。
fn _longest<'a: 'b, 'b: 'a>(x: &'a str, y: &'b str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// 'a: 'b は 'a のライフタイムが 'b と同じかより長生きすることを表す制約です。
// 'b: 'a も指定しているので、実質 'a と 'b のライフタイムが同じことを意味します。

fn main() {
    // ある行を移動させてコンパイルが通るようにしてください。
    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'.");
}

// 特殊なライフタイムとして 'static と '_ があります。
//
// 'static は、定義されてからプログラム全体が終了するまで生存するライフタイムです。
// グローバル変数やメモリのリークなどが代表的な例です。
//
// '_ は、プレースホルダー・ライフタイムと呼ばれます。
// 名前付きライフタイムの宣言が省略できるときに使われることがあります。
// 使いどころが分かりづらいかもしれませんが、そんな機能があるということだけ知ってもらえれば大丈夫です。
