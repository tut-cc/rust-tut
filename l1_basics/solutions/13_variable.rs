fn main() {
    let x = 1;
    println!("x = {x}");

    // 同じ変数名で再び宣言することもできます。
    let x = -1;
    println!("shadowed x = {x}"); // shadowed x = -1 と出力されるようにしてください。

    let x = "king";
    println!("shadowed x = {x}"); // shadowed x = king と出力されるようにしてください。
}

// この言語機能は、Shadowing（影化）とも呼ばれます。
// 影化された変数は再宣言後に使えなくなります。
