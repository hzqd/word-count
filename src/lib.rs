pub mod batch;  // 批处理
pub mod stream; // 流处理

pub use aoko::no_std::algebraic::sum::TimeUnit;
use clap::Parser;

/// Batch and stream processing word-count, support English & Chinese.
#[derive(Parser)]
#[clap(version = "0.1.4", author = "hzqd <hzqelf@yeah.net>")]
pub struct Args {
    /// Specify the dictionary name, no dictionary by default
    #[clap(short, long, default_value = "")]
    pub dictionary: String,

    /// Specify the time unit, support nanos, micros, millis, secs
    #[clap(short, long, default_value = "millis")]
    pub time: TimeUnit,

    /// Set Mode, support batch and stream processing
    #[clap(subcommand)]
    pub subcmd: Mode,
}

#[derive(Parser)]
pub enum Mode {
    /// A subcommand for specify the batch processing
    #[allow(non_camel_case_types)]
    batch {
        /// Specify the input file name
        #[clap(short, long)]
        input: String,

        /// Specify the output file name
        #[clap(short, long)]
        output: String,

        /// With -e to select English or Chinese otherwise
        #[clap(short, long)]
        en_or_cn: bool,
    },
    /// Not implemented — Will be implemented in the next version ; A subcommand for specify the stream processing
    #[allow(non_camel_case_types)]
    stream {
        /// Ip address and port, e.g. 127.0.0.1:8080
        #[clap(short, long)]
        ip: String,

        /// With -e to select English or Chinese otherwise
        #[clap(short, long)]
        en_or_cn: bool,
    }
}

pub fn get_args() -> Args {
    Args::parse()
}