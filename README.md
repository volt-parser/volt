# volt-rs / PEG Parser Volt

## 導入

## 利用

```rs
fn main() {
    // パーサ初期化
    let volt = &mut Volt::new();
    volt.set_max_recursion(1024);
    volt.add_module(MyModule::new());

    // 抽象構文木を生成
    let input = "volt";
    let entry_rule_id = "MyModule::syntax";
    let tree = volt.parse(volt, input, entry_rule_id);

    println!("{:?}", tree);
}

// モジュール定義
#[derive(RuleContainer)]
struct MyModule {
    syntax: Element,
}

// モジュールに構文規則を登録
impl Module for MyModule {
    fn new() -> MyModule {
        add_rules!{
            syntax := choice![str("volt"), str("watt")];
        }
    }
}
```

## 構文定義

### Expressions

|機能|構文|例|
|:-|:-|:-|
|選択|`choice![e1, e2, ...]`|`choice![str("volt"), str("watt")]`|
|連接|`seq![e1, e2, ...]`|`seq![str("volt"), str("watt")]`|
|規則|`Module::rule()`|`Symbol::spacing()`|
|文字列|`str(s: &str)`|`str("volt")`|
|ワイルドカード|`wildcard()`|`wildcard()`|
|文字クラス|`chars(patt: &str)`|`chars("[0-9a-z]")`|

### Modifiers

|機能|構文|例|
|:-|:-|:-|
|n回|`times(n: usize)`|`times(2)`|
|n回以上|`min(n: usize)`|`min(0)`|
|n回以下|`max(n: usize)`|`max(1)`|
|0回もしくは1回|`optional()`|`optional()`|
|肯定先読み|`poslook()`|`poslook()`|
|否定先読み|`neglook()`|`neglook()`|

### Additional Features

|機能|構文|例|説明|
|:-|:-|:-|:-|
|グループ化|`group(name: &str)`|`group("syntax")`|子要素をグループ化する|
|展開|`expand()`|`expand()`|ノードの全階層の子要素を親に展開する|
|展開|`expand_once()`|`expand_once()`|ノードの1階層の子要素を親に展開する|
|結合|`join(e: Element)`|`join(seq![str("volt"), str("watt")])`|子要素を1つのリーフに結合する|
||`reduce`||子要素を加工する|
|隠蔽|`hide()`|`hide()`|生成された要素を構文木に反映しない|
