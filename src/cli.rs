use clap::{Parser};

#[derive(Parser, Debug)]
#[clap(version, author, about)]
pub struct Cli {
    /// 搜索内容正则匹配
    #[clap(long, default_value = "")]
    pub pattern: String,

    /// 替换后的内容
    #[clap(long, default_value = "")]
    pub replace_with: String,

    /// 指定搜索的目录
    #[clap(long, default_value = "./")]
    pub search_path: String,

    /// 确认内容替换
    #[clap(long)]
    pub confirm: bool,
}

