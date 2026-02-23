// ifブロックは式として扱うこともできます。

// 番号に対応した役職名を出力するプログラムです。
fn main() {
    println!("{}", role_repr(0)); // Wizard
    println!("{}", role_repr(1)); // Thief
    println!("{}", role_repr(2)); // Bard
    println!("{}", role_repr(3)); // Warrior
    println!("{}", role_repr(4)); // Unknown role
}

fn role_repr(role: u8) -> &'static str {
    if role == 0 {
        "Wizard"
    } else if role == ??? {
        "Thief"
    } else if role == ??? {
        "Bard"
    } else if role == ??? {
        "Warrior"
    } ??? {
        panic!("Unknown role: {role}") // プログラムを強制終了する
    }
}

// role_repr の定義は複数行にまたがっていますが、セミコロンが無いので文がありません。
// 各々の条件ブロックが式になっていて、その集合もまた式になっているのです。
//
//     if 条件0 {
//         式0
//     } else if 条件1 {
//         式1
//     } else if 条件2 {
//         式2
//     } else if 条件3 {
//         式3
//     } else {
//         式4
//     }
//
// ところで、elseブロックは文字列型ではなくNever型を返しています。
// panic! はその場でプログラムを強制終了させるため、以降プログラムが実行され**ない**ことが保証されています。
// そのため、戻り値が文字列でなくても問題ありません。
