use crate::console_log;
use crate::get_app;
use crate::log;
use crate::page::Page as PageType;
use github::GithubClientCallback;
use std::collections::HashMap;
pub fn page_home() -> PageType {
    let mut params = HashMap::new();
    params.insert("title".to_string(), "Welcome to oli.fm".to_string());

    let render = |p: &PageType| {
        let title = p
            .params
            .get("title")
            .map(|s| s.as_str())
            .unwrap_or("Untitled");
        title.to_string()
    };

    let on_after_render = || {
        let client = GithubClientCallback::new();
        client.get_repo_content("olip-03", "oli-fm-content", "blog", |result| match result {
            Ok(mut content) => {
                let mut html = String::new();

                if let Some(pos) = content
                    .iter()
                    .position(|item| item.name.to_lowercase() == "readme.md")
                {
                    let readme = content.remove(pos);
                    // download readme lol
                    html.push_str(&format!("<h2>{}</h2>", &readme.name));
                }

                for item in content {
                    html.push_str(&format!(
                        "<div class=\"article-card\"><strong>{}</strong> - {} ({})</div>",
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
        });
    };

    PageType::new("Home", params, render).with_on_after_render(Some(Box::new(on_after_render)))
}
