use std::process;

use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use zloopctl::*;

fn main() {
    let ctx = parse_options();

    check_zloop_driver(&ctx).unwrap_or_else(|err| {
        eprintln!("zloop driver not found {err}");
        process::exit(1);
    });

    match ctx.command {
        ZLoopCtlCommand::ADD => {
            match add(&ctx) {
                Ok(_) => {},
                Err(e) => eprintln!("{}", e)
            }
        },
        ZLoopCtlCommand::LIST => list(&ctx),
        ZLoopCtlCommand::DEL => {
            match del(&ctx) {
                Ok(_) => {},
                Err(e) => eprintln!("{}", e)
            }
        }
    }
}

fn parse_options() -> ZLoopCtrlContext {
    let mut ctx: ZLoopCtrlContext = ZLoopCtrlContext {
        id: 0,
        debug: false,
        command: ZLoopCtlCommand::LIST,
        capacity: zloopctl::DEFAULT_CAPACITY,
        zone_size: zloopctl::DEFAULT_ZONE_SIZE,
        zone_capacity: zloopctl::DEFAULT_ZONE_SIZE,
        nr_conv: 0,
        base_dir: zloopctl::DEFAULT_BASE_DIR.to_string(),
        nr_queues: 1,
        queue_depth: zloopctl::DEFAULT_QUEUE_DEPTH,
        buffered: false,
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
                .arg(
                    Arg::new("ID")
                        .short('i')
                        .long("id")
                        .help("The ID of the zloop device to add")
                        .action(ArgAction::Set)
                        .required(true)
                        .value_parser(value_parser!(i32))
                )
                .arg(
                    Arg::new("capacity")
                        .short('c')
                        .long("capacity")
                        .help("the size of the zloop device in MB")
                        .required(false)
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(i32))
                )
                .arg(
                    Arg::new("zone size")
                        .short('s')
                        .long("zone-size")
                        .help("the zone size of the zloop device in MB")
                        .required(false)
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(i32))
                )
                .arg(
                    Arg::new("zone capacity")
                        .short('C')
                        .long("zone-capacity")
                        .help("the zone capacity of the zloop device in MB")
                        .required(false)
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(i32))
                )
                .arg(
                    Arg::new("nr conv zones")
                        .short('n')
                        .long("nr-conv-zones")
                        .help("the number of conventional zones of the zloop device")
                        .required(false)
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(i32))
                )
                .arg(
                    Arg::new("base dir")
                        .short('b')
                        .long("base-dir")
                        .help("the base directory for the zloop device")
                        .required(false)
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(String))
                )
                .arg(
                    Arg::new("nr queues")
                        .short('q')
                        .long("nr-queues")
                        .help("the number of I/O queues of the zloop device")
                        .required(false)
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(i32))
                )
                .arg(
                    Arg::new("queue depth")
                        .short('Q')
                        .long("queue-depth")
                        .help("the depth of the zloop device's I/O queue")
                        .required(false)
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(i32))
                )
                .arg(
                    Arg::new("Buffered I/O")
                        .short('B')
                        .long("buffered-io")
                        .help("use buffered I/O for the zloop device")
                        .action(ArgAction::Set)
                        .value_parser(value_parser!(i32))
                )
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
    } else if let Some(cmd) = matches.subcommand_matches("add") {
        ctx.command = ZLoopCtlCommand::ADD;
        parse_add_options(&mut ctx, cmd);
    } else if let Some(cmd) = matches.subcommand_matches("del") {
        ctx.command = ZLoopCtlCommand::DEL;
        ctx.id = *cmd.get_one::<i32>("ID").expect("ID not found");
    }

    if ctx.debug {
        println!("{:?}", ctx);
    }

    ctx
}

fn parse_add_options(ctx: &mut ZLoopCtrlContext, cmd: &ArgMatches)
{

    ctx.id = *cmd.get_one::<i32>("ID").expect("ID not found");

    if cmd.contains_id("capacity") {
        ctx.capacity = *cmd.get_one::<i32>("capacity")
            .expect("invalid capacity");
    }

    if cmd.contains_id("zone size") {
        ctx.zone_size = *cmd.get_one::<i32>("zone size")
            .expect("invalid zone size");
    }

    if cmd.contains_id("zone capacity") {
        ctx.zone_capacity = *cmd.get_one::<i32>("zone capacity")
            .expect("invalid zone capacity");
    }

    if cmd.contains_id("nr conv zones") {
        ctx.nr_conv = *cmd.get_one::<i32>("nr conv zones")
            .expect("invalid number of conventional zones");
    }

    if cmd.contains_id("base dir") {
        ctx.base_dir = cmd.get_one::<String>("base dir")
            .expect("invalid base directory")
            .to_string();
    }

    if cmd.contains_id("nr queues") {
        ctx.nr_queues = *cmd.get_one::<i32>("nr queues")
            .expect("invalid number of I/O queues");
    }

    if cmd.contains_id("queue depth") {
        ctx.queue_depth = *cmd.get_one::<i32>("queue depth")
            .expect("invalid I/O queue depth");
    }

    if cmd.contains_id("Buffered I/O") {
        ctx.buffered = cmd.get_flag("Buffered I/O");
    }

}
