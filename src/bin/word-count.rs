use std::time::Duration;
use aoko::{no_std::pipelines::pipe::Pipe, standard::functions::fun::{measure_time_with_value, time_conversion_with_unit}};
use world_count::{batch::{cn::batch_cn, en::batch_en}, TimeUnit, get_args, Mode::*};

fn word_count() -> (impl FnOnce(Duration) -> u128, TimeUnit) {
    // 读取命令行参数
    let (dict, mode, unit) = get_args().pipe(|s| (s.dictionary, s.subcmd, s.time));
    // 判断(批/流)处理(子命令)
    match mode {
        batch { input, output, en_or_cn } => if en_or_cn { batch_en(dict, input, output) } else { batch_cn(dict, input, output) }
        stream { ip: _, en_or_cn: _ } => (), // todo：待实现
    }
    // 返回转换函数和计时单位
    time_conversion_with_unit(unit)
}

fn main() {
    measure_time_with_value(word_count)
        .pipe(|(e, (f, u))| println!("Execution time: {} {:?}.", f(e), u));
}