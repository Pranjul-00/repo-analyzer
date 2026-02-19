use clap::Parser;
use reqwest::header::USER_AGENT;
use serde::Deserialize;

// --- STEP 1: Define our Data Structure ---
// GitHub sends back a massive JSON file. We don't want all of it.
// We only want specific fields. We define a 'struct' (custom data type) for this.
// #[derive(Deserialize)] tells Rust to automatically write the code to match JSON to these fields.
#[derive(Deserialize, Debug)]
struct RepoInfo {
    name: String,
    stargazers_count: u32, // u32 means an unsigned (positive) 32-bit integer
    open_issues_count: u32,
    
    // We use Option<String> because some repositories don't have a description or a primary language.
    // Instead of crashing, Rust safely stores 'None' if the data is missing.
    description: Option<String>, 
    language: Option<String>,
}

// --- STEP 2: Command Line Arguments ---
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    repo: String,
}

// --- STEP 3: The Main Function ---
// #[tokio::main] sets up the async environment. 
// We return a Result because network requests can fail (e.g., no Wi-Fi).
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    println!("Scanning repository: {}...\n", args.repo);

    // Format the exact URL we need to hit
    let url = format!("https://api.github.com/repos/{}", args.repo);

    // Create a web client
    let client = reqwest::Client::new();

    
    // Make the HTTP GET request
    let response = client
        .get(&url)
        // GitHub API STRICTLY requires a User-Agent header, or it blocks the request
        .header(USER_AGENT, "repo-analyzer-cli") 
        .send()
        .await?; // The '?' is Rust's magic error handler. 

    // --- STEP 4: Process the Response ---
    if response.status().is_success() {
        // Parse the JSON into our RepoInfo struct
        let repo_info: RepoInfo = response.json().await?;

        // Print the results nicely
        println!("--- Repository Status ---");
        println!("Name:      {}", repo_info.name);
        println!("Stars:     {}", repo_info.stargazers_count);
        
        // unwrap_or_else safely handles the Option types. If it's 'None', it prints the fallback text.
        println!("Language:  {}", repo_info.language.unwrap_or_else(|| "Unknown".to_string()));
        println!("Desc:      {}", repo_info.description.unwrap_or_else(|| "No description provided.".to_string()));
    } else {
        println!("Error: Could not find repository. GitHub returned status: {}", response.status());
    }

    Ok(()) // Tells Rust the program finished successfully
}
