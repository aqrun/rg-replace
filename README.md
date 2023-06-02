# rg-replace

使用rg(ripgrep)搜索指定目录匹配的文本，再全局替换


## 示例

```shell
cargo run -- --pattern "sea_orm_migration::prelude::\\*;" --replace-with "use sea_orm_migration::prelude::123::*"  --search-path /Users/migration --confirm 
```
