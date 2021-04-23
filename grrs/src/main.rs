use structopt::StructOpt;

fn main() {
    let args = Cli::from_args();
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
