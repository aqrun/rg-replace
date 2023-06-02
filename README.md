# rg-replace

使用rg(ripgrep)搜索指定目录匹配的文本，再全局替换

```
Usage: rg-replace [OPTIONS]

Options:
      --pattern <PATTERN>            搜索内容正则匹配 [default: ]
      --replace-with <REPLACE_WITH>  替换后的内容 [default: ]
      --search-path <SEARCH_PATH>    指定搜索的目录 [default: ./]
      --confirm                      确认内容替换
  -h, --help                         Print help
  -V, --version                      Print version
```


## 示例

```shell
cargo run -- --pattern "sea_orm_migration::prelude::\\*;" --replace-with "use sea_orm_migration::prelude::123::*"  --search-path /Users/migration --confirm 
```

## 依赖rg

rg 安装方式参考： https://docs.rs/crate/ripgrep

