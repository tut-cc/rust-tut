// 次に、単一型の文字型と文字列型について見ていきます。

fn main() {
    // 文字型 char はUnicode文字を表します。
    // Unicodeは国際的な文字コードの標準規格です。
    const PROFESSOR: char = ???; // 文字はシングルクオートで定義します。
    println!("Lowercased {PROFESSOR}: {}", PROFESSOR.to_ascii_lowercase());

    let crab = '🦀';
    println!("The length of {crab} in UTF-8 is {}.", crab.len_utf8());
    println!("The length of {crab} in UTF-16 is {}.", crab.len_utf16());

    // 文字列スライス型 &str はUTF-8の文字列を表します。
    // UTF-8は国際的な文字コード方式で、Unicodeの代表的な実装です。
    const GREETING: &str = ???; // 文字列はダブルクオートで定義します。
    let (left, right) = GREETING.split_at(5); // 5文字目で左と右に分割
    println!("{left}, {right}!");

    let crab = ???; // 文字ではなく文字列として定義してください。
    println!("The length of {crab} is {}.", crab.len());
}

// 計算が完了しないことを意味するNever型 ! も単一型の一つです。
