use rustix::fs;
use std::fs::{read_dir, File, DirEntry};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::io::Error;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ZLoopCtlCommand {
    LIST,
    ADD,
    DEL,
}

// Defaults
static DEFAULT_CAPACITY: i32 = 16384;
static DEFAULT_ZONE_SIZE: i32 = 256;
static DEFAULT_QUEUE_DEPTH: i32 = 128;
static DEFAULT_NR_QUEUES: i32 = 0;
static DEFAULT_NR_CONV: i32 = 0;
static DEFAULT_BASE_DIR: &str = "/var/local/zloop";

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
    pub buffered: bool
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
        }
    }
}

static CONTROL_PATH: &str = "/dev/zloop-control";

fn check_zloop_path(ctx: &ZLoopCtrlContext) -> bool
{
    let p = format!("{0}/{1}", ctx.base_dir, ctx.id);
    let path = Path::new(&p);

    path.exists()
}

fn basename(path: &PathBuf) -> Result<String, Error>{
    let str = path.strip_prefix("/dev/").unwrap().to_str().unwrap();
    Ok(String::from(str))
}

fn entry_is_zloop(ctx: &ZLoopCtrlContext, entry: &DirEntry) -> Result<bool, Error> {
    let path = entry.path();
    let basename = basename(&path)?;

    if !basename.starts_with("zloop") {
        if ctx.debug {
            println!("skipping '/dev/{}'", basename);
        }
        return Ok(false)
    }

    if basename == "zloop-control" {
        return Ok(false)
    }

    let stat = fs::stat(&path)?;
    let mode = fs::FileType::from_raw_mode(stat.st_mode);

    if !mode.is_block_device() {
        if ctx.debug {
            println!("found a zloop device that is not a block device '/dev/{}'", basename);
        }
        return Ok(false)
    }

    Ok(true)
}

fn collect_devs(ctx: &ZLoopCtrlContext) -> Result<Vec<String>, Error> {

    let mut devs = Vec::new();

    for entry in read_dir(Path::new("/dev/"))? {
        let entry = entry?;

        if !entry_is_zloop(ctx, &entry)? {
            continue;
        }

        let name = basename(&entry.path())?;
        devs.push(name);
    }

    Ok(devs)
}

pub fn list(ctx: &ZLoopCtrlContext) -> Result<(), Error>{
    let devs = collect_devs(ctx)?;

    devs.iter().for_each(|d| println!("{}", d));

    Ok(())
}

pub fn add(ctx: &ZLoopCtrlContext) -> Result<(), Error>{
    let mut args: String = String::new();

    if !check_zloop_path(ctx) {
        return Err(Error::new(std::io::ErrorKind::NotFound,
                              format!("zloop path {}/{} does not exist",
                              ctx.base_dir, ctx.id)))
    }

    args.push_str(&format!("add id={}", ctx.id));

    if ctx.capacity != DEFAULT_CAPACITY {
        args.push_str(&format!(",capacity_mb={}", ctx.capacity));
    }

    if ctx.zone_size != DEFAULT_ZONE_SIZE {
        args.push_str(&format!(",zone_size_mb={}", ctx.zone_size));
    }

    if ctx.zone_capacity != DEFAULT_ZONE_SIZE {
        args.push_str(&format!(",zone_capacity_mb={}", ctx.zone_capacity));
    }

    if ctx.nr_conv != DEFAULT_NR_CONV {
        args.push_str(&format!(",conv_zones={}", ctx.nr_conv));
    }

    if ctx.base_dir != DEFAULT_BASE_DIR {
        args.push_str(&format!(",base_dir={}", ctx.base_dir));
    }

    if ctx.nr_queues != DEFAULT_NR_QUEUES {
        args.push_str(&format!(",nr_queues={}", ctx.nr_queues));
    }

    if ctx.queue_depth != DEFAULT_QUEUE_DEPTH {
        args.push_str(&format!(",queue_depth={}", ctx.queue_depth));
    }

    if ctx.buffered {
        args.push_str(",buffered");
    }

    if ctx.debug {
        println!("{args}");
    }

    write_to_zloop(ctx, args)
}

pub fn del(ctx: &ZLoopCtrlContext) -> Result<(), Error>{
    let args: String = format!("remove id={}", ctx.id);

    if ctx.debug {
        println!("args: {}", args);
    }

    write_to_zloop(ctx, args)
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

fn write_to_zloop(ctx: &ZLoopCtrlContext, args: String) -> Result<(), Error>
{
    let path = Path::new(CONTROL_PATH);
    let mut ctrl = File::options().write(true).open(path)?;

    if ctx.debug {
        println!("args: {}", args);
    }

    let _ = ctrl.write(args.as_bytes())?;

    Ok(())
}
