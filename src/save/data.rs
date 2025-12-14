use bevy::prelude::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a saved game with player progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    /// Player's chosen name
    pub player_name: String,
    /// Highest level the player has unlocked (can play up to this level)
    pub highest_level_unlocked: usize,
    /// Best completion time for each level (level number -> time in seconds)
    pub level_times: HashMap<usize, f32>,
    /// When this save was first created
    pub created_at: DateTime<Utc>,
    /// When this save was last played
    pub last_played: DateTime<Utc>,
}

impl SaveData {
    /// Creates a new save data for a new player
    pub fn new(player_name: String) -> Self {
        let now = Utc::now();
        Self {
            player_name,
            highest_level_unlocked: 1,
            level_times: HashMap::new(),
            created_at: now,
            last_played: now,
        }
    }

    /// Records a level completion, updating best time if this is faster
    /// Returns true if this was a new best time
    pub fn record_level_completion(&mut self, level: usize, time: f32) -> bool {
        self.last_played = Utc::now();

        // Unlock next level if this is the highest completed
        if level >= self.highest_level_unlocked {
            self.highest_level_unlocked = level + 1;
        }

        // Update best time if this is faster (or first completion)
        let is_new_best = match self.level_times.get(&level) {
            Some(&best_time) => time < best_time,
            None => true,
        };

        if is_new_best {
            self.level_times.insert(level, time);
        }

        is_new_best
    }

    /// Gets the best time for a level, if any
    #[allow(dead_code)]
    pub fn get_best_time(&self, level: usize) -> Option<f32> {
        self.level_times.get(&level).copied()
    }

    /// Generates a safe filename from the player name
    pub fn filename(&self) -> String {
        let safe_name: String = self.player_name
            .chars()
            .map(|c| if c.is_alphanumeric() || c == '_' || c == '-' { c } else { '_' })
            .collect();
        format!("{}.json", safe_name)
    }
}

/// Resource to track the currently active save game
#[derive(Resource, Default)]
pub struct CurrentSave(pub Option<SaveData>);

impl CurrentSave {
    /// Returns a reference to the current save data, if loaded
    #[allow(dead_code)]
    pub fn get(&self) -> Option<&SaveData> {
        self.0.as_ref()
    }

    /// Returns a mutable reference to the current save data, if loaded
    pub fn get_mut(&mut self) -> Option<&mut SaveData> {
        self.0.as_mut()
    }

    /// Sets the current save data
    pub fn set(&mut self, save: SaveData) {
        self.0 = Some(save);
    }

    /// Clears the current save (e.g., when returning to main menu)
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.0 = None;
    }
}
