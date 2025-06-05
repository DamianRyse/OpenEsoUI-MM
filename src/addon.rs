use std::{fs, io};
use std::path::Path;
use regex::Regex;
use scraper::Html;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use zip::ZipArchive;

pub struct Addon {
    pub id: u16,
    pub name: String,
    pub dl_url: String,
    pub addon_version: String,
    pub game_version: String,
    pub updated: String,
}

impl Addon {
    pub fn new(addon_id: u16) -> Addon {
        let addon = Addon {
            id: addon_id,
            name: "".to_string(),
            dl_url: "".to_string(),
            addon_version: "".to_string(),
            game_version: "".to_string(),
            updated: "".to_string(),
        };

        addon
    }

    pub async fn parse_infopage(&mut self) {
        let page_url = format!("https://www.esoui.com/downloads/info{}", self.id);

        // Create HTTP client
        let client = reqwest::Client::new();

        // Request the page and receive the response
        let page_response = client
            .get(&page_url)
            .send()
            .await
            .expect(format!("Failed to fetch info page for addon ID {}.", self.id).as_str());

        // Make sure the response is 200 OK
        if !page_response.status().is_success() {
            return;
        }

        // Get the actual HTML content and convert it to a HTML document
        let page_html = page_response
            .text()
            .await
            .expect(format!("Failed to fetch info page for addon ID {}.", self.id).as_str());
        let document = Html::parse_document(&page_html);

        // Define Regex patterns for the info searches
        let regex_game_version =
            String::from("<div id=\"patch\"><abbr title=\".*\">(.+)</abbr></div>");
        let regex_addon_version = String::from("<div id=\"version\">Version: (.*)</div>");
        let regex_updated = String::from("<div id=\"safe\">Updated: (.*)</div>");
        let regex_addon_name = String::from("<title>(.+?) :.+</title>");

        // Find the game version
        let re = Regex::new(&regex_game_version).unwrap();
        if let Some(captures) = re.captures(&document.html()) {
            let game_version = captures.get(1).unwrap().as_str().to_string();
            self.game_version = game_version;
        }

        let re = Regex::new(&regex_addon_version).unwrap();
        if let Some(captures) = re.captures(&document.html()) {
            let addon_version = captures.get(1).unwrap().as_str().to_string();
            self.addon_version = addon_version;
        }

        let re = Regex::new(&regex_updated).unwrap();
        if let Some(captures) = re.captures(&document.html()) {
            let updated = captures.get(1).unwrap().as_str().to_string();
            self.updated = updated;
        }

        let re = Regex::new(&regex_addon_name).unwrap();
        if let Some(captures) = re.captures(&document.html()) {
            let addon_name = captures.get(1).unwrap().as_str().to_string();
            self.name = addon_name;
        }

        println!("## {} (ID: {})", self.name, self.id);
        println!("AddOn version: {}", self.addon_version);
    }

    pub async fn parse_downloadpage(&mut self) {
        let page_url = format!("https://www.esoui.com/downloads/download{}", self.id);

        // Create HTTP client
        let client = reqwest::Client::new();

        // Request the page and receive the response
        let page_response = client
            .get(&page_url)
            .send()
            .await
            .expect(format!("Failed to fetch download page for addon ID {}.", self.id).as_str());

        // Make sure the response is 200 OK
        if !page_response.status().is_success() {
            return;
        }

        // Get the actual HTML content and convert it to a HTML document
        let page_html = page_response
            .text()
            .await
            .expect(format!("Failed to fetch download page for addon ID {}.", self.id).as_str());
        let document = Html::parse_document(&page_html);

        // Define Regex patterns for the info searches
        let regex_download =
            String::from("Problems with the download\\? <a href=\"(.*)\">Click here</a>.");

        // Find the download link
        let re = Regex::new(&regex_download).unwrap();
        if let Some(captures) = re.captures(&document.html()) {
            let dl_link = captures.get(1).unwrap().as_str().to_string();
            self.dl_url = dl_link;
        }
    }

    pub async fn download_and_extract(&self, dest: &str) -> Result<(), Box<dyn std::error::Error>>
    {
        // Create HTTP client
        let client = reqwest::Client::new();

        if self.dl_url == "" {
            println!("  No download URL found.");
            return Ok(());
        }

        // download the file
        let download_response = client.get(&self.dl_url).send().await?;

        if !download_response.status().is_success() {
            return Err(format!(
                "HTTP error when downloading file: {}",
                download_response.status()
            )
                .into());
        }

        // Get filename from the download URL or Content-Disposition header
        let filename = get_filename(&download_response)
            .unwrap_or_else(|| format!("file{}.zip", self.id));

        println!("  - Downloading {}...", filename);

        // Download to temporary file
        let temp_file_path = std::env::temp_dir().join(&filename);
        let bytes = download_response.bytes().await?;

        // Write to temporary file
        let mut temp_file = File::create(&temp_file_path).await?;
        temp_file.write_all(&bytes).await?;
        temp_file.flush().await?;
        drop(temp_file); // Close the file

        // Extract the ZIP file
        extract_zip(&temp_file_path, dest)?;

        // Clean up temporary file
        fs::remove_file(&temp_file_path)?;

        Ok(())
    }
}


/// Method to receive the filename from the HTTP header data.
fn get_filename(response: &reqwest::Response) -> Option<String> {
    if let Some(content_disposition) = response.headers().get("content-disposition") {
        if let Ok(cd_str) = content_disposition.to_str() {
            // Parse Content-Disposition header to extract filename
            for part in cd_str.split(';') {
                let part = part.trim();
                if part.starts_with("filename=") {
                    let filename = part[9..].trim_matches('"');
                    return Some(filename.to_string());
                }
            }
        }
    }
    None
}

fn extract_zip<P: AsRef<Path>>(zip_path: P, target_dir: &str,) -> Result<(), Box<dyn std::error::Error>> {
    let zip_file = fs::File::open(&zip_path)?;
    let mut zip_archive = ZipArchive::new(&zip_file)?;

    println!("  - Extracting...");

    for i in 0..zip_archive.len() {
        let mut file = zip_archive.by_index(i)?;
        let file_path = file.mangled_name();

        if file_path.to_string_lossy().is_empty() {
            continue;
        }

        let output_path = Path::new(target_dir).join(&file_path);

        if file.is_dir() {
            // Create directory
            fs::create_dir_all(&output_path)?;
        } else {
            // Create parent directories if they don't exist
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }

            // Extract file
            let mut output_file = fs::File::create(&output_path).expect("  Failed to extract archive!");
            io::copy(&mut file, &mut output_file)?;
        }

        // Set file permissions on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&output_path, fs::Permissions::from_mode(mode)).expect(
                    &format!("Could not set permissions for file '{}'", &output_path.display()));
            }
        }
    }
    Ok(())
}