use clap::Parser;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct RepoInfo {
    name: String,
    stargazers_count: u32,
    open_issues_count: u32,
    description: Option<String>,
    language: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    repo: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("Scanning repository: {}...\n", args.repo);

    let url = format!("https://api.github.com/repos/{}", args.repo);
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header(USER_AGENT, "repo-analyzer-cli")
        .send()
        .await?;

    if response.status().is_success() {
        let repo_info: RepoInfo = response.json().await?;

        println!("--- Repository Status ---");
        println!("Name:      {}", repo_info.name);
        println!("Stars:     {}", repo_info.stargazers_count);
        println!("Issues:    {}", repo_info.open_issues_count);
        println!("Language:  {}", repo_info.language.unwrap_or_else(|| "Unknown".to_string()));
        println!("Desc:      {}", repo_info.description.unwrap_or_else(|| "No description provided.".to_string()));
    } else {
        println!("Error: Could not find repository. GitHub returned status: {}", response.status());
    }

    Ok(())
}
