pub mod cli;
pub mod dotr;

use std::path::PathBuf;

fn main() {
    if let Some(args) = cli::parse_args() {
        match args.command {
            cli::Command::Add => {
                let src = PathBuf::from(args.args[0].clone());
                let dest = PathBuf::from(args.args[1].clone());
                let is_symbolic = args.args[2] == "true";

                dotr::add(src.as_path(), dest.as_path(), is_symbolic);
            }
        }
    }
}
