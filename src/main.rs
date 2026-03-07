use clap::Parser;
use colored::*;
use dialoguer::Input;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct License {
    name: String,
}

#[derive(Deserialize, Debug)]
struct RepoInfo {
    name: String,
    stargazers_count: u32,
    open_issues_count: u32,
    forks_count: u32,
    subscribers_count: u32,
    size: u32,
    description: Option<String>,
    language: Option<String>,
    license: Option<License>,
    html_url: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The repository name in the format username/reponame
    #[arg(index = 1)]
    repo: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    // If repo is provided as a positional arg, use it. Otherwise, prompt the user.
    let repo_name = match args.repo {
        Some(r) => r,
        None => {
            println!("{}", "No repository provided via arguments.".yellow());
            Input::<String>::new()
                .with_prompt("Please enter a repository (username/repo)")
                .interact_text()?
        }
    };

    println!("\n{} {}...\n", "Scanning repository:".cyan().bold(), repo_name.yellow());

    let url = format!("https://api.github.com/repos/{}", repo_name);
    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header(USER_AGENT, "repo-analyzer-cli")
        .send()
        .await?;

    if response.status().is_success() {
        let repo_info: RepoInfo = response.json().await?;

        println!("{}", "--- Repository Status ---".magenta().bold());
        println!("{:<15} {}", "Name:".blue().bold(), repo_info.name.white());
        println!("{:<15} {}", "URL:".blue().bold(), repo_info.html_url.underline().white());
        println!("{:<15} {}", "Stars:".blue().bold(), repo_info.stargazers_count.to_string().yellow());
        println!("{:<15} {}", "Forks:".blue().bold(), repo_info.forks_count.to_string().green());
        println!("{:<15} {}", "Watchers:".blue().bold(), repo_info.subscribers_count.to_string().cyan());
        println!("{:<15} {}", "Open Issues:".blue().bold(), repo_info.open_issues_count.to_string().red());
        println!("{:<15} {} KB", "Size:".blue().bold(), repo_info.size.to_string().white());
        println!("{:<15} {}", "Language:".blue().bold(), repo_info.language.unwrap_or_else(|| "Unknown".to_string()).green());
        
        let license_name = match repo_info.license {
            Some(l) => l.name,
            None => "No license found".to_string(),
        };
        println!("{:<15} {}", "License:".blue().bold(), license_name.white());
        
        println!("{:<15} {}", "Description:".blue().bold(), repo_info.description.unwrap_or_else(|| "No description provided.".to_string()).italic().white());
        println!("{}", "---------------------------".magenta().bold());
    } else if response.status().as_u16() == 404 {
        println!("{} {}", "Error:".red().bold(), "Repository not found. Check the username/reponame format.".white());
    } else if response.status().as_u16() == 403 {
        println!("{} {}", "Error:".red().bold(), "GitHub Rate limit exceeded! Wait an hour or use a token.".white());
    } else {
        println!("{} {} (Status: {})", "Error:".red().bold(), "GitHub returned an error.".white(), response.status());
    }

    Ok(())
}
