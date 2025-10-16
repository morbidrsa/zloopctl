use rustix::fs;
use std::fs::{read_dir, File};
use std::io::Write;
use std::path::Path;
use std::io::Error;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ZLoopCtlCommand {
    LIST,
    ADD,
    DEL,
}

// Defaults
pub static DEFAULT_CAPACITY: i32 = 16384;
pub static DEFAULT_ZONE_SIZE: i32 = 256;
pub static DEFAULT_QUEUE_DEPTH: i32 = 128;
pub static DEFAULT_BASE_DIR: &'static str = "/var/local/zloop";

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

static CONTROL_PATH: &'static str = "/dev/zloop-control";

pub fn list(ctx: &ZLoopCtrlContext) -> Result<(), Error>{
    let dev = Path::new("/dev/");

    for entry in read_dir(dev)? {
        let entry = entry?;
        let path = entry.path();
        let basename = path
            .strip_prefix("/dev/")
            .unwrap()
            .to_str()
            .unwrap();

        if !basename.starts_with("zloop") {
            if ctx.debug {
                println!("skipping '/dev/{}'", basename);
            }
            continue;
        }

        if basename == "zloop-control" {
            continue;
        }

        let stat = fs::stat(&path)?;
        let mode = fs::FileType::from_raw_mode(stat.st_mode);

        if !mode.is_block_device() {
            if ctx.debug {
                println!("found a zloop device that is not a block device '/dev/{}'", basename);
            }
            continue;
        }

        println!("{}", basename);

    }

    Ok(())
}

pub fn add(ctx: &ZLoopCtrlContext) -> Result<(), Error>{
    let mut args: String = String::new();

    args.push_str(&format!("add id={}", ctx.id));
    args.push_str(&format!(",capacity_mb={}", ctx.capacity));
    args.push_str(&format!(",zone_size_mb={}", ctx.zone_size));
    args.push_str(&format!(",zone_capacity_mb={}", ctx.zone_capacity));
    args.push_str(&format!(",conv_zones={}", ctx.nr_conv));
    args.push_str(&format!(",base_dir={}", ctx.base_dir));
    args.push_str(&format!(",nr_queues={}", ctx.nr_queues));
    args.push_str(&format!(",queue_depth={}", ctx.queue_depth));
    if ctx.buffered {
        args.push_str(&format!(",buffered"));
    }

    if ctx.debug {
        println!("{args}");
    }

    write_to_zloop(ctx, args)
}

pub fn del(ctx: &ZLoopCtrlContext) -> Result<(), Error>{
    let args: String = String::from(format!("remove id={}", ctx.id));

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

    ctrl.write(args.as_bytes())?;

    Ok(())
}
