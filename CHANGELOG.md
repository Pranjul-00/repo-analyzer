# Changelog

All notable changes to this project will be documented in this file.

## [1.2.0] - 2026-03-07

### Added
- **Multi-repo Comparison**: Compare any number of repositories side-by-side.
- **Rate Limit Status**: Displays remaining GitHub API requests and reset time.
- **Full Name Display**: Clearly shows the owner and repository names in all views.
- **JSON Export**: Added `-j/--json` flag for machine-readable output.

## [1.1.0] - 2026-03-07

### Added
- **GitHub Token Support**: Load tokens from `.env` via `dotenvy` for higher rate limits.
- **Top Contributors**: Fetch and display the top 5 contributors for each repo.
- **Interactive Prompts**: Added `dialoguer` for missing input.
- **Professional UI**: Switched to `comfy_table` and `indicatif` for tables and spinners.

## [1.0.0] - 2026-03-07

### Added
- Initial release with basic repository metadata (stars, forks, issues).
- `clap` support for basic CLI arguments.
- `reqwest` and `serde` integration for GitHub API communication.
