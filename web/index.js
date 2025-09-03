import init, {
  get_github_user,
  get_github_repo,
  get_github_api_url,
} from "./pkg/web.js";

async function run() {
  console.log("Starting WASM initialization...");
  try {
    // Initialize the wasm module
    console.log("Calling init()...");
    await init();
    console.log("WASM module initialized successfully!");

    // Test GitHub service functions
    console.log("GitHub API URL:", get_github_api_url());

    // Fetch GitHub user data
    try {
      console.log("Fetching GitHub user data...");
      const userData = await get_github_user("octocat");
      const user = JSON.parse(userData);
      console.log("GitHub User:", user.login, "-", user.name);

      // Add user info to DOM
      const userInfo = document.createElement("div");
      userInfo.innerHTML = `
        <h3>GitHub User: ${user.login}</h3>
        <p><strong>Name:</strong> ${user.name || "N/A"}</p>
        <p><strong>Public Repos:</strong> ${user.public_repos}</p>
        <p><strong>Followers:</strong> ${user.followers}</p>
      `;
      userInfo.style.cssText =
        "margin: 20px; padding: 10px; border: 1px solid #ccc; background-color: #f9f9f9;";
      document.body.appendChild(userInfo);
    } catch (error) {
      console.error("Error fetching user:", error);
    }

    // Fetch GitHub repository data
    try {
      console.log("Fetching GitHub repository data...");
      const repoData = await get_github_repo("octocat", "Hello-World");
      const repo = JSON.parse(repoData);
      console.log("GitHub Repo:", repo.full_name);

      // Add repo info to DOM
      const repoInfo = document.createElement("div");
      repoInfo.innerHTML = `
        <h3>GitHub Repository: ${repo.full_name}</h3>
        <p><strong>Description:</strong> ${repo.description || "No description"}</p>
        <p><strong>Language:</strong> ${repo.language || "Unknown"}</p>
        <p><strong>Stars:</strong> ${repo.stargazers_count}</p>
        <p><strong>Forks:</strong> ${repo.forks_count}</p>
        <p><strong>URL:</strong> <a href="${repo.html_url}" target="_blank">${repo.html_url}</a></p>
      `;
      repoInfo.style.cssText =
        "margin: 20px; padding: 10px; border: 1px solid #ccc; background-color: #f0f8ff;";
      document.body.appendChild(repoInfo);
    } catch (error) {
      console.error("Error fetching repository:", error);
    }
  } catch (error) {
    console.error("Failed to initialize WASM module:", error);
  }
}

console.log("Loading script...");
run().catch(console.error);
