//! local build/src/main.rs

use cmd_lib::*;

fn main() -> CmdResult {
    let db_init_msg = "Initializing database for local development...";
    run_cmd!(echo $db_init_msg)?;
    run_cmd!(./cr-api/scripts/init_db.sh)?;
    let redis_init_msg = "Initializing Redis container for local development...";
    run_cmd!(echo $redis_init_msg)?;
    run_cmd!(./cr-api/scripts/init_redis.sh)?;
    Ok(())
}
