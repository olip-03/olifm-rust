use crate::console_log;
use crate::log;
use futures::join;
use github::{GithubClient, RepoContent, error::GithubError};
use wasm_bindgen_futures::spawn_local;

pub enum Style {
    Card,
    Photo,
    Music,
}

#[macro_export]
macro_rules! render_site {
    ($owner:expr, $repo:expr, $path:expr, $style:expr) => {{
        let owner = $owner.to_string();
        let repo = $repo.to_string();
        let path = $path.to_string();
        let doc_url = format!(
            "https://raw.githubusercontent.com/{}/{}/main/{}/readme.md",
            owner, repo, path
        );

        wasm_bindgen_futures::spawn_local(async move {
            match crate::pages::macros::get_github_data(&owner, &repo, &path, &doc_url).await {
                Ok((mut repo_content, document)) => {
                    let mut html = String::new();

                    if let Some(pos) = repo_content
                        .iter()
                        .position(|item| item.name.to_lowercase() == "readme.md")
                    {
                        let readme = repo_content.remove(pos);
                        // download readme lol
                        html.push_str(&format!("{}", &document));
                    }

                    for item in repo_content {
                        html.push_str(&format!(
                            "<div class=\"article-card\"><strong>{}</strong> - {} ({})</div>",
                            item.name, item.content_type, item.size
                        ));
                    }

                    get_app!().set_inner_html(&html);
                }
                Err(e) => {
                    crate::console_log!("Error fetching GitHub data: {:?}", e);
                    get_app!().set_inner_html(&format!(
                        "<p style=\"color: red;\">Error loading content: {}</p>",
                        e
                    ));
                }
            }
        });
    }};
}

pub async fn get_github_data(
    owner: &str,
    repo: &str,
    path: &str,
    doc_url: &str,
) -> Result<(Vec<RepoContent>, String), GithubError> {
    let client = GithubClient::new();

    let repo_content_fut = client.get_repo_content(owner, repo, path);
    let document_fut = client.get_document(doc_url);

    let (repo_content_res, document_res) = futures::join!(repo_content_fut, document_fut);

    let repo_content = repo_content_res?;
    let document = document_res?;

    Ok((repo_content, document))
}
