#![allow(dead_code)]

// 構造型 struct を使うと複数のデータをまとめた型を定義できます。
// タプル型も似た機能を持ちますが、データにアクセスするときに .数字 を用いていました。
// 構造型は .名前 を使ってデータにアクセスします。

struct Character {
    // `名前: 型` でフィールドを定義します。
    name: String,
    health: i32,
    // 列挙型も使えます。
    role: Role,
}

enum Role {
    Wizard,
    Thief,
    Bard,
    Warrior,
}

// 構造型も列挙型と同様に、implブロックでメソッドを定義できます。
impl Character {
    // 現在のステータスを表示します。
    //
    // &self ではなく単に self と書くと、所有権が移動してしまいます。
    // Character はコピー不可能なので、メソッドが2回目以降呼び出せなくなってしまいます。
    // ここでは、借用を意味する &self を使います。
    fn status(&self) {
        if self.is_unconscious() {
            println!("{} is unconcious...", self.name);
        } else {
            println!("{} has {} health.", self.name, self.health);
        }
    }

    // 意識を失っているかどうか判別します。
    fn is_unconscious(&self) -> bool {
        self.health <= 0
    }

    // 他のキャラクターを攻撃します。
    fn attack(&self, other: &mut Character) {
        // 自分が意識を失っているときは動くことが出来ません。
        if self.is_unconscious() {
            return;
        }
        // この世界では気絶した相手への攻撃が禁止されています。
        if other.is_unconscious() {
            panic!("{} IS BREAKING THE RULEZZ!!!!!", self.name.to_uppercase());
        }

        // 役職ごとに分岐して、セリフとダメージ量をタプルとして取得します。
        // このlet文は、そのタプルをパターン分解して変数宣言しています。
        let (message, damage) = match self.role {
            Role::Wizard => ("I hereby summon lighting in the field!", 50),
            Role::Thief => ("What wallet? Not me.", 25),
            Role::Bard => ("Anyway, here's Wonderwall...", 25),
            Role::Warrior => ("SMASH!", 55),
        };
        println!("{} steps forward: {message}", self.name);
        other.health -= damage;
    }

    // 体力を回復します。
    fn heal(&mut self, amount: i32) {
        self.health += amount;
    }
}

fn main() {
    let mut wanda = Character {
        name: "Wanda".to_string(),
        health: 105,
        role: Role::Wizard,
    };
    let mut thanos = Character {
        name: "Thanos".to_string(),
        health: 160,
        role: Role::Warrior,
    };

    wanda.status();
    thanos.status();
    println!();

    wanda.attack(&mut thanos);
    thanos.attack(&mut wanda);
    wanda.status();
    thanos.status();
    println!();

    // 最後の足掻きで 1 HP だけ残るよう回復してください。
    wanda.heal(6);

    wanda.attack(&mut thanos);
    thanos.attack(&mut wanda);
    wanda.status();
    thanos.status();
    println!();

    wanda.attack(&mut thanos);
    thanos.attack(&mut wanda);
    wanda.status();
    thanos.status();
}

// 構造型は、タプルもしくはユニット形式で定義できます。
//
//     struct Point(i32, i32, i32);
//     struct AlwaysEqual;
//
// わざわざ構造型として名前を付けて定義すると、implブロックが使えるように成ります。
// 上記のPoint型だったら、3次元空間での演算を専用のメソッドとして定義できるということです。
// ユニット構造型は、データを持たないマーカーとして使うこともあります。
