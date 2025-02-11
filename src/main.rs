use std::process::ExitCode;

use std::io;

use rs_find_long_lines::stdin2stdout;
use rs_find_long_lines::MIN_LEN_INCLUSIVE_DEFAULT;

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(io::Error::other)
}

fn min_len_inclusive() -> usize {
    env_val_by_key("ENV_MIN_LEN_INCLUSIVE")
        .ok()
        .and_then(|s| str::parse(s.as_str()).ok())
        .unwrap_or(MIN_LEN_INCLUSIVE_DEFAULT)
}

fn sub() -> Result<(), io::Error> {
    stdin2stdout(min_len_inclusive())
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
