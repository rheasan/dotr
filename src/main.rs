pub mod cli;
pub mod dotr;

use std::path::PathBuf;

fn main() {
    if let Some(args) = cli::parse_args() {
        match args {
            cli::Command::Add(args) => {
                let src = PathBuf::from(args.src);
                let dest = PathBuf::from(args.dest);

                dotr::add(&src, &dest, args.is_symlink, args.name, args.desc);
            },
            cli::Command::Init => {
                dotr::init();
            },
            cli::Command::Remote(command) => {
                match command.type_ {
                    cli::RemoteCommandTypes::Push => {
                        dotr::remote_push();
                    },
                    cli::RemoteCommandTypes::SetUrl => {
                        let url = command.url.unwrap();
                        dotr::remote_set_url(url);
                    }
                }
            }
        }
    }
}
