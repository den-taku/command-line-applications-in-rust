use anyhow::{Context, Result};
use structopt::StructOpt;

fn main() -> Result<()> {
    let args = grrs::Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{:?}`", &args.path))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}
