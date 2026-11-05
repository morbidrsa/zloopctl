use rustix::fs;
use std::io::Error;
use std::fs::{read_dir, DirEntry};
use std::path::{Path, PathBuf};

use crate::ZLoopCtrlContext;

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
