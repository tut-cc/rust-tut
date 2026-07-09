#![allow(clippy::while_let_on_iterator)]
#![allow(dead_code)]

// Optionは「値の有無」という一般的な概念を体現した型です。
// そのため、標準ライブラリの至るところで登場します。

struct Character {
    name: String,
    role: Option<Role>,
}

#[derive(Debug)] // println! で {:?} が使えるようになるおまじない
enum Role {
    Wizard,
    Thief,
    Bard,
    Warrior,
}

fn main() {
    let mut characters = vec![
        Character {
            name: "Alice".to_string(),
            role: Some(Role::Bard),
        },
        Character {
            name: "Bob".to_string(),
            role: Some(Role::Wizard),
        },
        Character {
            name: "Charlie".to_string(),
            role: Some(Role::Thief),
        },
    ];

    // last_mut メソッドを使うと、最後の要素への占有参照を取得できます。
    // ただし、ベクタが空の場合もあるので返り値はOptionになります。
    if let Some(last) = characters.last_mut() {
        // 盗人は役職を剥奪される運命にあるみたいです。
        last.role = None;
    };

    // 続いて、各キャラクターの役職名を出力してみましょう。
    // ベクタはイテレータになれる型なのでfor文を使えます。
    // ですが、ここではあえてイテレータを直接操作してみます。

    // ベクタの into_iter() メソッドでイテレータを取得できます。
    let mut it = characters.into_iter();
    // イテレータのnextメソッドは、次の要素があったら Some、無かったら None を返します。
    // while let は、パターンマッチし続ける間実行を繰り返します。
    while let Some(Character { name, role }) = it.next() {
        print!("The role of {name} is ");
        if let Some(role) = role {
            println!("{role:?}."); // 役職あり
        } else {
            println!("not assigned."); // 役職なし
        }
    }
}

// 最後のイテレータ操作をfor文で書き直すと次のようになります。
//
//     for Character { name, role } in characters {
//         // ...
//     }
