use std::io::Error;
use std::process::Command;

use crate::ZLoopCtrlContext;
use crate::write_to_zloop;
use crate::check_zloop_path;
use crate::DEFAULT_CAPACITY;
use crate::DEFAULT_ZONE_SIZE;
use crate::DEFAULT_QUEUE_DEPTH;
use crate::DEFAULT_NR_CONV;
use crate::DEFAULT_BASE_DIR;
use crate::DEFAULT_NR_QUEUES;

fn set_nocow_attribute(path: &str, ctx: &ZLoopCtrlContext) {
    let output = Command::new("chattr")
        .arg("+C")
        .arg(path)
        .output();

    match output {
        Ok(result) => {
            if !result.status.success() {
                if ctx.debug {
                    let error_msg = String::from_utf8_lossy(&result.stderr);
                    println!("Failed to set nocow attribute: {}", error_msg);
                }
            } else if ctx.debug {
                println!("Set nocow attribute on {}", path);
            }
        }
        Err(e) => {
            if ctx.debug {
                println!("chattr not available: {}", e);
            }
        }
    }
}

pub fn add(ctx: &ZLoopCtrlContext) -> Result<(), Error>{
    let mut args: String = String::new();

    if !check_zloop_path(ctx) {
        if ctx.create_dir {
            let path = format!("{}/{}", ctx.base_dir, ctx.id);
            std::fs::create_dir_all(&path)?;
            set_nocow_attribute(&path, ctx);
        } else {
            return Err(Error::new(std::io::ErrorKind::NotFound,
                                  format!("zloop path {}/{} does not exist",
                                  ctx.base_dir, ctx.id)))
        }
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

