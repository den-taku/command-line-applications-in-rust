use structopt::StructOpt;
use anyhow::{Context, Result};
use std::io::{self, Write};
use log::{info, warn};

fn main() -> Result<()> {
    // let pb = indicatif::ProgressBar::new(100);
    // let mut i = 0;
    // for i in 0..100 {
        // do_hard_work();
        // pb.println(format!("[+] finished #{}", i));
        // pb.inc(1);
    // }
    // pb.finish_with_message("done");
    // env_logger::init();
    // info!("starting up");
    // warn!("oops, nothing implemented!");
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;
    for line in content.lines() {
        if line.contains(&args.pattern) {
            writeln!(handle, "{}", line)?;
        }
    }
    Ok(())
}

/// Search for a patter in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

// fn do_hard_work() -> i32 {
//     let mut i = 0;
//     for j in 0..1000000 {
//         i += 1;
//     }
//     i
// }