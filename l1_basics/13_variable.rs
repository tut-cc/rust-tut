fn main() {
    // 同じ変数名で再び宣言することもできます。
    // この言語機能は、Shadowing（影化）とも呼ばれます。
    let x = 1;
    println!("x = {x}");

    let ??? = -1;
    println!("shadowed x = {x}"); // shadowed x = -1 と出力されるようにしてください。

    let ??? = "king";
    println!("shadowed x = {x}"); // shadowed x = king と出力されるようにしてください。
}
