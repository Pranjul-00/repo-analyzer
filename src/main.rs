use clap::Parser;
use colored::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use dialoguer::Input;
use dotenvy::dotenv;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use std::env;
use std::time::Duration;

#[derive(Deserialize, Serialize, Debug, Clone)]
struct License {
    name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Contributor {
    login: String,
    contributions: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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
    /// The repository names in the format username/reponame (provide multiple to compare)
    #[arg(index = 1)]
    repos: Vec<String>,

    /// Output data in JSON format
    #[arg(short, long)]
    json: bool,
}

async fn fetch_repo_data(
    client: &reqwest::Client,
    repo_name: &str,
    token: Option<&String>,
) -> Result<FullRepoData, Box<dyn std::error::Error>> {
    let repo_url = format!("https://api.github.com/repos/{}", repo_name);
    let mut repo_req = client.get(&repo_url).header(USER_AGENT, "repo-analyzer-cli");
    if let Some(t) = token {
        repo_req = repo_req.header(AUTHORIZATION, format!("Bearer {}", t));
    }
    let repo_res = repo_req.send().await?;

    if !repo_res.status().is_success() {
        return Err(format!("Failed to fetch repo {}: {}", repo_name, repo_res.status()).into());
    }

    let repo_info: RepoInfo = repo_res.json().await?;

    let contrib_url = format!("https://api.github.com/repos/{}/contributors?per_page=5", repo_name);
    let mut contrib_req = client.get(&contrib_url).header(USER_AGENT, "repo-analyzer-cli");
    if let Some(t) = token {
        contrib_req = contrib_req.header(AUTHORIZATION, format!("Bearer {}", t));
    }
    let contrib_res = contrib_req.send().await?;

    let contributors: Vec<Contributor> = if contrib_res.status().is_success() {
        contrib_res.json().await?
    } else {
        Vec::new()
    };

    Ok(FullRepoData {
        info: repo_info,
        top_contributors: contributors,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = dotenv();
    let args = Args::parse();
    
    let mut repo_names = args.repos;
    
    if repo_names.is_empty() {
        if args.json {
            eprintln!("{}", "Error: No repository provided.".red());
            std::process::exit(1);
        }
        println!("{}", "No repository provided via arguments.".yellow());
        let input = Input::<String>::new()
            .with_prompt("Please enter a repository (username/repo)")
            .interact_text()?;
        repo_names.push(input);
    }

    let client = reqwest::Client::new();
    let token = env::var("GITHUB_TOKEN").ok();
    
    let m = MultiProgress::new();
    let sty = ProgressStyle::default_spinner()
        .tick_chars("|/-\\")
        .template("{spinner:.cyan} {msg}")?;

    let mut results = Vec::new();

    for name in &repo_names {
        let pb = if !args.json {
            let pb = m.add(ProgressBar::new_spinner());
            pb.set_style(sty.clone());
            pb.set_message(format!("Fetching {}...", name));
            pb.enable_steady_tick(Duration::from_millis(100));
            Some(pb)
        } else {
            None
        };

        match fetch_repo_data(&client, name, token.as_ref()).await {
            Ok(data) => {
                if let Some(p) = pb { p.finish_and_clear(); }
                results.push(data);
            }
            Err(e) => {
                if let Some(p) = pb { p.finish_and_clear(); }
                if args.json {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                } else {
                    println!("{} {}", "Error:".red().bold(), e.to_string().white());
                }
            }
        }
    }

    if results.is_empty() { return Ok(()); }

    if args.json {
        println!("{}", serde_json::to_string_pretty(&results)?);
    } else if results.len() == 1 {
        let data = &results[0];
        let info = &data.info;
        
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

        table.add_row(vec![Cell::new("Name").fg(Color::Blue).add_attribute(Attribute::Bold), Cell::new(&info.name)]);
        table.add_row(vec![Cell::new("URL").fg(Color::Blue).add_attribute(Attribute::Bold), Cell::new(&info.html_url).fg(Color::DarkGrey).add_attribute(Attribute::Italic)]);
        table.add_row(vec![Cell::new("Language").fg(Color::Blue).add_attribute(Attribute::Bold), Cell::new(info.language.as_deref().unwrap_or("Unknown")).fg(Color::Green)]);
        table.add_row(vec![Cell::new("Stars").fg(Color::Blue).add_attribute(Attribute::Bold), Cell::new(info.stargazers_count.to_string()).fg(Color::Yellow)]);
        table.add_row(vec![Cell::new("Forks").fg(Color::Blue).add_attribute(Attribute::Bold), Cell::new(info.forks_count.to_string()).fg(Color::Magenta)]);
        table.add_row(vec![Cell::new("Watchers").fg(Color::Blue).add_attribute(Attribute::Bold), Cell::new(info.subscribers_count.to_string()).fg(Color::Cyan)]);
        table.add_row(vec![Cell::new("Issues").fg(Color::Blue).add_attribute(Attribute::Bold), Cell::new(info.open_issues_count.to_string()).fg(Color::Red)]);
        table.add_row(vec![Cell::new("Size").fg(Color::Blue).add_attribute(Attribute::Bold), Cell::new(format!("{} KB", info.size))]);
        
        let lic = info.license.as_ref().map(|l| l.name.clone()).unwrap_or_else(|| "No license".to_string());
        table.add_row(vec![Cell::new("License").fg(Color::Blue).add_attribute(Attribute::Bold), Cell::new(lic)]);
        table.add_row(vec![Cell::new("Description").fg(Color::Blue).add_attribute(Attribute::Bold), Cell::new(info.description.as_deref().unwrap_or("None")).add_attribute(Attribute::Italic)]);

        println!("\n{}", table);

        if !data.top_contributors.is_empty() {
            let mut ct = Table::new();
            ct.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS).set_header(vec![
                Cell::new("Top Contributors").fg(Color::Cyan).add_attribute(Attribute::Bold),
                Cell::new("Contributions").fg(Color::Cyan).add_attribute(Attribute::Bold),
            ]);
            for c in &data.top_contributors {
                ct.add_row(vec![Cell::new(&c.login).add_attribute(Attribute::Bold), Cell::new(c.contributions.to_string()).fg(Color::Yellow)]);
            }
            println!("\n{}", ct);
        }
    } else {
        // Multi-repo Comparison Mode (2 or more)
        let mut comp = Table::new();
        comp.load_preset(UTF8_FULL).apply_modifier(UTF8_ROUND_CORNERS).set_content_arrangement(ContentArrangement::Dynamic);
        
        let mut header = vec![Cell::new("Metric").fg(Color::Cyan).add_attribute(Attribute::Bold)];
        for res in &results {
            header.push(Cell::new(&res.info.name).fg(Color::Yellow).add_attribute(Attribute::Bold));
        }
        comp.set_header(header);

        // Language Row
        let mut lang_row = vec![Cell::new("Language").fg(Color::Blue).add_attribute(Attribute::Bold)];
        for res in &results {
            lang_row.push(Cell::new(res.info.language.as_deref().unwrap_or("-")).fg(Color::Green));
        }
        comp.add_row(lang_row);

        // Stars Row
        let mut stars_row = vec![Cell::new("Stars").fg(Color::Blue).add_attribute(Attribute::Bold)];
        for res in &results {
            stars_row.push(Cell::new(res.info.stargazers_count.to_string()).fg(Color::Yellow));
        }
        comp.add_row(stars_row);

        // Forks Row
        let mut forks_row = vec![Cell::new("Forks").fg(Color::Blue).add_attribute(Attribute::Bold)];
        for res in &results {
            forks_row.push(Cell::new(res.info.forks_count.to_string()).fg(Color::Magenta));
        }
        comp.add_row(forks_row);

        // Watchers Row
        let mut watch_row = vec![Cell::new("Watchers").fg(Color::Blue).add_attribute(Attribute::Bold)];
        for res in &results {
            watch_row.push(Cell::new(res.info.subscribers_count.to_string()).fg(Color::Cyan));
        }
        comp.add_row(watch_row);

        // Issues Row
        let mut issues_row = vec![Cell::new("Issues").fg(Color::Blue).add_attribute(Attribute::Bold)];
        for res in &results {
            issues_row.push(Cell::new(res.info.open_issues_count.to_string()).fg(Color::Red));
        }
        comp.add_row(issues_row);

        // Size Row
        let mut size_row = vec![Cell::new("Size (KB)").fg(Color::Blue).add_attribute(Attribute::Bold)];
        for res in &results {
            size_row.push(Cell::new(res.info.size.to_string()));
        }
        comp.add_row(size_row);

        println!("\n{}", "--- Side-by-Side Comparison ---".bold().magenta());
        println!("{}", comp);
    }

    Ok(())
}
