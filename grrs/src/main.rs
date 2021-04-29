use anyhow::{Context, Result};
use structopt::StructOpt;
use human_panic::setup_panic;

fn main() -> Result<()> {
    setup_panic!();

    let args = grrs::Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}
