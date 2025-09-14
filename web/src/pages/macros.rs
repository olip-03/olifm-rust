use crate::console_log;
use crate::content::{get_global_content, get_global_document, get_global_tags};
use crate::log;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use content_service::ContentServiceError;
use content_service::JsonEntry;
use std::cmp::Ordering;
pub enum Style {
    Card,
    Photo,
    Music,
}

#[macro_export]
macro_rules! render_site {
    ($path:expr, $style:expr) => {{
        let content_path = $path.to_string();
        let style = $style;

        wasm_bindgen_futures::spawn_local(async move {
            let base = get_base_url!().to_string();
            let doc_url = format!("{}/content/{}/readme.md", base, content_path);
            match crate::pages::macros::get_page_content(&content_path, &doc_url).await {
                Ok((mut repo_content, document, tags)) => {
                    let mut html = String::new();

                    load_readme(&mut repo_content, &mut html, &document);

                    console_log!("{:?}", tags);

                    html.push_str("<div class=\"tag-container\">");
                    html.push_str("<div class=\"tags\">");
                    for tag in tags {
                        html.push_str(&format!("<span class=\"tag\">{}</span>", tag));
                    }
                    html.push_str("</div>");
                    html.push_str("</div>");

                    if content_path != "" {
                        let div_class = format!("{}-container", &content_path);
                        html.push_str(&format!("<div class=\"{}\">", div_class));
                        for item in repo_content {
                            match style {
                                // TODO: Custom music card implementation
                                Style::Card | Style::Music => html
                                    .push_str(&crate::pages::page_home::page_home_card_html(item)),
                                Style::Photo => html.push_str(
                                    &crate::pages::page_pictures::page_pictures_card_html(item),
                                ),
                            }
                        }
                        html.push_str("</div>");
                    }

                    let parser = Parser::new(&html);
                    let mut html_output = String::new();
                    html::push_html(&mut html_output, parser);

                    get_app!().set_inner_html(&html_output);
                    setup_article_observer();
                }
                Err(e) => {
                    crate::console_log!("Error fetching directory data: {:?}", e);
                    get_app!().set_inner_html(&format!(
                        "<p style=\"color: red;\">Error loading content: {}</p>",
                        e
                    ));
                }
            }
        });
    }};
}

pub async fn get_page_content(
    _path: &str,
    doc_url: &str,
) -> Result<(Vec<JsonEntry>, String, Vec<String>), ContentServiceError> {
    let path = format!("/{}", _path);
    let mut items = get_global_content(path.clone(), Some("file".to_string())).await?;

    sort_entries_by_date(&mut items, true);

    let tags = get_global_tags(path.clone()).await?;
    let document = get_global_document(doc_url).await?;
    Ok((items, document, tags))
}

pub fn load_readme(content: &mut Vec<JsonEntry>, html: &mut String, document: &String) {
    html.push_str("<div class=\"page-title\">");
    if let Some(index) = content
        .iter()
        .position(|item| item.name.to_lowercase() == "readme.md")
    {
        content.remove(index);
        html.push_str(&document);
    } else {
        console_log!("No readme found");
        html.push_str(&document);
    }
    html.push_str("</div>");
}

fn parse_date_to_utc(s: &str) -> Option<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    if let Ok(ndt) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Some(DateTime::<Utc>::from_utc(ndt, Utc));
    }

    if let Ok(nd) = NaiveDate::parse_from_str(s, "%Y-%m-%d") {
        return Some(DateTime::<Utc>::from_utc(nd.and_hms(0, 0, 0), Utc));
    }
    let fmts = ["%Y/%m/%d", "%d-%m-%Y", "%m/%d/%Y", "%B %d, %Y", "%b %d, %Y"];

    for fmt in &fmts {
        if let Ok(ndt) = NaiveDateTime::parse_from_str(s, fmt) {
            return Some(DateTime::<Utc>::from_utc(ndt, Utc));
        }
        if let Ok(nd) = NaiveDate::parse_from_str(s, fmt) {
            return Some(DateTime::<Utc>::from_utc(nd.and_hms(0, 0, 0), Utc));
        }
    }

    None
}

pub fn sort_entries_by_date(entries: &mut [JsonEntry], newest_first: bool) {
    entries.sort_by(|a, b| {
        let a_date = a.metadata.get("date").and_then(|s| parse_date_to_utc(s));
        let b_date = b.metadata.get("date").and_then(|s| parse_date_to_utc(s));

        match (a_date, b_date) {
            (Some(ad), Some(bd)) => {
                if newest_first {
                    bd.cmp(&ad)
                } else {
                    ad.cmp(&bd)
                }
            }
            (Some(_), None) => Ordering::Less,
            (None, Some(_)) => Ordering::Greater,
            (None, None) => a.name.cmp(&b.name),
        }
    });
}
