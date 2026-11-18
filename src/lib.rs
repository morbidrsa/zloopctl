use rustix::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::Error;
use clap::{ArgAction, Parser, Subcommand};

mod del;
mod list;
mod add;

pub use del::del;
pub use list::list;
pub use add::add;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ZLoopCtlCommand {
    LIST,
    ADD,
    DEL,
}

impl Clone for ZLoopCtlCommand {
    fn clone(&self) -> Self {
        match self {
            ZLoopCtlCommand::LIST => ZLoopCtlCommand::LIST,
            ZLoopCtlCommand::ADD => ZLoopCtlCommand::ADD,
            ZLoopCtlCommand::DEL => ZLoopCtlCommand::DEL,
        }
    }
}

#[derive(Parser, Debug)]
#[command(name = "zloopctl", version = "0.1", about = "Control zloop devices")]
pub struct Args {
    /// Enable debug output
    #[arg(short, long, action = ArgAction::SetTrue)]
    debug: bool,

    /// Sub-command (`list`, `add`, `del`)
    #[command(subcommand)]
    command: ZLoopCommand,
}

#[derive(Subcommand, Debug)]
pub enum ZLoopCommand {
    /// list zloop devices
    List,

    ///  add zloop device
    Add(AddArgs),

    /// delete zloop device
    Del(DelArgs),
}

#[derive(Parser, Debug)]
pub struct AddArgs {
    #[arg(short = 'i', long, help = "The ID of the zloop device to add (auto-assigned if not provided")]
    id: Option<i32>,

    #[arg(short = 'c', long, help = "The size of the zloop device in MB", default_value = "16384")]
    capacity: i32,

    #[arg(short = 's', long = "zone-size", help = "the zone size of the zloop device in MB", default_value = "256")]
    zone_size: i32,

    #[arg(short = 'C', long = "zone-capacity", help = "the zone capacity of the zloop device in MB", default_value = "256")]
    zone_capacity: i32,

    #[arg(short = 'n', long = "nr-conv-zones", help = "the number of conventional zones of the zloop device", default_value = "0")]
    nr_conv: i32,

    #[arg(short = 'b', long = "base-dir", help = "The base directory for the zloop device", default_value = "/var/local/zloop")]
    base_dir: String,

    #[arg(long = "create", help = "create the device directory if it doesn't exist")]
    create_dir: bool,

    #[arg(short = 'q', long = "nr-queues", help = "the number of I/O queues of the zloop device", default_value = "0")]
    nr_queues: i32,

    #[arg(short = 'Q', long = "queue-depth", help = "the depth of the zloop device's I/O queue", default_value = "128")]
    queue_depth: i32,

    #[arg(short = 'B', long = "buffered-io", help = "use buffered I/O for the zloop device")]
    buffered: bool,
}

#[derive(Parser, Debug)]
pub struct DelArgs {
    #[arg(short = 'i', long, help = "The ID of the zloop device to delete")]
    id: i32,
}

#[derive(Debug)]
pub struct ZLoopCtrlContext {
    pub id: i32,
    pub debug: bool,
    pub command: ZLoopCtlCommand,
    pub capacity: i32,
    pub zone_size: i32,
    pub zone_capacity: i32,
    pub nr_conv: i32,
    pub base_dir: String,
    pub nr_queues: i32,
    pub queue_depth: i32,
    pub buffered: bool,
    pub create_dir: bool,
}

impl ZLoopCtrlContext {

    pub fn from_args() -> Result<Self, &'static str> {
        let args = Args::parse();

        let mut ctx = ZLoopCtrlContext {
            debug: args.debug,
            command: ZLoopCtlCommand::LIST,
            id: 0,
            capacity: 0,
            zone_size: 0,
            zone_capacity: 0,
            nr_conv: 0,
            base_dir: "".into(),
            nr_queues: 0,
            queue_depth: 0,
            buffered: false,
            create_dir: false,
        };

        match args.command  {
            ZLoopCommand::List => ctx.command = ZLoopCtlCommand::LIST,

            ZLoopCommand::Add(add) => {
                ctx.command = ZLoopCtlCommand::ADD;
                ctx.id = add.id.unwrap_or_else(|| find_next_free_id(&ctx));
                ctx.capacity = add.capacity;
                ctx.zone_size = add.zone_size;
                ctx.zone_capacity = add.zone_capacity;
                ctx.nr_conv = add.nr_conv;
                ctx.base_dir = add.base_dir;
                ctx.nr_queues = add.nr_queues;
                ctx.buffered = add.buffered;
                ctx.create_dir = add.create_dir;
            },

            ZLoopCommand::Del(del) => {
                ctx.command = ZLoopCtlCommand::DEL;
                ctx.id = del.id;
            },
        }

        if ctx.debug {
            println!("{:?}", ctx)
        }

        Ok(ctx)

    }
}

fn find_next_free_id(ctx: &ZLoopCtrlContext) -> i32 {
    // Start from 0 and look for the first unused ID
    let mut id = 0;
    loop {
         let test_ctx = ZLoopCtrlContext {
             id,
             debug: ctx.debug,
             command: ctx.command.clone(),
             capacity: ctx.capacity,
             zone_size: ctx.zone_size,
             zone_capacity: ctx.zone_capacity,
             nr_conv: ctx.nr_conv,
             base_dir: ctx.base_dir.clone(),
             nr_queues: ctx.nr_queues,
             queue_depth: ctx.queue_depth,
             buffered: ctx.buffered,
             create_dir: ctx.create_dir
         };

        if !check_zloop_path(&test_ctx) {
            return id;
        }
        id += 1;
    }
}


static CONTROL_PATH: &str = "/dev/zloop-control";

pub fn check_zloop_path(ctx: &ZLoopCtrlContext) -> bool
{
    let p = format!("{0}/{1}", ctx.base_dir, ctx.id);
    let path = Path::new(&p);

    path.exists()
}

pub fn check_zloop_driver(
    ctx: &ZLoopCtrlContext
) -> Result<bool, &'static str> {
    if ctx.debug {
        println!("Checking {}", CONTROL_PATH);
    }

    let stat = fs::stat(CONTROL_PATH).expect("stat failed");

    if ctx.debug {
        println!("stat.st_mode: 0x{:08x}", stat.st_mode);
    }

    let mode = fs::FileType::from_raw_mode(stat.st_mode);

    if !mode.is_char_device() {
        return Err("not a character device")
    }

    Ok(mode.is_char_device())
}

pub fn write_to_zloop(ctx: &ZLoopCtrlContext, args: String) -> Result<(), Error>
{
    let path = Path::new(CONTROL_PATH);
    let mut ctrl = File::options().write(true).open(path)?;

    if ctx.debug {
        println!("args: {}", args);
    }

    let _ = ctrl.write(args.as_bytes())?;

    Ok(())
}
