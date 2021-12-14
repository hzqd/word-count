pub use aoko::no_std::algebraic::sum::TimeUnit;
use clap::Parser;

/// Batch processing word-count, support English & Chinese.

#[derive(Parser)]
#[clap(version = "0.1.3", author = "hzqd <hzqelf@yeah.net>")]
pub struct Args {
    /// Specify the dictionary name, no dictionary by default
    #[clap(short, long, default_value = "")]
    pub dictionary: String,

    /// Specify the input file name
    #[clap(short, long)]
    pub input: String,

    /// Specify the output file name
    #[clap(short, long)]
    pub output: String,

    /// Specify the time unit, support nanos, micros, millis, secs
    #[clap(short, long, default_value = "millis")]
    pub time: TimeUnit,

    /// Set language, support English(en) and Chinese(cn)
    #[clap(subcommand)]
    pub subcmd: Lang,
}

#[derive(Parser)]
pub enum Lang {
    /// A subcommand for specify English
    EN,
    /// A subcommand for specify Chinese
    CN,
}

pub fn get_args() -> Args {
    Args::parse()
}