// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![warn(dead_code)]
#![forbid(unsafe_code)]

use std::{str::{Chars, FromStr}, sync::Arc};


const PRE_NEWS_PAT: &str = r#"<td class="contentbox_headermiddle"><h2><center>"#;
const POST_NEWS_CUTOFF: &str = r#"&raquo;"#;

fn extract_string_between(input: &str, pre: &str, post: &str) -> Option<(String, usize, usize)> {
    let input_str = input.to_string();
    let pre_index = input_str.find(pre)? + pre.len();

    let post_index = input_str[pre_index..].find(post)? + pre_index;

    Some((input[pre_index..post_index].to_string(), pre_index, post_index))
}

fn extract_string_between_vec(input: &str, pre: &str, post: Vec<&str>) -> Option<String> {
    let mut nearest_match: (String, usize) = (String::new(), 0);
    for search in post {
        match extract_string_between(input, pre, search) {
            Some(a) => {
                let (_, x) = nearest_match;
                let (b, _, c) = a;

                if c > x { nearest_match = (b, c) }
            },
            None => ()
        }
    }

    let (matched_str, match_i) = nearest_match;
    match match_i { 0 => return None, _ => () }

    return Some(matched_str)
}

/// Recursively strips HTML tags from a given str.
/// 
fn strip_html(input: &str) -> String {
    return String::from_str(s)
};

/// 
/// 
fn strip_html_arc(input: &Chars) -> Chars {
    let mut output = String::new();
    let mut to_rem = false;

    // reverse string once, search from opposite end once open bracket is found

    for i in input.into_iter() {
    }

    return output;
}


// #[tauri::command]
// fn parse_wizard_news(response: String) -> Option<String> {
//     // let news_links: Vec<(String, String)> = Vec::new();

//     for (i, elem) in response.split(PRE_NEWS_PAT).enumerate() {
//         match i { 0 => continue , _ => ()}

//         // Only push everything before "&raquo" to the split_response vector
//         let cut_elem: &str = elem.split(POST_NEWS_CUTOFF).collect::<Vec<&str>>()[0];

//         let date = extract_string_between_vec(cut_elem, "", ["</center></h2>"].to_vec()).expect("Date not found");
//         let news_content = extract_string_between_vec(cut_elem, "<br>", [r#"</p>"#, r#"<a href="#].to_vec()).expect("Content not found");
//         let title_w_link = extract_string_between_vec(cut_elem, r#"<p style="font-size: 13px; color: #000;">"#, ["</b>"].to_vec()).expect("Could not find title");
//         // println!("{}", title_w_link);
//         let image = extract_string_between_vec(cut_elem, r#"<img src=""#, [r#"""#].to_vec()).expect("Could not find image/link block.");
//         let link = match extract_string_between_vec(cut_elem, r#"<a href=""#, [r#"""#].to_vec()) {
//             Some(a) => a,
//             None => String::new()
//         };
//         let title = match extract_string_between_vec(&title_w_link, "target=_blank>", ["</a>"].to_vec()) {
//             Some(a) => a,
//             None => {
//                 title_w_link.split("<b>").collect::<Vec<&str>>()[1].to_owned()
//             }
//         };
//         println!("Date: {}", date);
//         println!("Title: {}", title);
//         println!("Image: {}", image);
//         println!("Link: {}", link);
//         println!("Content: {}", news_content);
//         println!("-------------------------------------------------------------------------------------------------------------");
//     }

//     return Some(String::new());
// }


#[tauri::command]
fn parse_pirate_news(response: String) -> Option<String> {
    for (i, elem) in response.split(r#"<div class="boxheadermiddle"><h2>"#).enumerate() {
        match i { 0 => continue , 1 => continue, _ => ()}

        // Only push everything before "&raquo" to the split_response vector
        let cut_elem: &str = elem.split(POST_NEWS_CUTOFF).collect::<Vec<&str>>().get(0)?;

        // Date is the beginning, so no front slicing is needed
        let date: String = extract_string_between_vec(cut_elem, "", vec!["<"])?;
        
        // Actual news content is nested inbetween a br tag and a /p or a href tag
        let news_content: String = extract_string_between_vec(cut_elem, "<br>", vec![r#"</p>"#, r#"<a href="#])?;
        
        // Link/title formatting is inconsistent and will need further manipulation
        let title_w_link: String = extract_string_between_vec(cut_elem, r#"<p><b>"#, vec!["</b>"])?;

        let image: String = extract_string_between_vec(cut_elem, r#"<img src=""#, vec![r#"""#])?;

        let link: String = match extract_string_between_vec(cut_elem, r#"<a href=""#, vec![r#"""#]) {
            Some(a) => { //Link will either be full URL or something like "/game/news", this code handles that.
                let b = match str::starts_with(&a, '/') { true => format!("https://www.pirate101.com{}", &a), _ => a };
                b
            },
            None => String::new()
        };

        //TODO: Investigate if this actually does anything, too far in to remember exact parsing steps for title
        // let raw_title = match extract_string_between_vec(&title_w_link, "target=_blank>", vec!["</a>"]) {
        //     Some(a) => a,
        //     None => title_w_link
        // };

        
        let title = match extract_string_between_vec(&title_w_link, r#"">"#, vec!["</a", "</b"]) { 
            Some(a) => a,
            None => title_w_link
        };
        println!("Date: {}", date);
        println!("Title: {}", title);
        println!("Image: {}", image);
        println!("Link: {}", link);
        println!("Content: {}", news_content);
        println!("-------------------------------------------------------------------------------------------------------------");
    }

    return Some(String::new());
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![parse_pirate_news])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
