// 文字列を導入したときに &str は文字列「スライス」型であると紹介しました。
// つまり、データ自体はメモリ空間上に連続して配置されていることを意味します。
//
// しかし、普通のスライス型同様に、宣言した後は要素の数を変えられません。
// ベクタ型 Vec<T> のように、実行時に要素を追加したり削除したりできるのが String です。

fn main() {
    let mut animal = "".to_string(); // 文字列の初期化 (String::from("") でも出来ます)
    animal.push('d'); // 文字を追加
    animal.push_str("eer"); // 文字列スライスをまとめて追加
    print!("I thought I saw a {animal} over there but looking closer ");
    animal.pop(); // 後ろの文字を削除
    let animal = animal.replace("ee", "og"); // 文字列を置き換えて実際居た動物にしてください。
    println!("it was just a lazy {animal}. And then suddenly...");

    // format!マクロを使うと、println!と文字列内に値を埋め込むことが出来ます。
    let mut pangram = format!("The quick brown ox jumps over the lazy {animal}!");
    let ox_idx = pangram.find("ox").unwrap();
    pangram.insert(ox_idx, 'f'); // パングラムになるよう文字を挿入してください。
    if check_pangram(&pangram) {
        println!("{pangram}");
    }
}

// 入力文字列がすべてのアルファベットを使った文であるか確かめる関数です。
//
// 中身の実装は気にしないで大丈夫です。
fn check_pangram(input: &str) -> bool {
    let mut mask = 0u32;
    for byte in input.bytes() {
        if byte.is_ascii_alphabetic() {
            mask |= 1 << (byte.to_ascii_lowercase() - b'a');
        }
    }
    mask == 0x3FF_FFFF
}

// Stringは内部的には Vec<u8> で表現されています。
// ただし、特定のメソッド呼び出しからしか編集出来ないようになっています。
// なぜなら、Stringが不正なUTF-8文字を弾くよう設計されているからです。
