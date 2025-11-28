use std::process::Command;
use anyhow::{Result, Context, bail};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let task = std::env::args().nth(1);
    match task.as_deref() {
        Some("bot") => bot(),
        Some(task) => bail!("Unknown task: {}", task),
        None => {
            eprintln!("Available tasks:");
            eprintln!("  cargo xtask bot    - Build WASM and run bot server");
            Ok(())
        }
    }
}

fn bot() -> Result<()> {
    println!("🔨 Building WASM UI...");

    // Build WASM with wasm-pack
    let status = Command::new("wasm-pack")
        .args(&["build", "crates/web", "--target", "web", "--release"])
        .status()
        .context("Failed to run wasm-pack")?;

    if !status.success() {
        bail!("wasm-pack build failed");
    }

    println!("✅ WASM build complete");
    println!("📦 Copying WASM assets to dist/...");

    // Copy WASM output to dist/
    let status = Command::new("cp")
        .args(&["-r", "crates/web/pkg/.", "dist/"])
        .status()
        .context("Failed to copy WASM assets")?;

    if !status.success() {
        bail!("Failed to copy WASM assets");
    }

    println!("✅ Assets copied");
    println!("🔧 Building bot...");

    // Build bot
    let status = Command::new("cargo")
        .args(&["build", "--release", "-p", "bot"])
        .status()
        .context("Failed to build bot")?;

    if !status.success() {
        bail!("Bot build failed");
    }

    println!("✅ Bot build complete");
    println!("🚀 Starting bot server...");
    println!("");

    // Run bot
    let status = Command::new("cargo")
        .args(&["run", "--release", "-p", "bot"])
        .status()
        .context("Failed to run bot")?;

    if !status.success() {
        bail!("Bot failed to run");
    }

    Ok(())
}
