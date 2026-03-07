use clap::Parser;
use colored::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use dialoguer::Input;
use dotenvy::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Deserialize, Serialize, Debug)]
struct License {
    name: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Contributor {
    login: String,
    contributions: u32,
}

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Serialize)]
struct FullRepoData {
    info: RepoInfo,
    top_contributors: Vec<Contributor>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The repository name in the format username/reponame
    #[arg(index = 1)]
    repo: Option<String>,

    /// Output data in JSON format
    #[arg(short, long)]
    json: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv();
    let args = Args::parse();
    
    let repo_name = match args.repo {
        Some(r) => r,
        None => {
            if args.json {
                // If JSON mode is on, we can't really do an interactive prompt cleanly to stdout
                // So we'll just error out.
                eprintln!("{}", "Error: No repository provided.".red());
                std::process::exit(1);
            }
            println!("{}", "No repository provided via arguments.".yellow());
            Input::<String>::new()
                .with_prompt("Please enter a repository (username/repo)")
                .interact_text()?
        }
    };

    let pb = ProgressBar::new_spinner();
    if !args.json {
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("|/-\\")
                .template("{spinner:.cyan} {msg}")?,
        );
        pb.set_message("Fetching repository data...");
        pb.enable_steady_tick(Duration::from_millis(100));
    }

    let client = reqwest::Client::new();
    let token = env::var("GITHUB_TOKEN").ok();

    // 1. Fetch Repo Info
    let repo_url = format!("https://api.github.com/repos/{}", repo_name);
    let mut repo_req = client.get(&repo_url).header(USER_AGENT, "repo-analyzer-cli");
    if let Some(ref t) = token {
        repo_req = repo_req.header(AUTHORIZATION, format!("Bearer {}", t));
    }
    let repo_res = repo_req.send().await?;

    // 2. Fetch Contributors
    let contrib_url = format!("https://api.github.com/repos/{}/contributors?per_page=5", repo_name);
    let mut contrib_req = client.get(&contrib_url).header(USER_AGENT, "repo-analyzer-cli");
    if let Some(ref t) = token {
        contrib_req = contrib_req.header(AUTHORIZATION, format!("Bearer {}", t));
    }
    let contrib_res = contrib_req.send().await?;

    if !args.json {
        pb.finish_and_clear();
    }

    if repo_res.status().is_success() {
        let repo_info: RepoInfo = repo_res.json().await?;
        let contributors: Vec<Contributor> = if contrib_res.status().is_success() {
            contrib_res.json().await?
        } else {
            Vec::new()
        };

        if args.json {
            let full_data = FullRepoData {
                info: repo_info,
                top_contributors: contributors,
            };
            println!("{}", serde_json::to_string_pretty(&full_data)?);
        } else {
            // Build Main Info Table
            let mut main_table = Table::new();
            main_table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_width(80)
                .set_header(vec![
                    Cell::new("Metric").fg(Color::Cyan).add_attribute(Attribute::Bold),
                    Cell::new("Details").fg(Color::Cyan).add_attribute(Attribute::Bold),
                ]);

            main_table.add_row(vec![
                Cell::new("Name").fg(Color::Blue).add_attribute(Attribute::Bold),
                Cell::new(&repo_info.name).fg(Color::White),
            ]);
            main_table.add_row(vec![
                Cell::new("URL").fg(Color::Blue).add_attribute(Attribute::Bold),
                Cell::new(&repo_info.html_url).fg(Color::DarkGrey).add_attribute(Attribute::Italic),
            ]);
            main_table.add_row(vec![
                Cell::new("Language").fg(Color::Blue).add_attribute(Attribute::Bold),
                Cell::new(repo_info.language.unwrap_or_else(|| "Unknown".to_string())).fg(Color::Green),
            ]);
            main_table.add_row(vec![
                Cell::new("Stars").fg(Color::Blue).add_attribute(Attribute::Bold),
                Cell::new(repo_info.stargazers_count.to_string()).fg(Color::Yellow),
            ]);
            main_table.add_row(vec![
                Cell::new("Forks").fg(Color::Blue).add_attribute(Attribute::Bold),
                Cell::new(repo_info.forks_count.to_string()).fg(Color::Magenta),
            ]);
            main_table.add_row(vec![
                Cell::new("Watchers").fg(Color::Blue).add_attribute(Attribute::Bold),
                Cell::new(repo_info.subscribers_count.to_string()).fg(Color::Cyan),
            ]);
            main_table.add_row(vec![
                Cell::new("Open Issues").fg(Color::Blue).add_attribute(Attribute::Bold),
                Cell::new(repo_info.open_issues_count.to_string()).fg(Color::Red),
            ]);
            main_table.add_row(vec![
                Cell::new("Size").fg(Color::Blue).add_attribute(Attribute::Bold),
                Cell::new(format!("{} KB", repo_info.size)).fg(Color::White),
            ]);
            
            let license_name = match repo_info.license {
                Some(l) => l.name,
                None => "No license found".to_string(),
            };
            main_table.add_row(vec![
                Cell::new("License").fg(Color::Blue).add_attribute(Attribute::Bold),
                Cell::new(license_name).fg(Color::White),
            ]);
            
            main_table.add_row(vec![
                Cell::new("Description").fg(Color::Blue).add_attribute(Attribute::Bold),
                Cell::new(repo_info.description.unwrap_or_else(|| "No description provided.".to_string())).add_attribute(Attribute::Italic),
            ]);

            println!("\n{}", main_table);

            if !contributors.is_empty() {
                let mut contrib_table = Table::new();
                contrib_table
                    .load_preset(UTF8_FULL)
                    .apply_modifier(UTF8_ROUND_CORNERS)
                    .set_content_arrangement(ContentArrangement::Dynamic)
                    .set_width(80)
                    .set_header(vec![
                        Cell::new("Top Contributors").fg(Color::Cyan).add_attribute(Attribute::Bold),
                        Cell::new("Contributions").fg(Color::Cyan).add_attribute(Attribute::Bold),
                    ]);

                for person in contributors {
                    contrib_table.add_row(vec![
                        Cell::new(&person.login).fg(Color::White).add_attribute(Attribute::Bold),
                        Cell::new(person.contributions.to_string()).fg(Color::Yellow),
                    ]);
                }
                println!("\n{}", contrib_table);
            }
        }

    } else {
        if !args.json {
            match repo_res.status().as_u16() {
                404 => println!("{} {}", "Error:".red().bold(), "Repository not found.".white()),
                403 => println!("{} {}", "Error:".red().bold(), "Rate limit exceeded or forbidden.".white()),
                _ => println!("{} {} (Status: {})", "Error:".red().bold(), "GitHub returned an error.".white(), repo_res.status()),
            }
        } else {
            eprintln!("Error: GitHub returned status {}", repo_res.status());
            std::process::exit(1);
        }
    }

    Ok(())
}
