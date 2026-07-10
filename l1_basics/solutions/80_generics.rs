// ジェネリクスは型を汎用的に使えるようにするための機能です。
// &T, &[T], [T; N], Vec<T>, Option<T>, Result<T, E> などがその例です。
//
// Rustコンパイラは、これらの枠に実際何の型が入るか最終的に決定します。
// コンピュータ上で具体的にどんなビット表現でデータを配置すればいいか、正確に把握するためです。
// とはいえ、プログラマは型を全て明示しなければならないわけではありません。
// なぜなら、型推論の機能があるからです。
// 例えば、変数宣言において
//
//     let s = "Hello, world!";
//
// と書いたときに、変数 s の型は暗黙的に &str となります。
// コンパイラは、これらの変数に「型の変数」を与えていると考えられます。
// 基本データ型や複合型を芋づる式に導いていって、型変数を一つずつ穴埋めするのが型推論の役目です。

// 構造型は、名前の後ろに <T> を付けることで型変数を定義できます。
struct Wrapper<T> {
    value: T,
}

// implキーワードの後ろで型変数を定義出来ます。
impl<T> Wrapper<T> {
    fn new(value: T) -> Self {
        Wrapper { value }
    }
}

fn main() {
    const MESSAGE: &str =
        "The answer to the ultimate question of life, universe, and everything is:";
    // 文字列 &str をラップすると Wrapper<&str> になります。
    println!("{}", Wrapper::new(MESSAGE).value);

    // ユニット型 () も立派な型の一員です。
    println!("{:?}", Wrapper::new(()).value);

    // T = Wrapper<u8> とすると、Wrapper<Wrapper<u8>> になります。
    println!("{}", Wrapper::new(Wrapper::new(42u8)).value.value);
}
