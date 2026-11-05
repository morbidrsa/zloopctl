use std::io::Error;

use crate::ZLoopCtrlContext;
use crate::write_to_zloop;

pub fn del(ctx: &ZLoopCtrlContext) -> Result<(), Error>{
    let args: String = format!("remove id={}", ctx.id);

    if ctx.debug {
        println!("args: {}", args);
    }

    write_to_zloop(ctx, args)
}


