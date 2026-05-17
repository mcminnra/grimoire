use std::fmt;
use std::fs;
use std::path;

use anyhow::Context;
use gray_matter::ParsedEntity;
use gray_matter::{Matter, engine::YAML};

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum LogStatus {
    Backlog,
    Playing,
    Played,
}

impl fmt::Display for LogStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LogStatus::Backlog => "Backlog",
            LogStatus::Playing => "Playing",
            LogStatus::Played => "Played",
        };
        write!(f, "{s}")
    }
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum PlayedCategory {
    Played,
    Abandoned,
    Completed,
    #[serde(alias = "completed+", alias = "completedplus")]
    CompletedPlus,
    Mastered,
}

impl fmt::Display for PlayedCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PlayedCategory::Played => "Played",
            PlayedCategory::Abandoned => "Abandoned",
            PlayedCategory::Completed => "Completed",
            PlayedCategory::CompletedPlus => "Completed+",
            PlayedCategory::Mastered => "Mastered",
        };
        write!(f, "{s}")
    }
}

#[derive(serde::Deserialize, Debug)]
struct Frontmatter {
    title: String,
    log_status: LogStatus,
    played_category: Option<PlayedCategory>,
    rating: Option<i8>,
    played_platform: Option<String>,
    revisit: Option<bool>,
    tags: Option<Vec<String>>,
}

pub fn list_cmd(games_directory: &str) -> anyhow::Result<()> {
    let mut game_filepaths = Vec::<path::PathBuf>::new();

    // Get all filepaths
    // TODO: Add support for recursing into directories
    let dir_iter = fs::read_dir(games_directory)
        .with_context(|| format!("Unable to find filepath {games_directory}"))?;

    for entry in dir_iter {
        let entry = entry?;
        let ft = entry.file_type()?;
        let path = entry.path();

        if ft.is_dir() {
            println!("{} is a directory. Skipping...", path.display());
            continue;
        }
        game_filepaths.push(path);
    }
    game_filepaths.sort();

    // Parse YAML
    let matter = Matter::<YAML>::new();

    for path in game_filepaths {
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Unable to read {}", path.display()))?;
        let parsed: ParsedEntity = matter.parse(&contents)?;

        if let Some(data) = parsed.data {
            let fm: Frontmatter = data.deserialize()?;

            println!("{}", path.display());
            println!(
                "{} - {}/{}",
                fm.title,
                fm.log_status,
                fm.played_category
                    .map_or("None".to_string(), |v| v.to_string()),
            );
            println!(
                "  Played Platform: {}",
                fm.played_platform.map_or("None".to_string(), |v| v)
            );
            println!(
                "  Rating: {}",
                fm.rating.map_or("None".to_string(), |v| v.to_string())
            );
            println!("  Revisit: {}", fm.revisit.unwrap_or_default());

            if let Some(tags) = fm.tags {
                println!("  tags:");
                for tag in tags {
                    println!("    - {tag}");
                }
            }

            println!();
        }
    }

    Ok(())
}
