mod addon;
mod config;

use clap::Parser;
use std::fs;

#[derive(Parser)]
#[command(name = "openesoui-mm")]
#[command(
    about = "Open ESOUI Mod Manager - A terminal based mod manager for Elder Scrolls Online, using the ESOUI mod repository"
)]
struct Cli {
    /// Download a single file ID instead of using config
    #[arg(long, value_name = "FILE_ID")]
    download: Option<u16>,

    /// Target directory for extraction (overrides config)
    #[arg(long, short, value_name = "DIR")]
    target: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("OpenEsoUI-MM - Starting...");

    let cli = Cli::parse();

    // Configuration file mode
    let config =  config::Config::load_from_file()?;
    let target_dir = cli.target.unwrap_or(config.target_directory);
    
    let addon_ids: Vec<u16>;
    
    // Print configuration info
    println!("  ESO addon directory: {}", target_dir);
    if let Some(single_addon_id) = cli.download {
        println!("Addon IDs: {}", single_addon_id);
        addon_ids = vec!(single_addon_id);
    } else {
        println!("Addon IDs: {:?}", config.addon_ids);
        addon_ids = config.addon_ids;
    }

    // Create target directory if it doesn't exist
    fs::create_dir_all(&target_dir)?;

    // Download and extract each file
    for addon_id in &addon_ids {
        let mut addon = addon::Addon::new(addon_id.clone());
        addon.parse_infopage().await;
        addon.parse_downloadpage().await;
        addon.download_and_extract(&target_dir).await?;
        println!();
    }

    println!("\nCompleted!");
    Ok(())
}



