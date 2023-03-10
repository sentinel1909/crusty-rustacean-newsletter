//! local build/src/main.rs

use cmd_lib::*;

fn main() -> CmdResult {
    let init_msg = "Initializing database for local development...";
    run_cmd!(echo $init_msg)?;
    run_cmd!(./cr-api-local/scripts/init_db.sh)?;
    Ok(())
}
