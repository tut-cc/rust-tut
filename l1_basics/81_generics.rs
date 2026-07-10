// 型推論によって全ての型変数が一意に定まるとは限りません。
// プログラマはコンパイラが推論できない部分の穴埋めをする必要があります。

fn main() {
    // 型の推論がどのように起こっているのかを理解するために、次の2つのメソッドを導入します。
    //
    //     Into::into(T) -> U
    //     TryInto::try_into(T) -> Result<U, Error>
    //
    // Into::into は型Tから型Uの変換が成功することが保証された関数で、
    // TryInto::try_into は型Tから型Uへの変換が失敗するかもしれない関数です。
    // このエクササイズでは、整数型どうしの変換から関係性を見ていきます。

    // ベクタの要素は全て同じ型でなければなりません。
    // ところが、Vec::push で u8 の値と i8 の値を詰め込んでいます。
    // u8とi8のどちらの範囲もカバーできて、それでいて最小のビット数で表現できる整数の型は何でしょうか？
    let mut numbers: ???;
    numbers = Vec::new(); // ベクタ Vec<T> の型変数 T の宣言
    let a = 42u8;
    numbers.push(a.into()); // u8 から変換可能な整数型は不明
    let b = -1i8;
    numbers.push(b.into()); // u8, i8 から変換可能な整数型はまだ不明 -> 型変数の明示的な定義が必要
    println!("{numbers:?}");
    println!("{}", std::any::type_name_of_val(&numbers));

    let mut numbers = Vec::new(); // ベクタ Vec<T> の型変数 T の宣言
    let a = 42u8;
    // a.try_into() の返却値の型はまだ未知数です。
    if let ???(a) = a.try_into() {
        numbers.push(a);
    }
    let b = -1i8;
    numbers.push(b); // 型変数 T を i8 に固定
    println!("{numbers:?}");
    println!("{}", std::any::type_name_of_val(&numbers));
}

// IntoとTryIntoは列挙型や構造体ではなくトレイトと呼ばれるものです。
// トレイトについては次に解説します。
