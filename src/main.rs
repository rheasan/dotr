pub mod cli;
pub mod dotr;

use std::path::PathBuf;

fn main() {
    if let Some(args) = cli::parse_args() {
        match args {
            cli::Command::Add(args) => {
                let src = PathBuf::from(args.src);
                let dest = PathBuf::from(args.dest);

                dotr::add(&src, &dest, args.is_symlink);
            }
        }
    }
}
