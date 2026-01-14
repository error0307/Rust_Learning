# Rust学習記録その2 - 所有権について
Rustの重要な概念である所有権についての学習記録です。

## 「所有権」とは  
変数に値を割り当てる基本的なプログラミング操作における、Rust特有のメモリ管理の仕組み。
変数に値が割り当てられた場合、その変数がその値の「所有権」を持つことになる。

## 所有権の移譲
「所有権」は変数間で移譲可能です。
`変数B = 変数A`は、他のプログラミング言語では変数Aの値を変数Bにコピーする操作(コピーセマンティクス)となります。  
しかし、Rustでは基本的に **「所有権」が変数Aから変数Bへ移譲される** 操作(ムーブセマンティクス)となります。 

* サンプルコード
```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // 所有権がs1からs2に移譲される

    println!("{}", s1); // ここでコンパイルエラー: s1はもう使用できない
    println!("{}", s2); // s2は有効
}
```
* 出力(コンパイルエラー)
```
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:20
  |
2 |     let s1 = String::from("Hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1; // 所有権がs1からs2に移譲される
  |              -- value moved here
4 |
5 |     println!("{}", s1); // ここでコンパイルエラー: s1はもう使用できない
  |                    ^^ value borrowed here after move
  |
```

「所有権」が移譲された変数(s1)は、以降使用できなくなります。
しかし、基本型(整数型、浮動小数点型、ブーリアン型、文字型など)やCopyトレイトを実装している型の場合は、所有権の移譲ではなくコピーが行われます。

**サンプルコード**
```rust
fn main() {
    let x = 5;
    let y = x; // xの値がyにコピーされる

    println!("x: {}, y: {}", x, y); // xもyも有効
}
```

**出力**
```
x: 5, y: 5
```

### Copyトレイトを実装している型

| 型 | 説明 | 例 |
|---|---|---|
| 整数型 | i8, i16, i32, i64, i128, isize | `let x = 5;` |
| 符号なし整数型 | u8, u16, u32, u64, u128, usize | `let x = 5u32;` |
| 浮動小数点型 | f32, f64 | `let x = 3.14;` |
| 真偽値型 | bool | `let x = true;` |
| 文字型 | char | `let x = 'a';` |
| タプル（全要素がCopy） | (i32, i32), (bool, char) など | `let x = (1, 2);` |
| 配列（要素がCopy） | [i32; 5], [bool; 3] など | `let x = [1, 2, 3];` |
| 参照 | &T, &mut T | `let x = &5;` |
| 関数ポインタ | fn(i32) -> i32 | `let f = some_fn;` |

**重要な補足**：参照型（`&T`や`&mut T`）は**参照自体**がCopyですが、参照が指す先の値がCopyという意味ではありません。例えば、`&String`はCopyですが、`String`自体はCopyではありません。

```rust
let s = String::from("hello"); // StringはCopy非対応
let r = &s; // &StringはCopyだが、Stringはコピーできない
let r2 = r; // 参照がコピーされるため、rとr2の両方が有効
```

### Copyトレイトを実装していない型

| 型 | 説明 | 例 |
|---|---|---|
| String | ヒープ上の可変長文字列 | `let s = String::from("hello");` |
| Vec<T> | 動的配列 | `let v = vec![1, 2, 3];` |
| Box<T> | ヒープ上のポインタ | `let b = Box::new(5);` |
| HashMap<K, V> | ハッシュマップ | `let mut map = HashMap::new();` |
| タプル（非Copy要素含む） | (String, i32) など | `let x = (String::from("a"), 1);` |
| 配列（非Copy要素） | [String; 3] など | `let arr = [String::from("a")];` |
| 構造体（非Copy要素含む） | struct Person { name: String } | `let p = Person { name: s };` |
| ファイルハンドル | std::fs::File | `let f = File::open("file.txt")?;` |



## 「所有権」の借用
関数に引数として変数を渡す際や、変数の値を一時的に使用したい場合、都度「所有権」を移譲するのは実装負担が大きいため、  
「所有権」を移譲せずに一時的に「借用」する仕組みがあります。    
具体的には、参照(&)を使用して「借用」を実現します。  
この場合、借用元から「所有権」は移譲されないため、借用元の変数は引き続き有効となります。

**サンプルコード**
```rust
fn main() {
    let s1 = String::from("Hello");
    let r1 = &s1; // 所有権を借用

    println!("Length of '{}' is {}", s1, r1); // s1, r1は両方とも有効
}
```

**出力**
```
Length of 'Hello' is Hello
```

## 参照の種類
参照には「不変な参照(&)」と「可変な参照(&mut)」の2種類があり、「借用」の仕組みも少し異なります。

### 不変な参照(&)  
不変な参照は、無制限に複数の参照を借用可能です。

**サンプルコード**
```rust
fn main() {
    let s1 = String::from("Hello");
    let r1 = &s1; // 不変な参照
    let r2 = &s1; // もう一つの不変な参照

    println!("{} and {}", r1, r2);
}
```

**出力**
```
Hello and Hello
```

不変のため、借用先で値を変更することはできません。

**サンプルコード**
```rust
fn main() {
    let s1 = String::from("Hello");
    let r1 = &s1; // 不変な参照
    let r2 = &s1; // もう一つの不変な参照

    r1.push_str(", world!"); // r1を通じてs1を変更

    println!("{} and {}", r1, r2); // ここでコンパイルエラー: 不変な参照は変更できない
    println!("{}", s1); // s1は有効
}
```

**出力（コンパイルエラー）**
```
error[E0596]: cannot borrow `*r1` as mutable, as it is behind a `&` reference
 --> src/main.rs:6:5
  |
6 |     r1.push_str(", world!"); // r1を通じてs1を変更
  |     ^^ `r1` is a `&` reference, so the data it refers to cannot be borrowed as mutable
```

### 可変な参照(&mut)  
可変な参照(&mut)は、その名の通り可変のため、借用先で値を変更することが可能です。
借用先で変更した場合は、借用元の変数も変更されます。

**サンプルコード**
```rust
fn main() {
    let mut s1 = String::from("Hello");
    let r1 = &mut s1; // 可変な参照

    r1.push_str(", world!"); // r1を通じてs1を変更

    println!("{}", r1); // r1は有効
    println!("{}", s1);
}
```

**出力**
```
Hello, world!
Hello, world!
```
  
しかし、複数の借用先で値が変更されると整合性が保てなくなる恐れがあるため、**同時に複数の可変な参照を借用することはできません。**

**サンプルコード**
```rust
fn main() {
    let mut s1 = String::from("Hello");
    let r1 = &mut s1; // 可変な参照
    let r2 = &mut s1; // ここでコンパイルエラー: 同時に複数の可変な参照はできない
    
    println!("{} and {}", r1, r2);
}
```

**出力（コンパイルエラー）**
```
error[E0499]: cannot borrow `s1` as mutable more than once at a time
 --> src/main.rs:4:14
  |
3 |     let r1 = &mut s1; // 可変な参照
  |              ------- first mutable borrow occurs here
4 |     let r2 = &mut s1; // ここでコンパイルエラー: 同時に複数の可変な参照はできない
  |              ^^^^^^^ second mutable borrow occurs here
5 |     
6 |     println!("{} and {}", r1, r2);
  |                           -- first borrow later used here
```

また、不変な参照(&)は、その値が変更されないことを保証するため、
同時に可変な参照(&mut)を借用することもできません。

**サンプルコード**
```rust
fn main() {
    let mut s1 = String::from("Hello");
    let r1 = &s1; // 不変な参照
    let r2 = &mut s1; // ここでコンパイルエラー: 不変な参照と可変な参照は同時に使用できない

    println!("{} and {}", r1, r2);
}
```

**出力（コンパイルエラー）**
```
error[E0502]: cannot borrow `s1` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:14
  |
3 |     let r1 = &s1; // 不変な参照
  |              --- immutable borrow occurs here
4 |     let r2 = &mut s1; // ここでコンパイルエラー: 不変な参照と可変な参照は同時に使用できない
  |              ^^^^^^^ mutable borrow occurs here
5 |
6 |     println!("{} and {}", r1, r2);
  |                           -- immutable borrow later used here
```


## ライフタイム

Rustでは、変数や参照の有効期間を「ライフタイム」と呼びます。

### ライフタイムの開始と終了
ライフタイムはスコープによって決まります。

| タイミング | 説明 |
|---|---|
| 開始 | 変数や参照が宣言された時点 |
| 終了 | 変数や参照が所属するスコープを抜けた時点 |

**重要**：参照のライフタイムはスコープによって自動的に決まります。`drop()`を明示的に呼び出す必要はありません。

**サンプルコード**
```rust
fn main() {
    let s1 = String::from("Hello"); // s1のライフタイム開始
    {
        let r1 = &s1; // r1のライフタイム開始
        println!("{}", r1);
    } // r1のスコープ終了 → r1のライフタイム終了
    
    println!("{}", s1); // s1はまだ有効
} // s1のスコープ終了 → s1のライフタイム終了
```

借用した参照のライフタイムは、借用元のライフタイムを超えることはできません。

**サンプルコード**
```rust
fn main() {
    let mut s1 = String::from("Hello"); // |s1のライフタイム開始
    let r1 = &s1; //                       |  |r1のライフタイム開始
    drop(s1); //                           |s1のライフタイム終了
    println!("{}", r1); // ここでコンパイルエラー: r1のライフタイムがs1のライフタイムを超えている
}
```

**出力（コンパイルエラー）**
```
error[E0505]: cannot move out of `s1` because it is borrowed
 --> src/main.rs:4:10
  |
2 |     let s1 = String::from("Hello"); // |s1のライフタイム開始
  |         -- binding `s1` declared here
3 |     let r1 = &s1; //                       |  |r1のライフタイム開始
  |              --- borrow of `s1` occurs here
4 |     drop(s1); //                           |s1のライフタイム終了
  |          ^^ move out of `s1` occurs here
5 |     println!("{}", r1); // ここでコンパイルエラー: r1のライフタイムがs1のライフタイムを超えている
  |                    -- borrow later used here
  |
```

不変な参照と可変な参照のライフタイムは重複できません。と前述しました。
それは、両者のライフタイムが重複している場合に、不整合が発生する可能性があるためです。
つまり、両者のライフタイムが重複していなければ、そのような心配は不要となります。

このように、ライフタイムが重複しないように工夫することで、以下のようにコンパイルエラーを回避できます。(Rust 1.31以降)

* サンプルコード
```rust
fn main() {
    let mut s1 = String::from("Hello"); // |s1のライフタイム開始
    let r1 = &s1; //                         |  |r1のライフタイム開始
    let r2 = &mut s1;//                      |  |ここでコンパイルエラー: 不変な参照と可変な参照のライフタイムが重複している
    println!("{} and {}", r1, r2); //        |  |
}
```

**出力（コンパイルエラー）**
```
error[E0502]: cannot borrow `s1` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:14
  |
3 |     let r1 = &s1; //                         |  |r1のライフタイム開始
  |              --- immutable borrow occurs here
4 |     let r2 = &mut s1;//                      |  |ここでコンパイルエラー: 不変な参照と可変な参照のライフ                       ...
  |              ^^^^^^^ mutable borrow occurs here
5 |     println!("{} and {}", r1, r2); //        |  |
  |                           -- immutable borrow later used here

```

**サンプルコード（成功例）**
```rust
fn main() {
    let mut s1 = String::from("Hello"); //   |s1のライフタイム開始
    let r1 = &s1; //                         |  |r1のライフタイム開始, しかし使用されないためすぐに終了
    let r2 = &mut s1; //                     |  |r2のライフタイム開始
    println!("{}", r2); //                   |  |不変な参照と可変な参照のライフタイムは重複していないため、r2は有効
}
```
* 出力
```
Hello
```

## 明示的なライフタイム注釈
関数の引数や戻り値に参照を使用する場合、コンパイラはライフタイムを自動的に推論します。
しかし、場合によっては自動推論ができず、コンパイルエラーとなることがあります。

**サンプルコード**
```rust
fn longest<'a>(s1: &str, s2: &str) -> &str {　// ここでコンパイルエラー: 戻り値がどの参照に対応するか不明確
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
fn main() {
    let string1 = String::from("long string is long");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

**出力（コンパイルエラー）**
```
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:39
  |
1 | ...'a>(s1: &str, s2: &str) -> &str {　// ここでコンパイルエラー: 戻り値がどの参照に対応するか不明確
  |            ----      ----     ^ expected named lifetime parameter
  |
```

その場合、関数のシグネチャで `<'a>` のように、明示的にライフタイム注釈を指定する必要があります。

**サンプルコード（修正後）**
```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str { // 'aライフタイム注釈を追加
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}
fn main() {
    let string1 = String::from("long string is long");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

**出力**
```
The longest string is long string is long
```

また、構造体のフィールドに参照を持たせる場合や
`impl`ブロックで参照を扱う場合も、明示的にライフタイム注釈を指定する必要があります。

**サンプルコード**
```rust
struct ImportantExcerpt<'a> { // 'aライフタイム注釈を追加
    part: &'a str, // フィールドが参照を持つため、'aライフタイム注釈を追加
}
impl<'a> ImportantExcerpt<'a> { // 'aライフタイム注釈を追加
    fn words(&self) -> usize { // selfが参照を持つため、'aライフタイム注釈を追加
        self.part.split_whitespace().count()
    }
}
fn main() {
    let novel = String::from("Hello, My Name is Keisuke. Nice to Meet you.");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("First sentence: {}, Words: {}", i.part, i.words());
}
```

**出力**
```
First sentence: Hello, My Name is Keisuke, Words: 5
```

## まとめ
他のプログラミング言語では、変数への値の割り当てや関数への引数渡しについて特に深く考える必要がありません。
しかし、Rustではこの部分に対して、所有権の概念を明確に意識しながら実装する必要があります。

車載ソフトなど、安全性が求められるシステムを構築する際には、メモリ管理を意識した堅牢な設計が必要です。
Rustの所有権システムは、そのような設計を言語レベルでサポートする強力な仕組みです。  

所有権と参照、ライフタイムをしっかり理解して、Rustを使いこなしていきましょう。
