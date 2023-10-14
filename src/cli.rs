use clap;

pub struct AddCommand {
    pub src : String,
    pub dest: String,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub is_symlink: bool 
}
pub enum RemoteCommandTypes {SetUrl, Push}
pub struct RemoteCommand {
    pub type_ : RemoteCommandTypes,
    pub url: Option<String>
}

pub enum Command {Add(AddCommand), Init, Remote(RemoteCommand)}


pub fn parse_args() -> Option<Command>{
    let matched = clap::Command::new("dotr")
        .about("simple dotfile manager")
        .author("rheasan")
        .arg_required_else_help(true)
        .subcommand_required(true)

        // init
        .subcommand(
            clap::Command::new("init")
            .about(
                "setup dotr directory.\nif used twice it will delete data stored by dotr\n(files added with symlinks will be deleted)"
            )
        )

        // add
        .subcommand(
            clap::Command::new("add")
            .about("adds the file at <src> to dotr file list and copies it to <dest>")
            .arg(
                clap::Arg::new("src")
                .help("Dotfile source")
                .action(clap::ArgAction::Set)
                .num_args(1)
                .required(true)
            )

            .arg(
                clap::Arg::new("dest")
                .help("Dotfile dest")
                .action(clap::ArgAction::Set)
                .num_args(1)
                .required(true)
            )

            .arg(
                clap::Arg::new("symbolic")
                .long("symlink")
                .short('s')
                .help("copies the file at <src> to dotr directory and makes a symlink at <dest>")
                .action(clap::ArgAction::SetTrue)
            )

            .arg(
                clap::Arg::new("name")
                .help("Name for dotfile. (only used for dotr)")
                .long("name")
                .short('n')
                .action(clap::ArgAction::Set)
                .num_args(1)
                .required(false)
            )

            .arg(
                clap::Arg::new("desc")
                .help("Short description for dotfile. (only used for dotr)")
                .long("desc")
                .short('d')
                .action(clap::ArgAction::Set)
                .num_args(1)
                .required(false)
            )
        )

        // remote
        .subcommand(
            clap::Command::new("remote")
            .about("modify remote for dotfiles")
            .subcommand_required(true)
            .subcommand(
                clap::Command::new("set-url")
                .arg(
                    clap::Arg::new("remote-url")
                    .help("url of remote repository")
                    .num_args(1)
                    .action(clap::ArgAction::Set)
                    .required(true)
                )
                .about("Set the url for remote repository")
            )
            .subcommand(
                clap::Command::new("push")
                .about("Push all dotfiles to remote repository")
            )
        )
        .get_matches();

    match matched.subcommand() {
        Some(("add", add_matches)) => {
            if !add_matches.args_present() {
                return None;
            }

            let src = add_matches.get_one::<String>("src").unwrap();
            let dest = add_matches.get_one::<String>("dest").unwrap();
            let desc = add_matches.get_one::<String>("desc");
            let name = add_matches.get_one::<String>("name");
            let is_symlink = add_matches.get_flag("symbolic");

            return Some(
                Command::Add(
                    AddCommand {
                        src: src.to_owned(),
                        dest: dest.to_owned(),
                        name: name.cloned(),
                        desc: desc.cloned(),
                        is_symlink,
                    }
                )
            )
        },
        Some(("init", init_matches)) => {
            if init_matches.args_present() {
                return None;
            }

            return Some(Command::Init);
        },
        Some(("remote", remote_matches)) => {
            match remote_matches.subcommand() {
                Some(("set-url", set_url_matches)) => {
                    let remote_url = set_url_matches.get_one::<String>("remote-url").unwrap();
                    return Some(
                        Command::Remote(
                            RemoteCommand { type_: RemoteCommandTypes::SetUrl, url: Some(remote_url.to_owned()) }
                        )
                    )
                },
                Some(("push", _)) => {
                    return Some(
                        Command::Remote(
                            RemoteCommand { type_: RemoteCommandTypes::Push, url: None }
                        )
                    )
                },
                Some((&_, _)) => {
                    unreachable!()
                }
                None => {
                    unreachable!()
                }
            }
        }
        _ => unreachable!()
    }
}