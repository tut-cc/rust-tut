// トレイトを用いると、複数の型に共通の振る舞いを定義できます。

trait Licensed {
    // デフォルトのライセンス情報を返す実装です。
    fn licensing_info(&self) -> String {
        "Permissive".to_string()
    }
    // デフォルト実装が無い場合は関数ブロックを省略して、
    //
    //     fn licensing_info(&self) -> String;
    //
    // と記述します。
}

// 特に何の変哲もないユニット型です。
struct SomeSoftware;
struct OtherSoftware;
struct Loicense;

// `impl トレイト for 型` で振る舞いを実装できます。
// これで各ユニット型に licensing_info() メソッドが定義されます。
impl Licensed for SomeSoftware {}
impl Licensed for OtherSoftware {}
impl Licensed for Loicense {
    // デフォルト実装は上書きすることが出来ます。
    fn licensing_info(&self) -> String {
        "OI! YOU GOT A LOICENSE FOR THAT, MATE?".to_string()
    }
}

// Licenseトレイトを実装した型だけ受け取り、ライセンス情報を出力する関数です。
//
// `impl トレイト` と書くことで、そのトレイトを実装した型だけを引数として受け取ることができます。
fn print_license_impl(material: impl Licensed) {
    println!("Impl: {}", material.licensing_info());
}

// 型変数 <T> を道入した別の書き方です。
// `T: トレイト` と書くと、そのトレイトを実装した型だけを受け付けることになります。
fn print_license_bound<T: Licensed>(material: T) {
    println!("Bound: {}", material.licensing_info());
}

fn main() {
    print_license_impl(SomeSoftware);
    print_license_impl(OtherSoftware);
    print_license_impl(Loicense);

    print_license_bound(SomeSoftware);
    print_license_bound(OtherSoftware);
    print_license_bound(Loicense);
}

// 単なる型変数 <T> だけだと、どんな型でも引き受ける有象無象の集まりになります。
// そこで、`T: トレイト` を使うと、指定した習性 (trait) を持つ型だけが選抜されます。
// この文法は、習性を持つ型と持たない型の間に境界線を引くため、トレイト境界と言われます。
