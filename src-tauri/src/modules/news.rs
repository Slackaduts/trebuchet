use crate::modules::errors::GUIResult;
use anyhow::Context;

const PRE_NEWS_PAT: &str = r#"<td class="contentbox_headermiddle"><h2><center>"#;
const POST_NEWS_CUTOFF: &str = r#"&raquo;"#;
const IMG_PAT: &str = r#"<img src=""#;
const LINK_PAT: &str = r#"<a href=""#;
const END_QUOTE_PAT: &str = r#"""#;
const PIRATE101_SITE: &str = "https://pirate101.com";
const WIZARD101_SITE: &str = "https://wizard101.com";

/// Extracts a string between two substrings.
///
fn extract_string_between(input: &str, pre: &str, post: &str) -> Option<(String, usize, usize)> {
    let input_str = input.to_string();
    let pre_index = input_str.find(pre)? + pre.len();

    let post_index = input_str[pre_index..].find(post)? + pre_index;

    Some((
        input[pre_index..post_index].to_string(),
        pre_index,
        post_index,
    ))
}

/// Extracts a string between two substrings, but the latter can be any of a vector of strings.
///
fn extract_string_between_vec(input: &str, pre: &str, post: Vec<&str>) -> Option<String> {
    let mut nearest_match: (String, usize) = (String::new(), 0);
    for search in post {
        match extract_string_between(input, pre, search) {
            Some(a) => {
                let (_, x) = nearest_match;
                let (b, _, c) = a;

                if c > x {
                    nearest_match = (b, c)
                }
            }
            None => (),
        }
    }

    let (matched_str, match_i) = nearest_match;
    match match_i {
        0 => return None,
        _ => (),
    }

    return Some(matched_str);
}

/// Strips top-level HTML tags.
///
fn strip_html(input: &str) -> String {
    let mut result = String::new();
    let mut will_rem = false;

    for ch in input.chars().peekable() {
        match ch {
            '<' => will_rem = true,
            '>' => will_rem = false,
            _ => {
                if !will_rem {
                    result.push(ch)
                }
            }
        }
    }

    return result;
}

/// Cuts off everything after a certain substring. Used to cut off the post-news section.
///
fn cut_pre_response(input: &str) -> Option<&str> {
    return Some(input.split(POST_NEWS_CUTOFF).nth(0)?);
}

fn extract_link(input: &str, site: &str) -> String {
    return match extract_string_between_vec(input, LINK_PAT, vec![END_QUOTE_PAT]) {
        Some(a) => {
            //Link will either be full URL or something like "/game/news", this code handles that.
            let b = match str::starts_with(&a, '/') {
                true => format!("{}{}", site, &a),
                _ => a,
            };
            b
        }
        None => site.to_string(),
    };
}

fn extract_news(input: &str) -> Option<String> {
    let mut content = extract_string_between_vec(input, "<br>", vec![r#"</p>"#, LINK_PAT])?;
    content = match &content.strip_prefix("\r\n") {
        Some(a) => a.to_string(),
        None => content,
    };

    content = match &content.strip_suffix(" ") {
        Some(a) => a.to_string(),
        None => content,
    };

    return Some(content);
}

#[derive(serde::Serialize)]
pub struct NewsEntry {
    date: String,
    title: String,
    image: String,
    link: String,
    content: String,
}

#[tauri::command]
pub fn parse_wizard_news(response: String) -> GUIResult<Vec<NewsEntry>> {
    let mut news_entires: Vec<NewsEntry> = Vec::new();

    for (i, elem) in response.split(PRE_NEWS_PAT).enumerate() {
        match i {
            0 => continue,
            _ => (),
        }

        let cut_elem: &str =
            cut_pre_response(elem).context("Could not cut content from element part.")?;

        let date = extract_string_between_vec(cut_elem, "", vec!["</center></h2>"])
            .context("Could not extract date.")?;

        let news_content = extract_news(cut_elem).context("Could not extract news content.")?;

        let title_w_link = extract_string_between_vec(
            cut_elem,
            r#"<p style="font-size: 13px; color: #000;">"#,
            vec!["</b>"],
        )
        .context("Could not extract title or link.")?;

        let image = extract_string_between_vec(cut_elem, IMG_PAT, vec![END_QUOTE_PAT])
            .context("Could not extract image URL.")?;
        let link = extract_link(cut_elem, WIZARD101_SITE);

        let title = match extract_string_between_vec(&title_w_link, "target=_blank>", vec!["</a>"])
        {
            Some(a) => a,
            None => {
                let a = title_w_link
                    .split("<b>")
                    .nth(1)
                    .unwrap_or(&title_w_link)
                    .to_string();
                a
            }
        };

        news_entires.push(NewsEntry {
            date: strip_html(&date),
            title: strip_html(&title),
            image: image,
            link: link,
            content: strip_html(&news_content),
        });
    }

    return Ok(news_entires);
}

#[tauri::command]
pub fn parse_pirate_news(response: String) -> GUIResult<Vec<NewsEntry>> {
    let mut news_entires: Vec<NewsEntry> = Vec::new();

    for (i, elem) in response
        .split(r#"<div class="boxheadermiddle"><h2>"#)
        .enumerate()
    {
        match i {
            0 => continue,
            1 => continue,
            _ => (),
        }

        let cut_elem: &str =
            cut_pre_response(elem).context("Could not cut content from element part.")?;

        let date: String = extract_string_between_vec(cut_elem, "", vec!["<"])
            .context("Could not extract date.")?;

        let news_content = extract_news(cut_elem).context("Could not extract news content.")?;

        let title_w_link: String = extract_string_between_vec(cut_elem, r#"<p><b>"#, vec!["</b>"])
            .context("Could not extract title or link.")?;

        let image: String = extract_string_between_vec(cut_elem, IMG_PAT, vec![END_QUOTE_PAT])
            .context("Could not extract image URL.")?;

        let link: String = extract_link(cut_elem, PIRATE101_SITE);

        let title = extract_string_between_vec(&title_w_link, r#"">"#, vec!["</a", "</b"])
            .unwrap_or(title_w_link);

        news_entires.push(NewsEntry {
            date: strip_html(&date),
            title: strip_html(&title),
            image: image,
            link: link,
            content: strip_html(&news_content),
        });
    }

    return Ok(news_entires);
}
