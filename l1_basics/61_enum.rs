#![allow(dead_code)]
#![allow(clippy::single_match)]

// 列挙候補は単に識別子としてだけでなく、データを紐づけることができます。

enum Role {
    // 魔法使いには整数値を紐づけます。
    Wizard(u64), // マナの総量
    // 盗人にはタプル型のような .数字 でアクセスできる複合型を紐づけます。
    Thief(bool, u64), // .0 = 駆け出し; .1 = 金
    // 吟遊詩人には数字ではなく .フィールド名 でアクセスできる複合型を紐づけます。
    Bard { novice: bool, songs: Vec<String> },
    // 戦士には別の列挙型をもたせます。
    Warrior(Weapon), // 武器
}

#[derive(Debug)] // println! で {:?} が使えるようになるおまじない
enum Weapon {
    Knife,
    Axe,
    Mace,
    Sword,
}

fn main() {
    let mana = 512;
    let wizard = Role::Wizard(mana);
    match wizard {
        Role::Wizard(wizard_mana) => println!("The wizard has {wizard_mana} mana."),
        // アンダースコアで他のパターンを _ で無視することもできます。
        _ => {}
    }

    let thief = Role::Thief(true, 12);
    // if let分を使うと、上のようなmatch式をより簡潔に記述できます。
    if let Role::Thief(beginner, money) = thief {
        let adj = if beginner { "expert" } else { "amateur" };
        println!("The {adj} thief has {money} gold.")
    };

    let bard = Role::Bard {
        novice: false,
        songs: vec![
            "Beowulf".to_string(),
            "The Iliad and the Odyssey".to_string(),
        ],
    };
    match bard {
        // novice の真偽値でパターンを分けて書くことができます。
        Role::Bard {
            novice: true,
            songs,
        } => println!("The amateur bard can sing: {:?}", songs),
        Role::Bard {
            novice: false,
            songs,
        } => println!("The veteran bard can sing: {:?}", songs),
        _ => println!("Only bards shall sing in this world. (sadly)"),
    }

    // The warrior is equipped with: Mace と表示されるようにしてください。
    let warrior = Role::Warrior(???);
    // if let は else ブランチでも同じ型の値を返すことで式としても使えます。
    let weapon = if let Role::Warrior(warrior_weapon) = warrior {
        warrior_weapon
    } else {
        Weapon::Knife // どの役職も持っているデフォルト装備
    };
    println!("The warrior is equipped with: {weapon:?}");
}
