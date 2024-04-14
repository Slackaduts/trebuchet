// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(dead_code)]
#![forbid(unsafe_code)]

use reqwest::get;

const SLIDESHOW_URL: &str = "https://www.wizard101.com/WizardPatchClient?abs=1";
const GAME_URL: &str = "https://www.wizard101.com";

/// Extracts the slideshow URL from the given input.
/// 
/// The input is split by comma, and the 0th element is returned with apostrophes removed.
fn extract_slideshow_url(input: &str) -> String {
    input.split(",").collect::<Vec<&str>>()[0].replace("'", "")
}

/// Extracts the clickable link URL from the given input
/// 
/// The input is split by quotation mark, and the 2nd element is returned.
/// If the output starts with "/", it is appended to the GAME_URL.
fn extract_click_url(input: &str) -> String {
    let output = input.split("\"").collect::<Vec<&str>>()[1].to_string();

    if output.starts_with("/") {
        return GAME_URL.to_string() + &output;
    }

    output
}

/// Retrieves the slideshow image/href links from the Wizard101 website.
/// 
/// Returns a tuple containing two vectors: image URLs and link URLs.
#[tauri::command]
async fn get_slideshow_images() -> (Vec<String>, Vec<String>) {
    let response = get(SLIDESHOW_URL)
        .await
        .expect("Failed to read the slideshow page content.");
    let html = response
        .text()
        .await
        .expect("Failed to read the slideshow page content.");

    let lines: Vec<&str> = html.split("\n").collect();

    let mut within_card_i: i8 = 0;
    let mut image_urls: Vec<String> = Vec::new();
    let mut link_urls: Vec<String> = Vec::new();

    for line in lines {
        match within_card_i {
            0 => {}
            1 => {
                image_urls.push(extract_slideshow_url(&line));
            }
            2 => {
                link_urls.push(extract_click_url(&line));
            }
            _ => {
                within_card_i = 0;
            }
        }

        if 0 != within_card_i {
            within_card_i += 1;
        }
        if line.contains("myCarousel.addCard(") & !line.contains("//") {
            within_card_i += 1;
        }
    }

    return (image_urls, link_urls);
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_slideshow_images])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
