use directories::ProjectDirs;
use std::fs;
use std::io;
use std::path::PathBuf;

use super::SaveData;

/// Gets the save directory for the game, creating it if necessary
fn get_save_dir() -> io::Result<PathBuf> {
    let project_dirs = ProjectDirs::from("com", "BevyDriver", "BevyDriver")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not determine save directory"))?;

    let save_dir = project_dirs.data_dir().join("saves");
    fs::create_dir_all(&save_dir)?;
    Ok(save_dir)
}

/// Saves game data to a JSON file
pub fn save_to_file(save_data: &SaveData) -> io::Result<()> {
    let save_dir = get_save_dir()?;
    let file_path = save_dir.join(save_data.filename());

    let json = serde_json::to_string_pretty(save_data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    fs::write(file_path, json)?;
    Ok(())
}

/// Loads game data from a JSON file by player name
pub fn load_from_file(filename: &str) -> io::Result<SaveData> {
    let save_dir = get_save_dir()?;
    let file_path = save_dir.join(filename);

    let json = fs::read_to_string(file_path)?;
    let save_data: SaveData = serde_json::from_str(&json)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(save_data)
}

/// Deletes a save file
pub fn delete_save_file(filename: &str) -> io::Result<()> {
    let save_dir = get_save_dir()?;
    let file_path = save_dir.join(filename);
    fs::remove_file(file_path)?;
    Ok(())
}

/// Lists all available save files with their metadata
pub fn list_saves() -> io::Result<Vec<SaveData>> {
    let save_dir = get_save_dir()?;
    let mut saves = Vec::new();

    if !save_dir.exists() {
        return Ok(saves);
    }

    for entry in fs::read_dir(save_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "json") {
            if let Ok(json) = fs::read_to_string(&path) {
                if let Ok(save_data) = serde_json::from_str::<SaveData>(&json) {
                    saves.push(save_data);
                }
            }
        }
    }

    // Sort by last played (most recent first)
    saves.sort_by(|a, b| b.last_played.cmp(&a.last_played));

    Ok(saves)
}

/// Checks if a save file exists for the given player name
pub fn save_exists(player_name: &str) -> bool {
    if let Ok(save_dir) = get_save_dir() {
        let safe_name: String = player_name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect();
        let file_path = save_dir.join(format!("{}.json", safe_name));
        file_path.exists()
    } else {
        false
    }
}
