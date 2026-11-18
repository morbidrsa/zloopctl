use std::process;

use zloopctl::*;

fn main() {
    let ctx = ZLoopCtrlContext::from_args().unwrap();

    check_zloop_driver(&ctx).unwrap_or_else(|err| {
        eprintln!("zloop driver not found {err}");
        process::exit(1);
    });

    match ctx.command {
        ZLoopCtlCommand::ADD => {
            match add(&ctx) {
                Ok(_) => {},
                Err(e) => eprintln!("{}", e)
            }
        },
        ZLoopCtlCommand::LIST => {
            match list(&ctx) {
                Ok(_) => {},
                Err(e) => eprintln!("{}", e)
            }
        }
        ZLoopCtlCommand::DEL => {
            match del(&ctx) {
                Ok(_) => {},
                Err(e) => eprintln!("{}", e)
            }
        }
    }
}

