//! shuttle-build/src/main.rs

use cmd_lib::*;

fn main() -> CmdResult {
    let msg = "Hello, World!";
    run_cmd!(echo $msg)?;

    Ok(())
}
