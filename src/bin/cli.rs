use clap::{value_parser, Arg, Command};

extern crate rust;
#[tokio::main]
async fn main() {
    let matches = Command::new("Rust")
        .about("Rust commands")
        .arg_required_else_help(true)
        .subcommand(
            Command::new("users")
                    .about("User Management")
                    .arg_required_else_help(true)
                    .subcommand(
                    Command::new("create")
                            .about("Create a new user")
                            .arg_required_else_help(true)
                            .arg(Arg::new("username").required(true))
                            .arg(Arg::new("password").required(true))
                            .arg(Arg::new("roles").required(true).num_args(1..).value_delimiter(','))
                    )
                    .subcommand(
                    Command::new("list")
                            .about("List existing users")
                    )
                    .subcommand(
                    Command::new("delete")
                            .about("Delete user by ID")
                            .arg_required_else_help(true)
                            .arg(Arg::new("id").required(true).value_parser(value_parser!(i32)))
                ) 
        )
        .subcommand(
            Command::new("digest-send")
                .about("Send a digest with latest data via email")
                .arg(Arg::new("email").required(true))
                .arg(Arg::new("hours_since").required(true).value_parser(value_parser!(i32)))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("users", sub_matches)) => match sub_matches.subcommand() {
            Some(("create", sub_matches)) => rust::commands::create_user(
                sub_matches.get_one::<String>("username").unwrap().to_owned(),
                sub_matches.get_one::<String>("password").unwrap().to_owned(),
                sub_matches.get_many::<String>("roles").unwrap().map(|v| v.to_owned()).collect(),
            ).await,
            Some(("list", _)) => rust::commands::list_users().await,
            Some(("delete", sub_matches)) => rust::commands::delete_user(
                sub_matches.get_one::<i32>("id").unwrap().to_owned(),
            ).await,
            _ => {},
        },
        Some(("digest-send", sub_matches)) => rust::commands::digest_send(
            sub_matches.get_one::<String>("email").unwrap().to_owned(),
            sub_matches.get_one::<i32>("hours_since").unwrap().to_owned(),
        ).await,
        _ => {},
    }

}