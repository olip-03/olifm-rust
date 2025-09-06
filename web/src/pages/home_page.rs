use crate::console_log;
use crate::log;
use crate::page::Page as PageType;
use github::GithubClientCallback;
use std::collections::HashMap;
pub fn home_page() -> PageType {
    let mut params = HashMap::new();
    params.insert("title".to_string(), "Welcome to oli.fm".to_string());

    let render = |p: &PageType| {
        let title = p
            .params
            .get("title")
            .map(|s| s.as_str())
            .unwrap_or("Untitled");
        format!(
            "<h1>{}</h1><div id=\"github-content\">Loading GitHub content...</div>",
            title
        )
    };

    let on_after_render = || {
        console_log!("Page rendered, fetching GitHub content...");

        let client = GithubClientCallback::new();

        client.get_repo_content("olip-03", "oli-fm-content", "", |result| {
            match result {
                Ok(content) => {
                    console_log!("Fetched {} items from GitHub repository", content.len());

                    // Process the repository content and update DOM
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(element) = document.get_element_by_id("github-content") {
                                let mut html = String::new();
                                html.push_str("<h2>Repository Content:</h2>");
                                html.push_str("<ul>");

                                for item in content {
                                    html.push_str(&format!(
                                        "<li><strong>{}</strong> - {} ({})</li>",
                                        item.name, item.content_type, item.size
                                    ));
                                }

                                html.push_str("</ul>");
                                element.set_inner_html(&html);
                            }
                        }
                    }
                }
                Err(err) => {
                    console_log!("Error fetching repo content: {}", err);

                    // Update DOM with error message
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(element) = document.get_element_by_id("github-content") {
                                element.set_inner_html(&format!(
                                    "<p style=\"color: red;\">Error loading content: {}</p>",
                                    err
                                ));
                            }
                        }
                    }
                }
            }
        });
    };

    PageType::new("Home", params, render).with_on_after_render(Some(Box::new(on_after_render)))
}
