use clap;

pub enum Command {Add}
pub struct DotCommand {
    pub command: Command,
    pub args: Vec<String>
}


pub fn parse_args() -> Option<DotCommand>{
    let matched = clap::Command::new("dotr")
        .about("simple dotfile manager")
        .author("rheasan")
        .arg_required_else_help(true)
        .subcommand_required(true)

        .subcommand(
            clap::Command::new("add")
            .about("adds the file at <src> to dotr file list and copies it to <dest>")
            .arg(
                clap::Arg::new("src")
                .help("Dotfile source")
                .action(clap::ArgAction::Set)
                .num_args(1)
                .action(clap::ArgAction::Set)
            )
            .arg_required_else_help(true)

            .arg(
                clap::Arg::new("dest")
                .help("Dotfile dest")
                .action(clap::ArgAction::Set)
                .num_args(1)
                .action(clap::ArgAction::Set)
            )
            .arg_required_else_help(true)
        )
        
        .get_matches();

    match matched.subcommand() {
        Some(("add", add_matches)) => {
            if !add_matches.args_present() {
                return None;
            }

            let src = add_matches.get_one::<String>("src").unwrap();
            let dest = add_matches.get_one::<String>("dest").unwrap();

            return Some(DotCommand { 
                command : Command::Add,
                // TODO: tidy up this weird thing
                args: vec![src.to_owned(), dest.to_owned()]
            });
        }
        _ => unreachable!()
    }
}