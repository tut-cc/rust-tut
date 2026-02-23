// while文を使うと条件が偽になるまで繰り返し実行できます。
//
//     while 条件 {
//         文1;
//         文2;
//         ...
//     }

fn main() {
    println!("sum_power_of_twos(100) = {}", sum_power_of_twos(100));
}

// 2からnのうち2のべき乗だけ足し合わせる関数を実装しましょう。
fn sum_power_of_twos(n: u32) -> u32 {
    let mut sum = 0;

    // 2のべき乗を保存する関数です。
    let mut i = 2;
    while i <= n {
        sum += i;
        i *= 2;
    }

    sum
}
