use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("videodir")
        .about("video registrator storage backend")
        //.subcommand_required(true)
        //.arg_required_else_help(true)
        .subcommand(
            Command::new("list")
                .about("list users in the htpasswd")
        )
        .subcommand(
            Command::new("add")
                .about("add or update user in the htpasswd")
                .arg(arg!(<username>).required(true))
                .arg(arg!(<password>).required(true))
        )
        .subcommand(
            Command::new("remove")
                .about("remove user from htpasswd")
                .arg(arg!(<username>).required(true))
        )
}