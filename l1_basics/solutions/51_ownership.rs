#![allow(dangling_pointers_from_temporaries)]

// 所有権は関数に譲渡することも出来ます。

// 引数としてベクタを受け取り、合計値を返す関数です。
fn sum(v: Vec<i32>) -> i32 {
    let mut res = 0;
    for x in v {
        res += x;
    }
    res
}

// 文字列を受け取り、後ろに "world" を付け足す関数です。
fn push_world(mut string: String) -> String {
    string.push_str("world");
    // 呼び出し元に引数の所有権をそのまま返すことも出来ます。
    string
}

fn main() {
    let numbers = vec![1, 2, 3];
    // 毎回クローンすれば何回でも sum 関数に渡すことができます。
    println!("sum(numbers.clone()) = {}", sum(numbers.clone()));
    println!("sum(numbers.clone()) = {}", sum(numbers.clone()));
    println!("sum(numbers.clone()) = {}", sum(numbers.clone()));
    // クローンしたベクタはデータの中身が同じでも違うヒープ領域に保存されています。
    assert_ne!(numbers.as_ptr(), numbers.clone().as_ptr());
    // 一度でも元データを渡すと、所有権が関数内に移ります。
    println!("sum(numbers) = {}", sum(numbers));
    // 以降、変数 numbers は所有権が剥奪され使えなくなります。

    let mut hello = String::with_capacity(13); // 文字列 "Hello, world!" の長さ分容量を確保する。
    hello.push_str("Hello, ");
    let hello_ptr = hello.as_ptr(); // 生ポインタをコピーする。
    // hello 変数を関数に譲渡し、戻ってきた値を新たな変数に代入します。
    let hello_world = push_world(hello);
    // 以降、変数 hello は所有権が剥奪され使えなくなります。

    // push_world 関数に渡したStringと戻り値のStringは同じ生ポインタを持っています。
    // つまり、main::hello -> push_world::string -> main::hello_world
    // という順に値が複製されることなく所有権が渡されていったということです。
    assert_eq!(hello_ptr, hello_world.as_ptr());
    println!("{}!", hello_world);

    // Rustは、関数スコープが終わる前に drop() という関数を自動的に呼び出します。
    // drop() は確保されたメモリを開放する関数で、コピーが不可能で且つまだ所有権を持った変数が渡されます。
    // 今いる main() 関数スコープだと、
    //
    //     drop(hello_world);
    //
    // が呼び出されます。
    // hello_ptr はコピー可能型で、hello は所有権が剥奪されているためドロップされません。
    //
    // 変数 numbers については、関数 sum() に所有権が移動しています。
    // そのため、sum() 関数スコープの最後で
    //
    //     drop(v);
    //
    // されます。
    // ベクタや文字列をドロップすると、ヒープ領域に確保されたバッファーデータが開放されます。
    //
    // 初期化された値は、どれだけ所有者が移り変わっても最終的には必ず一度だけドロップされる。
    // それが、所有権の課すルールです。
}
