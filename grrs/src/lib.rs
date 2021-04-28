use anyhow::Result;
use structopt::StructOpt;

/// Search for a patter in a file and display the lines that contain it.
#[derive(StructOpt)]
pub struct Cli {
    /// The pattern to look for
    pub pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,
}

pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) -> Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}

#[test]
fn find_a_match() -> Result<()> {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
    assert_eq!(result, b"lorem ipsum\n");
    Ok(())
}
