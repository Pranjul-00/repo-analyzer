use clap::Parser;
use colored::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use dialoguer::Input;
use dotenvy::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use std::env;
use std::time::Duration;

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
    // Load environment variables from .env file if it exists
    let _ = dotenv();
    
    let args = Args::parse();
    
    let repo_name = match args.repo {
        Some(r) => r,
        None => {
            println!("{}", "No repository provided via arguments.".yellow());
            Input::<String>::new()
                .with_prompt("Please enter a repository (username/repo)")
                .interact_text()?
        }
    };

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("|/-\\")
            .template("{spinner:.cyan} {msg}")?,
    );
    pb.set_message("Fetching repository data...");
    pb.enable_steady_tick(Duration::from_millis(100));

    let url = format!("https://api.github.com/repos/{}", repo_name);
    let client = reqwest::Client::new();
    
    // Check if GITHUB_TOKEN is set in the environment
    let mut request_builder = client
        .get(&url)
        .header(USER_AGENT, "repo-analyzer-cli");

    if let Ok(token) = env::var("GITHUB_TOKEN") {
        request_builder = request_builder.header(AUTHORIZATION, format!("Bearer {}", token));
    }

    let response = request_builder.send().await?;

    pb.finish_and_clear();

    if response.status().is_success() {
        let repo_info: RepoInfo = response.json().await?;

        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(vec![
                Cell::new("Metric").fg(Color::Cyan).add_attribute(Attribute::Bold),
                Cell::new("Details").fg(Color::Cyan).add_attribute(Attribute::Bold),
            ]);

        table.add_row(vec![
            Cell::new("Name").fg(Color::Blue).add_attribute(Attribute::Bold),
            Cell::new(&repo_info.name).fg(Color::White),
        ]);
        table.add_row(vec![
            Cell::new("URL").fg(Color::Blue).add_attribute(Attribute::Bold),
            Cell::new(&repo_info.html_url).fg(Color::DarkGrey).add_attribute(Attribute::Italic),
        ]);
        table.add_row(vec![
            Cell::new("Language").fg(Color::Blue).add_attribute(Attribute::Bold),
            Cell::new(repo_info.language.unwrap_or_else(|| "Unknown".to_string())).fg(Color::Green),
        ]);
        table.add_row(vec![
            Cell::new("Stars").fg(Color::Blue).add_attribute(Attribute::Bold),
            Cell::new(repo_info.stargazers_count.to_string()).fg(Color::Yellow),
        ]);
        table.add_row(vec![
            Cell::new("Forks").fg(Color::Blue).add_attribute(Attribute::Bold),
            Cell::new(repo_info.forks_count.to_string()).fg(Color::Magenta),
        ]);
        table.add_row(vec![
            Cell::new("Watchers").fg(Color::Blue).add_attribute(Attribute::Bold),
            Cell::new(repo_info.subscribers_count.to_string()).fg(Color::Cyan),
        ]);
        table.add_row(vec![
            Cell::new("Open Issues").fg(Color::Blue).add_attribute(Attribute::Bold),
            Cell::new(repo_info.open_issues_count.to_string()).fg(Color::Red),
        ]);
        table.add_row(vec![
            Cell::new("Size").fg(Color::Blue).add_attribute(Attribute::Bold),
            Cell::new(format!("{} KB", repo_info.size)).fg(Color::White),
        ]);
        
        let license_name = match repo_info.license {
            Some(l) => l.name,
            None => "No license found".to_string(),
        };
        table.add_row(vec![
            Cell::new("License").fg(Color::Blue).add_attribute(Attribute::Bold),
            Cell::new(license_name).fg(Color::White),
        ]);
        
        table.add_row(vec![
            Cell::new("Description").fg(Color::Blue).add_attribute(Attribute::Bold),
            Cell::new(repo_info.description.unwrap_or_else(|| "No description provided.".to_string())).add_attribute(Attribute::Italic),
        ]);

        println!("\n{}", table);
    } else if response.status().as_u16() == 404 {
        println!("{} {}", "Error:".red().bold(), "Repository not found. Check the username/reponame format.".white());
    } else if response.status().as_u16() == 403 {
        println!("{} {}", "Error:".red().bold(), "GitHub Rate limit exceeded! Wait an hour or use a token.".white());
    } else {
        println!("{} {} (Status: {})", "Error:".red().bold(), "GitHub returned an error.".white(), response.status());
    }

    Ok(())
}
