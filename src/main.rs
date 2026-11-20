use std::process;
use std::io::Error;

use zloopctl::*;

fn main() -> Result<(), Error>  {
    let ctx = ZLoopCtrlContext::from_args();

    check_zloop_driver(&ctx).unwrap_or_else(|err| {
        eprintln!("zloop driver not found {err}");
        process::exit(1);
    });

    match ctx.command {
        ZLoopCtlCommand::ADD => add(&ctx)?,
        ZLoopCtlCommand::LIST => list(&ctx)?,
        ZLoopCtlCommand::DEL => del(&ctx)?,
    }

    Ok(())
}

