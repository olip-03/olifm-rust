use crate::console_log;
use crate::get_app;
use crate::log;
use crate::page::Page as PageType;
use github::GithubClientCallback;
use std::collections::HashMap;

pub fn page_pictures() -> PageType {
    let mut params = HashMap::new();
    params.insert("title".to_string(), "Pictures".to_string());

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
        let client = GithubClientCallback::new();
        client.get_repo_content(
            "olip-03",
            "oli-fm-content",
            "pictures",
            |result| match result {
                Ok(content) => {
                    let mut html = String::new();
                    html.push_str("<h2>Repository Content:</h2>");
                    html.push_str("<ul>");

                    for item in content {
                        html.push_str(&format!(
                            "<div><strong>{}</strong> - {} ({})</div>",
                            item.name, item.content_type, item.size
                        ));
                    }

                    html.push_str("</ul>");
                    get_app!().set_inner_html(&html);
                }
                Err(err) => {
                    get_app!().set_inner_html(&format!(
                        "<p style=\"color: red;\">Error loading content: {}</p>",
                        err
                    ));
                }
            },
        );
    };

    PageType::new("Home", params, render).with_on_after_render(Some(Box::new(on_after_render)))
}
