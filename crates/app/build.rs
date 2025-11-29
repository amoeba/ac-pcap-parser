use std::{env, process::Command};

fn main() {
    // Get git SHA at build time
    let git_sha = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    println!("cargo:rustc-env=GIT_SHA={git_sha}");

    // BOT_BASE_URL
    let base_url =
        env::var("BOT_BASE_URL").unwrap_or_else(|_| "https://bot.treestats.net".to_string());
    println!("cargo:rustc-env=BOT_BASE_URL={base_url}");

    println!("cargo:rerun-if-changed=.git/HEAD");
}
