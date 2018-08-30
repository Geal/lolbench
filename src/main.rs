#[macro_use]
extern crate failure;
extern crate log;
#[macro_use]
extern crate structopt;

extern crate chrono;
extern crate clap;
extern crate lolbench_support;
extern crate simple_logger;

use std::path::PathBuf;

use chrono::{NaiveDate, Utc};
use structopt::StructOpt;

use lolbench_support::*;

fn main() -> Result<()> {
    simple_logger::init().unwrap();
    Cli::from_args().exec()
}

#[derive(Debug, StructOpt)]
struct Measure {
    // TODO(anp): structopt mangles this help message horribly
    /// Selects specific CPUs on which *only* benchmarks will run. Currently only supported when run
    /// as root on Linux. Accepts a pattern of CPU IDs and ID ranges delimited by commas
    /// (e.g. 0,1,2 or 0-2,4). Defaults to performing no CPU isolation.
    #[structopt(short = "c", long = "cpus")]
    cpu_pattern: Option<String>,

    /// If a CPU pattern is set, also ask the kernel to try to relocate kernel tasks off of
    /// benchmark CPUs.
    #[structopt(short = "k", long = "move-kthreads")]
    move_kernel_threads: bool,

    /// Limit the benchmarks run to those assigned to the given runner.
    #[structopt(long = "runner")]
    runner: Option<String>,

    /// Run benchmarks with a single toolchain.
    #[structopt(long = "single-toolchain")]
    single_toolchain: Option<String>,

    /// Run benchmarks with nightlies starting from a specific.
    #[structopt(long = "nightlies-since")]
    nightlies_since: Option<NaiveDate>,

    /// Path to data directory. Will be created if empty.
    #[structopt(long = "data-dir", parse(from_os_str))]
    data_dir: PathBuf,
}

impl Measure {
    fn run(self) -> Result<()> {
        let toolchains = match self {
            Self {
                single_toolchain: Some(toolchain),
                nightlies_since: None,
                ..
            } => ToolchainSpec::Single(toolchain.clone()),

            Self {
                single_toolchain: None,
                nightlies_since: Some(start),
                ..
            } => ToolchainSpec::Range(start.clone(), Utc::today().naive_utc()),

            _ => bail!("unsupported toolchain configuration"),
        };

        let kthread_on = self.move_kernel_threads;

        let shield_spec = self.cpu_pattern.as_ref().map(move |cpus| ShieldSpec {
            cpu_mask: cpus.to_string(),
            kthread_on,
        });

        let opts = BenchOpts {
            toolchains,
            runner: self.runner.clone(),
            shield_spec,
        };

        let collector = Collector::rehydrate(&self.data_dir)?;

        for (toolchain, benchmarks) in plan_benchmarks(opts)? {
            toolchain.install()?;

            for benchmark in benchmarks {
                collector.run(benchmark)?;
            }
        }

        Ok(())
    }
}

/// Run benchmarks to assess the performance of code generated by Rust toolchains.
#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
enum SubCommand {
    #[structopt(name = "measure")]
    Measure {
        #[structopt(flatten)]
        inner: Measure,
    },
}

impl Cli {
    pub fn exec(self) -> Result<()> {
        match self.cmd {
            SubCommand::Measure { inner } => inner.run(),
        }
    }
}
