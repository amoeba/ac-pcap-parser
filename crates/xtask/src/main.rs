use std::process::Command;
use std::fs;
use std::path::Path;
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
        Some("bot") => {
            // Check for --serve flag
            let serve = std::env::args().any(|arg| arg == "--serve");
            bot(serve)
        }
        Some(task) => bail!("Unknown task: {}", task),
        None => {
            eprintln!("Available tasks:");
            eprintln!("  cargo xtask bot         - Build WASM and bot");
            eprintln!("  cargo xtask bot --serve - Build WASM, bot, and run server");
            Ok(())
        }
    }
}

fn bot(serve: bool) -> Result<()> {
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

    // Create dist directory if it doesn't exist
    fs::create_dir_all("dist").context("Failed to create dist directory")?;

    // Calculate content hash for WASM file for cache busting
    let wasm_path = "crates/web/pkg/web_bg.wasm";
    let wasm_content = fs::read(wasm_path).context("Failed to read WASM file")?;
    let hash = format!("{:x}", md5::compute(&wasm_content));
    let short_hash = &hash[..16]; // Use first 16 chars of hash

    // Cache-busted filenames
    let js_filename = format!("web.{}.js", short_hash);
    let wasm_filename = format!("web_bg.{}.wasm", short_hash);

    println!("  ✓ Cache bust hash: {}", short_hash);

    // Copy and update JS file to reference cache-busted WASM filename
    let js_content = fs::read_to_string("crates/web/pkg/web.js")
        .context("Failed to read JS file")?;
    let updated_js = js_content.replace("web_bg.wasm", &wasm_filename);
    fs::write(format!("dist/{}", js_filename), updated_js)
        .context("Failed to write updated JS file")?;
    println!("  ✓ Copied and updated web.js -> {} (references {})", js_filename, wasm_filename);

    fs::copy(wasm_path, format!("dist/{}", wasm_filename))
        .context("Failed to copy WASM file")?;
    println!("  ✓ Copied web_bg.wasm -> {}", wasm_filename);

    // Copy other files from pkg to dist
    for entry in fs::read_dir("crates/web/pkg").context("Failed to read pkg dir")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Skip files we already handled or don't want
        if file_name_str == "web.js"
            || file_name_str == "web_bg.wasm"
            || file_name_str == ".gitignore"
            || file_name_str.ends_with(".d.ts") {
            continue;
        }

        let dest = Path::new("dist").join(&file_name);
        fs::copy(entry.path(), &dest)?;
        println!("  ✓ Copied {}", file_name_str);
    }

    // Copy static files from static/ directory and update index.html with cache-busted JS filename
    for entry in fs::read_dir("static").context("Failed to read static dir")? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        let dest = Path::new("dist").join(&file_name);

        if file_name_str == "index.html" {
            // Read, update, and write index.html with cache-busted JS filename
            let content = fs::read_to_string(entry.path())
                .context("Failed to read index.html")?;
            let updated_content = content.replace("./web.js", &format!("./{}", js_filename));
            fs::write(&dest, updated_content)
                .context("Failed to write updated index.html")?;
            println!("  ✓ Copied and updated index.html (using {})", js_filename);
        } else {
            fs::copy(entry.path(), &dest)?;
            println!("  ✓ Copied {}", file_name_str);
        }
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

    if serve {
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
    } else {
        println!("✅ Build complete! Run with --serve to start the server.");
    }

    Ok(())
}
