//! shuttle-build/src/main.rs

use cmd_lib::*;

fn main() -> CmdResult {
    let init_msg = "Initializing database...";
    run_cmd!(echo $init_msg)?;
    run_cmd!(./cr-api/scripts/init_db.sh)?;
    let test_msg = "Running tests...";
    run_cmd!(echo $test_msg)?;
    run_cmd!(cargo test)?;
    Ok(())
}
