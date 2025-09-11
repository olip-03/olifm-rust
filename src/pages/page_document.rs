use crate::console_log;
use crate::content::{get_global_content, get_global_document, replace_images, strip_frontmatter};
use crate::get_app;
use crate::get_base_url;
use crate::get_document;
use crate::log;
use crate::page::Page as PageType;
use crate::pages::macros::Style;
use crate::pages::macros::load_readme;
use crate::render_site;
use crate::setup_article_observer;
use content_service::{Img, JsonEntry};
use pulldown_cmark::{Parser, html};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub fn page_document(document: &str) -> PageType {
    let mut params = HashMap::new();

    let container_id = "document-content";

    let render = move |p: &PageType| {
        format!(
            r#"
            <div class="document-container">
                <div id="{}" class="document-content">
                    <div class="loading">Fetching document...</div>
                </div>
            </div>
            "#,
            container_id
        )
    };

    let document_path = document.to_string();
    let on_after_render = move || {
        let document_path = document_path.clone();

        spawn_local(async move {
            let decoded_path = urlencoding::decode(&document_path)
                .unwrap_or_else(|_| document_path.clone().into())
                .into_owned();

            let mut base_path = "/blog".to_string();
            let url = if decoded_path.starts_with("/blog/") {
                base_path = "/blog".to_string();
                format!("{}/content{}", get_base_url!(), decoded_path)
            } else if decoded_path.starts_with("/pictures/") {
                base_path = "/pictures".to_string();
                format!("{}/content{}", get_base_url!(), decoded_path)
            } else {
                format!("{}/content/blog/{}", get_base_url!(), decoded_path)
            };

            let mut img: Vec<Img> = Vec::new();
            let items: Vec<JsonEntry> =
                get_global_content(decoded_path.clone(), Some("file".to_string()))
                    .await
                    .expect("should have content");
            if let Some(pos) = items.iter().position(|item| item.path == decoded_path) {
                let meta = items[pos].clone();
                img = meta.images.clone();
            }

            match get_global_document(&url).await {
                Ok(markdown_content) => {
                    let untagged_content = strip_frontmatter(&markdown_content);
                    let fixed_content = replace_images(&untagged_content, &img);

                    let parser = Parser::new(&fixed_content);
                    let mut html_output = String::new();
                    html::push_html(&mut html_output, parser);

                    if let Some(element) = get_document!().get_element_by_id(container_id) {
                        element.set_inner_html(&html_output);
                    } else {
                        console_log!("Could not find document container element");
                    }
                }
                Err(e) => {
                    console_log!("Failed to load document '{}': {:?}", decoded_path, e);
                    if let Some(element) = get_document!().get_element_by_id(container_id) {
                        element.set_inner_html(&format!(
                            r#"
                            <div class="error">
                                <h2>Document Not Found</h2>
                                <p>Could not load document: {}</p>
                                <p>Error: {:?}</p>
                            </div>
                            "#,
                            decoded_path, e
                        ));
                    }
                }
            }
        });
    };

    PageType::new(document, params, render).with_on_after_render(Some(Box::new(on_after_render)))
}
