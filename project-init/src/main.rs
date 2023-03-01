//! shuttle-build/src/main.rs

use cmd_lib::*;

fn main() -> CmdResult {
    let msg = "This is the project init crate...";
    run_cmd!(echo $msg)?;

    Ok(())
}
