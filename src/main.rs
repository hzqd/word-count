use aoko::no_std::ext::{AnyExt1, Utf8Ext};
use itertools::Itertools;
use std::{env, fs};

fn main() {
    // 读取命令行参数
    let args = env::args().collect_vec();
    // 解构
    let (r#in, out) = (&args[1], &args[2]);
    // 读文件
    fs::read(r#in).unwrap()
        // 转字符串
        .to_str().unwrap()
        // 按空格切割
        .split_whitespace()
        // 映射，计数`1`
        .map(|s| (s, 1))
        // 按类分组
        .into_grouping_map_by(|t| t.0) // Map { key: [key, value] }
        // 聚合:  累计  K V(K, 计数)
        .fold(0, |acc, _, (_, count)| acc + count) // Map { key: value }
        // 迭代
        .iter()
        // 排序(从大到小)(高频到低频)
        .sorted_by_key(|v| v.1).rev()
        // 映射，转字符串
        .map(|t| format!("{:?}", t))
        // 合并字符串加换行
        .join("\n")
        // 输出文件
        .let_owned(|byt| fs::write(out, byt).unwrap());
}