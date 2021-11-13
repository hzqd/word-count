use std::time::Duration;
use aoko::{no_std::ext::AnyExt1, standard::fun::{measure_time_with_value, time_conversion_with_unit}};
use world_count::batch::{cli::{Lang::*, get_args}, lang::{cn::word_count_cn, en::word_count_en}};

fn word_count_batch() -> (impl FnOnce(Duration) -> u128, String) {
    // 读取命令行参数
    let (dict, lang, r#in, out, unit) = get_args().let_owned(|s| (s.dictionary, s.subcmd, s.input, s.output, s.time));
    // 判断语言(子命令)
    match lang {
        EN => word_count_en(dict, r#in, out),
        CN => word_count_cn(dict, r#in, out),
    }
    // 返回转换函数和计时单位
    time_conversion_with_unit(unit)
}

fn main() {
    measure_time_with_value(|| word_count_batch())
        .let_owned(|((f, u), e)| println!("Execution time: {} {}.", f(e), u));
}