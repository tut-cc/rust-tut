// Option<T> は値の有無を候補値とした列挙型です。
//
//     enum Option<T> {
//         Some(T),
//         None,
//     }
//
// None は値の生成失敗や値の欠如などを示すために用いられます。
// Some(T) は値の存在を意味します。
//
// ここで T は <T> で定義された型変数で、任意の型を意味します。
// 例えば、T が String の場合は、 Some(T) が Some(String) になります。
// 型ジェネリクスと呼ばれるこの機能については、後々詳しく解説します。

// 割られる数と割る数を受け取って、割り算の結果を返します。
// ただし、割る数が0のときはNone、それ以外のときはSomeを返します。
fn checked_division(dividend: f64, divisor: f64) -> Option<f64> {
    if divisor == 0. {
        // Option::None と書かなくても、候補値を直接使えます。
        None
    } else {
        // Some も同様に直接使えます。
        Some(dividend / divisor)
    }
}

fn main() {
    // 割る数が0の場合は Some() 、0の場合は None になります。
    println!("3.0 / 2.0 = {:?}", checked_division(3., 2.));
    println!("3.0 / 0.0 = {:?}", checked_division(3., 0.));

    // Optionは列挙型なので、if let 文でパターンマッチと変数宣言ができます。
    if let Some(one_third) = checked_division(1., 0.3) {
        println!("One third = {one_third}");
    }

    // Option::unwrap() メソッドを使うと、Some() の中身を取り出すことができます。
    println!("3.0 / 2.0 = {}", checked_division(3., 2.).unwrap());
    // ただしNoneだった場合はパニックが起こり、以降実行が中断されてしまいます。

    // x = 3.0 と表示されるようにしてください。
    let x = checked_division(1.5, 0.5).unwrap();
    println!("x = {x}");
}

// Option::unwrap() はふつう、Option値がSomeであることが分かりきっているときに使います。
// そうでない場合は、パターンマッチや次のようなメソッドを使います。
//
//    Option::unwrap_or(T)  デフォルト値を指定する
//    Option::expect(&str)  パニックのメッセージを指定する
//
// パニック自体はプログラムの実行がその場で終わるだけで、メモリ安全な操作です。
//
// 逆にメモリ安全**でない**のは、Noneであるのにも関わらず値を直接読み出すことです。
// それは、Option::unwrap_unchecked() メソッドでできます。
//
//   unsafe { value.unwrap_unchecked() }
//
// unwrap_unchecked() はunsafeなメソッドなので、unsafeブロック内でしか使えません。
// この関数が呼ばれた時点で value が None に成り得ないことをプログラマが保証しなければならないのです。
