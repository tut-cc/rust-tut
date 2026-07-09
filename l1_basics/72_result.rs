// 結果型 Result<T, E> はエラーの有無を示す列挙型です。
//
//     enum Result<T, E> {
//         Ok(T),
//         Err(E),
//     }
//
// Err(E) はエラーの存在を意味します。
// Ok(E) は正常に処理されたことを意味します。
// Option<T> とは違い、エラーの場合に何かしらの値が紐づけられています。
//
// ここで T, E は <T, E> で定義された型変数で、任意の型を意味します。
// 型ジェネリクスと呼ばれるこの機能については、後々詳しく解説します。(２回目)

#[derive(Debug)]
enum Role {
    Wizard,
    Thief,
    Bard,
    Warrior,
}

impl Role {
    // 役職IDを受け取って対応する役職を返す関数です。
    //
    // 対応する役職IDが無い場合はエラーとしてそのIDを返します。
    fn parse_id(id: u8) -> Result<Self, u8> {
        match id {
            0 => Ok(Role::Wizard),
            1 => Ok(Role::Thief),
            2 => Ok(Role::Bard),
            3 => Ok(Role::Warrior),
            unk => Err(unk),
        }
    }
}

fn main() {
    // 対応する役職IDがある場合は Ok() 、ない場合は Err になります。
    println!("Role 0 = {:?}", Role::parse_id(0));
    println!("Role 1 = {:?}", Role::parse_id(1));
    println!("Role 4 = {:?}", Role::parse_id(4));

    // Result::unwrap() メソッドを使うと、Ok() の中身を取り出すことができます。
    println!("Role 3 is {:?}.", Role::parse_id(3).unwrap());
    // ただしErrだった場合はパニックが起こり、以降実行が中断されます。

    // Optionとは違いエラーに紐づけられた値を取り出すこともできます。
    // この場合、Result::unwrap() ではなく Result::unwrap_err() を使います。
    println!("Unknown role ID = {}", Role::parse_id(4).unwrap_err());
    // ただしOkだった場合はパニックが起こり、以降実行が中断されます。

    // Resultは列挙型なので、if let 文でパターンマッチと変数宣言ができます。
    if let ???(unk) = Role::parse_id(42) {
        println!("Role {unk} doesn't exist but it sure has some finesse of ultimacy.");
    }
}
