use clap::{Arg, ArgAction, Command};
use zloopctl::*;

fn main() {
    let ctx = parse_options();

    if !check_zloop_driver(&ctx) {
        println!("{} not found, is the module loaded?", CONTROL_PATH);
        return
    }

    match ctx.command {
        ZLoopCtlCommand::ADD => add(&ctx),
        ZLoopCtlCommand::LIST => list(&ctx),
        ZLoopCtlCommand::DEL => del(&ctx),
    }
}

fn parse_options() -> ZloopCtrlContext {
    let mut ctx: ZloopCtrlContext = ZloopCtrlContext {
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
        )
        .get_matches();

    ctx.debug = matches.get_flag("debug");

    if let Some(_cmd) = matches.subcommand_matches("list") {
        ctx.command = ZLoopCtlCommand::LIST;
    } else if let Some(_cmd) = matches.subcommand_matches("add") {
        ctx.command = ZLoopCtlCommand::ADD;
    } else if let Some(_cmd) = matches.subcommand_matches("del") {
        ctx.command = ZLoopCtlCommand::DEL;
    }

    if ctx.debug {
        println!("{:?}", ctx);
    }

    ctx
}
