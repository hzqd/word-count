use aoko::no_std::ext::{AnyExt, AnyExt1, Utf8Ext};
use itertools::Itertools;
use std::fs;

pub fn word_count_en(dict: String, r#in: String, out: String) {
    // 定义标点符号
    let pp = ['"', '\'', '(', '[', '{', '<'].iter().map(|c| c.as_some()).collect_vec(); // prefix punctuation
    let sp = [',', '.', '!', '"', '\'', ':', ';', ')', ']', '}', '>', '-'].iter().map(|c| c.as_some()).collect_vec(); // suffix punctuation
    // 读文件
    fs::read(r#in).unwrap()
        // 转小写
        .to_ascii_lowercase()
        // 转字符串
        .to_str().unwrap()
        // 判断是否有字典 —— (1. 如果没有)
        .let_owned(|s| if let None = dict.chars().next() {
            // 按空格切割
            s.split_whitespace()
            // 去前缀
            .map(|s| if pp.contains(&s.chars().next().as_ref()) { &s[1..] } else { s })
            // 去后缀
            .map(|s| if sp.contains(&s.chars().rev().next().as_ref()) { &s[..s.len() - 1] } else { s })
            // 去空串
            .filter(|s| s.chars().next().is_some())
            // 映射，计数`1`
            .map(|s| (s, 1))
            // 按类分组(小写)
            .into_grouping_map_by(|t| t.0) // Map { "word": [("word", 1), ...], ... }
            // 聚合:  累计  K V(K, 计数)
            .fold(0, |acc, _, (_, count)| acc + count) // Map { "word": count, ... }
            // 迭代
            .into_iter()
            // 排序
            .sorted_by_key(|t| t.1)
        // 判断是否有字典 —— (2. 如果有)
        } else {
            // 读字典
            fs::read(dict).unwrap()
                // 转小写
                .to_ascii_lowercase()
                // 转字符串
                .to_str().unwrap()
                // 按行读(一行一词组)
                .lines()
                // 映射，[("word": count), ...]
                .flat_map(|phrase| s.matches(phrase).counts())
                // 排序
                .sorted_by_key(|t| t.1)
        })
            // 逆序(从大到小)(高频到低频)
            .rev()
            // 映射，转字符串
            .map(|t| format!("{:?}", t))
            // 合并字符串加换行
            .join("\n")
            // 输出文件
            .let_owned(|byt| fs::write(out, byt).unwrap())
}