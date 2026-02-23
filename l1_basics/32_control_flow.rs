// match式はif式と似ていて、条件に基づいて実行する文や式を切り替えることができます。
//
// match 入力式 {
//     候補0 => 分岐0,
//     候補1 => 分岐1,
//     候補2 => 分岐2,
//     ...
//     例外 => 例外分岐,
// }
//
// match式は入力式が型が持ちうるすべての候補を実装する必要があります。

// 前回と同じプログラムをif式ではなくmatch式で実装してみましょう。
fn main() {
    println!("{}", role_repr(0)); // Wizard
    println!("{}", role_repr(1)); // Thief
    println!("{}", role_repr(2)); // Bard
    println!("{}", role_repr(3)); // Warrior
    println!("{}", role_repr(4)); // Unknown role
}

fn role_repr(role: u8) -> &'static str {
    match role {
        0 => "Wizard",
        ??? => "Thief",
        ??? => "Bard",
        ??? => "Warrior",
        unk_role => panic!("Unknown role: {unk_role}"), // プログラムを強制終了する
    }
}

// 最後の unk_role はワイルドカードといい、他のどの候補にも当てはまらなかったときに実行されます。
// ワイルドカードは unk_role に限らず unknown や unk という変数名を付けてもいいです。
// また、変数定義を無視するアンダースコア _ を使うことも出来ます。
//
// ifとmatchにはパターンマッチという強力な機能があります。ワイルドカードもその1つの機能です。
// パターンマッチについては、enum/structやOption/Result型が出てきたときに少しずつ解説します。
