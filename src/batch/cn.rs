use aoko::no_std::{functions::ext::{BoolExt, Utf8Ext}, pipelines::pipe::Pipe};
use itertools::Itertools;
use std::fs;

// 定义标点符号 (punctuation)
const P: [char; 34] = [' ', '\r', '\n', '，', '。', '！', '？', '“', '”', '‘', '’', '：', '；', '(', ')', '（', '）', '「', '」', '『', '』', '[', ']', '【', '】', '〖', '〗', '{', '}', '《', '》', '—', '〔', '〕'];

pub fn batch_cn(dict: String, r#in: String, out: String) {
    // 读文件
    fs::read(r#in).unwrap()
        // 转字符串
        .to_str().unwrap()
        // 判断是否有字典 —— (1. 如果没有)
        .pipe(|s| if let None = dict.chars().next() {
            // 转字符
            s.chars()
            // 去标点，计数`1`
            .filter_map(|c| P.iter().all(|&p| c != p).if_true((c, 1)))
            // 按类分组(小写)
            .into_grouping_map_by(|t| t.0.to_string()) // Map { "word": [('char', 1), ...], ... }
            // 聚合:   累计             K          V  (K, 计数)
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
                // 映射，&str -> String (类型一致)
                .map(|(s, u)| (s.to_string(), u))
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
            .pipe(|byt| fs::write(out, byt).unwrap())
}