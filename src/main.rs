use shared::github_service;

#[tokio::main]
async fn main() {
    // let args: Vec<String> = std::env::args().collect();

    // Example 1: Using the original getJson function
    // println!("=== Original getJson example ===");
    // match getJson("https://jsonplaceholder.typicode.com/posts/1").await {
    //     Ok(result) => println!("{}", result),
    //     Err(e) => println!("Error: {}", e),
    // }

    // Example 2: Using the GitHub service
    // println!("\n=== GitHub Service example ===");
    let github = github_service();

    println!("GitHub API URL: {}", github.get_url());

    // Get user information
    match github.get_user("octocat").await {
        Ok(user) => {
            println!(
                "User: {} ({})",
                user.login,
                user.name.unwrap_or("No name".to_string())
            );
            println!(
                "Public repos: {}, Followers: {}",
                user.public_repos, user.followers
            );
        }
        Err(e) => println!("Error getting user: {:?}", e),
    }

    // Get repository information
    match github.get_repo("octocat", "Hello-World").await {
        Ok(repo) => {
            println!("Repo: {}", repo.full_name);
            println!(
                "Description: {}",
                repo.description.unwrap_or("No description".to_string())
            );
            println!(
                "Stars: {}, Language: {}",
                repo.stargazers_count,
                repo.language.unwrap_or("Unknown".to_string())
            );
        }
        Err(e) => println!("Error getting repo: {:?}", e),
    }
}
