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

#[derive(Debug)]
pub struct ZloopCtrlContext {
    pub id: i32,
    pub debug: bool,
    pub command: ZLoopCtlCommand,
}

pub static CONTROL_PATH: &'static str = "/dev/zloop-control";

pub fn list(ctx: &ZloopCtrlContext) {
    let dev = Path::new("/dev/");

    for entry in read_dir(dev).unwrap() {
        let entry = entry.unwrap();
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

        let stat = fs::stat(&path).unwrap();
        let mode = fs::FileType::from_raw_mode(stat.st_mode);

        if !mode.is_block_device() {
            if ctx.debug {
                println!("found a zloop device that is not a block device '/dev/{}'", basename);
            }
            continue;
        }

        println!("{}", basename);

    }
}

pub fn add(_ctx: &ZloopCtrlContext) {
    println!("add")
}

pub fn del(ctx: &ZloopCtrlContext) -> Result<(), Error>{
    let path = Path::new(CONTROL_PATH);
    let args: String = String::from(format!("remove id={}", ctx.id));

    let mut ctrl = File::options().write(true).open(path)?;

    if ctx.debug {
        println!("args: {}", args);
    }

    ctrl.write(args.as_bytes())?;
    Ok(())
}


pub fn check_zloop_driver(
    ctx: &ZloopCtrlContext
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
