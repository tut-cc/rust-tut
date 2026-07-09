#![allow(clippy::question_mark)]

use std::num::ParseIntError;

// Resultは「処理の成功・失敗」という一般的な概念を体現した型です。
// そのため、標準ライブラリの至るところで登場します。

// 16進数の文字列から8ビットの符号なし整数に変換します。
//
// 引数がどちらともu8に変換できたら、足し算の結果を返します。
// もし変換に失敗したら整数解析エラー (ParseIntError) を返します。
fn add_base_16_nums(a: &str, b: &str) -> Result<u8, ParseIntError> {
    // u8::from_str_radix は文字列と基数を受け取って数字に変換する関数です。
    // ただし、文字列が数字のフォーマットに沿っているとは限りません。
    // ここではmatch式を用いて、エラーの場合は add_base_16_nums の返り値としています。
    let a = match u8::from_str_radix(a, 16) {
        Ok(int) => int,
        Err(err) => return Err(err),
    };

    // 上記のエラー記法は ? 記法で大幅に省略できます。
    let b = u8::from_str_radix(b, 16)?;

    // a + b と記述するとオーバーフローする可能性があります。
    // ここでは、オーバーフローを許容するために u8::wrapping_add を用います。
    Ok(a.wrapping_add(b))
}

// main関数は結果型を返すことも出来ます。
// 正常値はユニットで、エラー値は整数解析エラーです。
fn main() -> Result<(), ParseIntError> {
    let zero = add_base_16_nums("00", "00").unwrap();
    // {:x} で数字を16進数表記で出力できます。
    println!("0x00 + 0x00 = 0x{zero:x}");
    // ? 記法はmain関数内でも使えます。
    let num = add_base_16_nums("11", "aa")?;
    println!("0x11 + 0xaa = 0x{num:x}");

    // 続いて、u8::from_str_radix がエラーとなるケースを見てみましょう。
    // g は16進数 [0123456789ABCDEF] の範囲外です。
    println!("hex(0g): {:?}", u8::from_str_radix("g", 16).unwrap_err());
    // 0x100 はu8の表現できる整数の範囲外です。
    println!("hex(100): {:?}", u8::from_str_radix("100", 16).unwrap_err());

    // 0x12 + 0x34 の行が出力された直後に、main関数を終了させてください。

    println!("0x12 + 0x34 = 0x{:x}", add_base_16_nums("12", "34")?);
    // 0x2A = 2*16+10 = 42 => The Universe is filled with EMPTINESS....
    println!("0x__ + 0x2A = 0x{:x}", add_base_16_nums("", "2A")?);
    println!("0xab + 0xcd = 0x{:x}", add_base_16_nums("ab", "cd")?);

    Ok(())
}

// エラー処理は冗長になりがちです。
// Rustは ? 記法があるおかげで大分簡潔に記述できることが多いです。
