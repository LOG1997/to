use clap::{Parser, Subcommand};
#[derive(Debug, Subcommand)]
pub enum Action {
    #[command(
        about = "添加命令，格式为 to add [name] [value]",
        long_about = "如果需要参数，则在value中添加{}为占位符，执行时传入"
    )]
    Add { items: Vec<String> },

    #[command(
        about = "搜索命令",
        long_about = "格式为 to search [param] name和value都可以进行搜索"
    )]
    Search { query: Vec<String> },

    #[command(
        about = "删除指定命令",
        long_about = "可以连续删除多个，格式为 to delete [name1] [name2]..."
    )]
    Delete { ids: Vec<String> },

    #[command(about = "列出所有命令", long_about = "格式为 to list，不用参数")]
    List,
    #[command(
        about = "编辑配置文件",
        long_about = "格式为to edit，使用系统应用打开，如果要使用vim编辑请添加--vim参数"
    )]
    Edit {
        #[arg(short, long)]
        vim: bool,
    },
    #[command(about = "关于本应用的介绍信息")]
    About,
    #[command(external_subcommand)]
    Custom(Vec<String>),
}

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    #[command(subcommand)]
    pub action: Action,
}
