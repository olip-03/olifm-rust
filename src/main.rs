use futures::future;
use github::{github_service, models::RepoContent};
use smol::io;

fn main() -> io::Result<()> {
    // test user service
    smol::block_on(async {
        let service = github_service();

        let result = service
            .get_repo_content("olip-03", "oli-fm-content", "")
            .await;

        match result {
            Ok(contents) => {
                for item in contents {
                    println!("Found document: {}", item.name)
                }
            }
            Err(e) => {
                // handle GithubError (InvalidInput, NotFound, RateLimited, etc.)
                eprintln!("Failed to fetch repo content: {:?}", e);
            }
        }

        // Concurrent execution test
        let mut usernames = vec!["olip-03", "torvalds", "graydon", "octocat"];
        usernames.push("mary-ext");

        let user_futures: Vec<_> = usernames
            .iter()
            .map(|username| service.get_user(username))
            .collect();
        let results = future::join_all(user_futures).await;
        for (i, result) in results.into_iter().enumerate() {
            match result {
                Ok(user) => {
                    let name = user.name.as_deref().unwrap_or("No name");
                    println!("User {}: {} ({})", i + 1, name, user.login);
                }
                Err(e) => eprintln!("Failed to get user {}: {:?}", i + 1, e),
            }
        }

        Ok(())
    })
}
