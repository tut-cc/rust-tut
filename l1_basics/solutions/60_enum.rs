#![allow(dead_code)]

// 列挙型 enum を使うと複数の候補を持つ値を定義できます。

// 以前に出てきた4つの役職を、番号ではなく列挙型で表現してみましょう。
enum Role {
    Wizard,
    Thief,
    Bard,
    Warrior,
}

// implブロックを使って、列挙型固有のメソッドを定義できます。
impl Role {
    // メソッドは通常の関数と同じように定義できます。
    // 以下の関数は Role::description() で呼び出せます。
    fn description() {
        println!("Select your role...");
    }

    // 最初の引数を self にすると、 列挙値.play() で呼び出せるようにもなります。
    // self の型はimplブロックの型、つまり Role になります。
    fn play(self) {
        // 列挙値はmatch式で分岐できます。
        match self {
            Role::Wizard => println!("I hereby summon lighting in the field!"),
            Role::Thief => println!("What wallet? Not me."),
            Role::Bard => println!("Anyway, here's Wonderwall..."),
            // 戦士のときは SMASH! と表示してください。
            Role::Warrior => println!("SMASH!"),
        }
    }
}

fn main() {
    Role::description();

    // Role::play(role) でメソッドを呼び出せます。
    let thief = Role::Thief;
    Role::play(thief);

    // Role::play() は self を受け取るメソッドなので、 列挙値.play() で呼び出せます。
    let bard = Role::Bard;
    bard.play();

    // 値をわざわざ変数に格納しなくても、直接呼び出せます。
    Role::Warrior.play();
}

// 役職を番号で管理していたときは、0〜3番までしか使っていないのに4番以上も考慮しなければなりませんでした。
// 列挙型とmatchを組み合わせることで、漏れなく定義できる上に余すことなく条件分岐できるのです。
