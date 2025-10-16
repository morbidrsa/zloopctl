use rustix::fs;
use std::fs::read_dir;
use std::path::Path;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum ZLoopCtlCommand {
    LIST,
    ADD,
    DEL,
}

#[derive(Debug)]
pub struct ZloopCtrlContext {
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

pub fn del(_ctx: &ZloopCtrlContext) {
    println!("del")
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
