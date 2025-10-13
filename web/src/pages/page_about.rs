use crate::console_log;
use crate::content::{get_global_content, get_global_document};
use crate::get_app;
use crate::get_base_url;
use crate::log;
use crate::page::Page as PageType;
use crate::pages::macros::{Style, get_page_tags, load_readme};

use crate::render_site;
use crate::setup_article_observer;
use pulldown_cmark::{Parser, html};
use std::collections::HashMap;

pub fn page_about() -> PageType {
    let params = HashMap::new();
    let render = |_: &PageType| "".to_string();

    let on_after_render = || {
        // render_site!("resume", Style::Card);
        render_readme();
        // render_employment_history();
    };

    PageType::new("About", params, render).with_on_after_render(Some(Box::new(on_after_render)))
}

fn render_readme() {
    wasm_bindgen_futures::spawn_local(async {
        let mut html = String::new();
        let base = get_base_url!().to_string();
        let doc_url = format!("{}/content/resume/readme.md", base).to_string();
        match get_global_document(&doc_url).await {
            Ok(document) => {
                let parser = Parser::new(&document);
                let mut html_output = String::new();
                html.push_str("<div class=\"page-title\">");
                html::push_html(&mut html_output, parser);
                html.push_str(&html_output);
                html.push_str("</div>");
            }
            Err(e) => html.push_str(&format!(
                "<p style=\"color: red;\">Error loading content: {}</p>",
                e
            )),
        }

        let items = get_global_content("/resume".to_string(), Some("file".to_string())).await;

        get_app!().set_inner_html(&html);
    });
}

fn render_employment_history() {
    wasm_bindgen_futures::spawn_local(async {});
}
