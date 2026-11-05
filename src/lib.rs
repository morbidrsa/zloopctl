use rustix::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::Error;

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

// Defaults
pub static DEFAULT_CAPACITY: i32 = 16384;
pub static DEFAULT_ZONE_SIZE: i32 = 256;
pub static DEFAULT_QUEUE_DEPTH: i32 = 128;
pub static DEFAULT_NR_QUEUES: i32 = 0;
pub static DEFAULT_NR_CONV: i32 = 0;
pub static DEFAULT_BASE_DIR: &str = "/var/local/zloop";

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
    pub create_dir: bool
}

impl ZLoopCtrlContext {
    pub fn new() -> ZLoopCtrlContext {
        ZLoopCtrlContext {
            id: 0,
            debug: false,
            command: ZLoopCtlCommand::LIST,
            capacity: DEFAULT_CAPACITY,
            zone_size: DEFAULT_ZONE_SIZE,
            zone_capacity: DEFAULT_ZONE_SIZE,
            nr_conv: DEFAULT_NR_CONV,
            base_dir: DEFAULT_BASE_DIR.to_string(),
            nr_queues: DEFAULT_NR_QUEUES,
            queue_depth: DEFAULT_QUEUE_DEPTH,
            buffered: false,
            create_dir: false,
        }
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
