use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

/// The type that captures our CLI API.
#[derive(Debug, StructOpt)]
struct Cli {
    // The input file we will process
    input_file: PathBuf,
}

/// The errors we handle
#[derive(thiserror::Error, Debug)]
enum Error {
    /// io error when reading the input file
    #[error("could not read the specified input file: {0}")]
    InputFile(#[from] std::io::Error),

    /// An error while trying to deserialize [lib::Message]
    #[error("{0}")]
    Message(#[from] lib::InvalidMessageError),
}

fn main() -> Result<(), Error> {
    let Cli { input_file } = Cli::from_args();
    let file_contents = fs::read_to_string(input_file)?;

    let result = lib::group_messages(file_contents);
    match result {
        Ok(groups) => {
            for (type_, stats) in groups {
                println!(
                    "{}: {} times, {} bytes",
                    type_, stats.occurences, stats.total_byte_size
                );
            }
            Ok(())
        }
        Err(error) => {
            println!("Oops! An error occured: {}", error);
            Err(error.into())
        }
    }
}
