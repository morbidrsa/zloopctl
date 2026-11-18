use std::io::Error;
use std::process::Command;

use crate::ZLoopCtrlContext;
use crate::write_to_zloop;
use crate::check_zloop_path;

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
    args.push_str(&format!(",capacity_mb={}", ctx.capacity));
    args.push_str(&format!(",zone_size_mb={}", ctx.zone_size));
    args.push_str(&format!(",zone_capacity_mb={}", ctx.zone_capacity));
    args.push_str(&format!(",conv_zones={}", ctx.nr_conv));
    args.push_str(&format!(",base_dir={}", ctx.base_dir));
    args.push_str(&format!(",nr_queues={}", ctx.nr_queues));
    args.push_str(&format!(",queue_depth={}", ctx.queue_depth));

    if ctx.buffered {
        args.push_str(",buffered");
    }

    if ctx.debug {
        println!("{args}");
    }

    write_to_zloop(ctx, args)
}

