// 標準ライブラリの中には言語機能と密接に関わるトレイトが数多く存在します。
//
//     Display::fmt                {} で値のフォーマット化
//     Debug::fmt                  {:?} で値のフォーマット化
//     <Into<T, U>>::into,         確実な型の変換
//        <From<T>>::from
//     <TryInto<T, U>>::try_into,  不確実な型の変換
//        <TryFrom<T>>::try_from
//     Copy                        コピー
//     Clone::clone                クローン
//     Iterator::next              イテレータの次の要素を取得
//     IntoIterator::into_iter     イテレータになる
//     Deref::deref                参照外し
//
// 基本データ型がprintln!で使えること、整数型どうしの型変換、
// スライス・配列・ベクタがfor文で使えるのはこのようなトレイトの実装があるからです。
//
// このエクササイズでは複素数を新たに定義して、その足・引・掛け算を実装してみます。

// #[derive()] は、トレイトを自動で実装するマクロです。
// f64 は Debug, Clone, Copy を実装しています。
// それを2つ組み合わせた Complex も Debug, Clone, Copy が実装できるということです。
#[derive(Debug, Clone, Copy)]
struct Complex {
    re: f64, // 実部
    im: f64, // 虚部
}

impl Complex {
    fn new(re: f64, im: f64) -> Self {
        Complex { re, im }
    }
}

// 足し算の実装です。
impl std::ops::Add for Complex {
    // type 名前 = 型; は関連型と呼ばれるます。
    // トレイト専用の、<T> とは少し勝手が違う型変数です。
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let re = self.re + rhs.re;
        let im = self.im + rhs.im;
        Complex { re, im }
    }
}

// 引き算の実装です。
impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let re = self.re - rhs.re;
        let im = self.im - rhs.im;
        Complex { re, im }
    }
}

// 掛け算の実装です。
impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        // (a + ib) * (c + id) = (???) + i(ad + bc)
        let re = ???;
        let im = self.re * rhs.im + self.im * rhs.re;
        Complex { re, im }
    }
}

fn main() {
    let a = Complex::new(2., 3.);
    let b = Complex::new(0.5, 1.);
    let c = Complex::new(-4.0, 2.5);

    println!("a = {a:?}");
    println!("b = {b:?}");
    println!("c = {c:?}");

    // 足し算、引き算、掛け算を定義したので、演算子 + - * が使えます。
    println!("a + b = {:?}", a + b);
    println!("a - b = {:?}", a - b);
    println!("a * b = {:?}", a * b);
    println!("b * c = {:?}", b * c);
    println!("c * a = {:?}", c * a);
}
