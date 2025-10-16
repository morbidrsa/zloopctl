use std::process;

use clap::{Arg, ArgAction, Command, value_parser};
use zloopctl::*;

fn main() {
    let ctx = parse_options();

    check_zloop_driver(&ctx).unwrap_or_else(|err| {
        eprintln!("zloop driver not found {err}");
        process::exit(1);
    });

    match ctx.command {
        ZLoopCtlCommand::ADD => add(&ctx),
        ZLoopCtlCommand::LIST => list(&ctx),
        ZLoopCtlCommand::DEL => {
            match del(&ctx) {
                Ok(_) => {},
                Err(e) => eprintln!("{}", e)
            }
        }
    }
}

fn parse_options() -> ZloopCtrlContext {
    let mut ctx: ZloopCtrlContext = ZloopCtrlContext {
        id: 0,
        debug: false,
        command: ZLoopCtlCommand::LIST
    };

    let matches = Command::new("zloopctl")
        .version("0.1")
        .about("Control zloop devices")
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Enable debug output")
                .action(ArgAction::SetTrue)
        )
        .subcommand(
            Command::new("list")
                .about("list zloop devices")
        )
        .subcommand(
            Command::new("add")
                .about("add zloop device")
        )
        .subcommand(
            Command::new("del")
                .about("delete zloop device")
                .arg(
                    Arg::new("ID")
                        .short('i')
                        .long("id")
                        .help("The ID of the zloop device to delete")
                        .action(ArgAction::Set)
                        .required(true)
                        .value_parser(value_parser!(i32))
                )
        )
        .get_matches();

    ctx.debug = matches.get_flag("debug");

    if let Some(_cmd) = matches.subcommand_matches("list") {
        ctx.command = ZLoopCtlCommand::LIST;
    } else if let Some(_cmd) = matches.subcommand_matches("add") {
        ctx.command = ZLoopCtlCommand::ADD;
    } else if let Some(cmd) = matches.subcommand_matches("del") {
        ctx.command = ZLoopCtlCommand::DEL;
        ctx.id = *cmd.get_one::<i32>("ID").expect("ID not found");
    }

    if ctx.debug {
        println!("{:?}", ctx);
    }

    ctx
}
