
# ``注意``

`暂时只支持对于整型和字符串的set （测试用例还不够全面）`
`不支持u128`
# 安装

``` Cargo
[dependencies]
set_derive = {git="https://github.com/qiongtubao/latte_rust_set_derive"}
latte_verify = {git = "https://github.com/qiongtubao/latte_rust_verify"}
```


# 例子

```rust
extern crate set_derive;
use set_derive::*;
extern crate latte_verify;
use latte_verify::*;
#[derive(Set)]
struct Struct {
    #[verify{"$gt":10, "$lt":100, "$in":[11,12,13,14]}]  
    x: i32,
    #[verify{"$regex": r"(\d{4})-(\d{2})-(\d{2})"}]
    y: String
}
fn main() {
    //缺陷在于初始化的时候并没有进行验证  之后可能会推出new方法辅助创建对象
    let s = Struct {
        x: 30,
        y: "1991-02-03".to_string(),
    };
    match s.setY("1991-02-03".to_string()) {
        Ok(v) => {
            println!("ok");
        }
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
}

```

## 支持类型

## 整型

### 支持属性
* $gt 大于
* $gte 大于等于
* $lt 小于
* $lte 小于等于
* $in 枚举
* $nin 不在此枚举范围内
* $ne 不等于
* $eq 等于

## String
### 属性
* $regex   正则表达式  这里用到的是[regex库](https://github.com/rust-lang/regex)