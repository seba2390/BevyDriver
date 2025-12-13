//! Random track generation using self-avoiding walk algorithm

use bevy::prelude::*;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::road::components::{Direction, RoadSegmentType};
use crate::road::constants::ROAD_SEGMENT_LENGTH;
use crate::road::helpers::get_exit_direction;

/// A dynamically generated track with owned layout
pub struct GeneratedTrack {
    /// The sequence of road segments that make up the track
    pub layout: Vec<RoadSegmentType>,
    /// The starting position of the track (world coordinates)
    pub starting_point: Vec2,
    /// Difficulty metrics
    pub metrics: TrackMetrics,
}

/// Metrics describing track difficulty
#[derive(Debug, Clone)]
pub struct TrackMetrics {
    /// Total number of segments
    pub segment_count: usize,
    /// Number of corners (turns)
    pub corner_count: usize,
    /// Number of left turns
    pub left_turns: usize,
    /// Number of right turns
    pub right_turns: usize,
    /// Turn density: corners / total segments (0.0 to 1.0)
    pub turn_density: f32,
    /// Difficulty score (0.0 = easy, 1.0 = hard)
    pub difficulty: f32,
}

/// Configuration for random track generation
pub struct TrackGeneratorConfig {
    /// Minimum number of segments (must be >= 4 for a valid loop)
    pub min_segments: usize,
    /// Maximum number of segments
    pub max_segments: usize,
    /// Target difficulty (0.0 = easy/few turns, 1.0 = hard/many turns)
    pub target_difficulty: f32,
    /// Seed for reproducible track generation
    pub seed: u64,
}

/// Minimum segments required for a valid closed loop (a square)
pub const MIN_VALID_SEGMENTS: usize = 4;

/// Calculate the maximum number of segments possible on the grid
/// This is the number of cells in the usable grid area (Hamiltonian cycle upper bound)
pub fn max_grid_segments() -> usize {
    let margin = 2; // Same margin used in generation
    let half_width = (WINDOW_WIDTH as f32 / 2.0 / ROAD_SEGMENT_LENGTH) as usize - margin;
    let half_height = (WINDOW_HEIGHT as f32 / 2.0 / ROAD_SEGMENT_LENGTH) as usize - margin;
    // Grid dimensions: from -half to +half inclusive = 2*half + 1
    let grid_width = 2 * half_width + 1;
    let grid_height = 2 * half_height + 1;
    grid_width * grid_height
}

impl Default for TrackGeneratorConfig {
    fn default() -> Self {
        Self {
            min_segments: 30,
            max_segments: 100,
            target_difficulty: 0.5,
            seed: 42,
        }
    }
}

impl TrackGeneratorConfig {
    /// Validate the configuration, panicking with descriptive errors if invalid
    pub fn validate(&self) {
        let max_possible = max_grid_segments();

        if self.min_segments < MIN_VALID_SEGMENTS {
            panic!(
                "Invalid TrackGeneratorConfig: min_segments ({}) must be >= {} (minimum for a closed loop)",
                self.min_segments, MIN_VALID_SEGMENTS
            );
        }

        if self.max_segments < MIN_VALID_SEGMENTS {
            panic!(
                "Invalid TrackGeneratorConfig: max_segments ({}) must be >= {} (minimum for a closed loop)",
                self.max_segments, MIN_VALID_SEGMENTS
            );
        }

        if self.max_segments > max_possible {
            panic!(
                "Invalid TrackGeneratorConfig: max_segments ({}) exceeds maximum possible segments ({}) for the grid size (Hamiltonian cycle upper bound)",
                self.max_segments, max_possible
            );
        }

        if self.min_segments > self.max_segments {
            panic!(
                "Invalid TrackGeneratorConfig: min_segments ({}) cannot be greater than max_segments ({})",
                self.min_segments, self.max_segments
            );
        }

        if self.target_difficulty < 0.0 || self.target_difficulty > 1.0 {
            panic!(
                "Invalid TrackGeneratorConfig: target_difficulty ({}) must be between 0.0 and 1.0",
                self.target_difficulty
            );
        }
    }
}



/// Generate a random track using a self-avoiding walk with backtracking.
///
/// The algorithm:
/// 1. Start at grid origin (0,0) heading Up
/// 2. Force first two segments to be straight (for start/finish line placement)
/// 3. Randomly choose Straight/CornerLeft/CornerRight based on target difficulty
/// 4. Check if the move stays within bounds and doesn't cross existing path
/// 5. Ensure no parallel road segments (prevents shortcut cheating)
/// 6. If we can close the loop back to origin, do so
/// 7. Backtrack if stuck
///
/// # Panics
/// Panics if the config is invalid:
/// - min_segments < 4 (minimum for a closed loop)
/// - max_segments > grid cell count (Hamiltonian cycle upper bound)
/// - min_segments > max_segments
/// - target_difficulty not in [0.0, 1.0]
///
/// Returns None if generation fails after max attempts.
pub fn generate_random_track(config: &TrackGeneratorConfig) -> Option<GeneratedTrack> {
    // Validate config - will panic with descriptive message if invalid
    config.validate();

    let mut rng = ChaCha8Rng::seed_from_u64(config.seed);

    // Grid bounds based on window size (with margin for road width)
    let margin = 2; // Keep 2 segments away from edges
    let half_width = (WINDOW_WIDTH as f32 / 2.0 / ROAD_SEGMENT_LENGTH) as i32 - margin;
    let half_height = (WINDOW_HEIGHT as f32 / 2.0 / ROAD_SEGMENT_LENGTH) as i32 - margin;

    // Try multiple times with fresh starts
    for _attempt in 0..100 {
        if let Some(track) = try_generate_track(&mut rng, config, half_width, half_height) {
            return Some(track);
        }
    }

    None
}

/// Single attempt at generating a track with backtracking
fn try_generate_track<R: Rng>(
    rng: &mut R,
    config: &TrackGeneratorConfig,
    half_width: i32,
    half_height: i32,
) -> Option<GeneratedTrack> {
    let mut layout: Vec<RoadSegmentType> = Vec::new();
    let mut visited: HashSet<IVec2> = HashSet::new();
    let mut path: Vec<(IVec2, Direction)> = Vec::new(); // For backtracking

    let mut current_pos = IVec2::ZERO;
    let mut current_dir = Direction::Up;

    visited.insert(current_pos);
    path.push((current_pos, current_dir));

    // Force first two segments to be straight (required for start/finish line placement)
    for _ in 0..2 {
        let next_pos = get_next_grid_position(current_pos, current_dir);
        layout.push(RoadSegmentType::Straight);
        visited.insert(next_pos);
        current_pos = next_pos;
        // Direction stays the same for straight segments
        path.push((current_pos, current_dir));
    }

    let max_backtracks = 1000;
    let mut backtrack_count = 0;

    while layout.len() < config.max_segments && backtrack_count < max_backtracks {
        // Check if we can close the loop (need minimum segments first)
        if layout.len() >= config.min_segments {
            if let Some(closing_segment) = can_close_loop(current_pos, current_dir, &visited) {
                layout.push(closing_segment);
                return Some(finalize_track(layout));
            }
        }

        // Get valid moves from current position
        let valid_moves = get_valid_moves(current_pos, current_dir, &visited, half_width, half_height);

        if valid_moves.is_empty() {
            // Backtrack (but never remove the first two forced straight segments)
            if layout.len() <= 2 {
                return None; // Can't backtrack into forced segments
            }

            layout.pop();
            path.pop();

            if let Some(&(pos, dir)) = path.last() {
                // Remove the position we're backtracking from
                visited.remove(&current_pos);
                current_pos = pos;
                current_dir = dir;
            }

            backtrack_count += 1;
            continue;
        }

        // Choose a move based on difficulty (higher = more corners)
        let segment = choose_segment(rng, &valid_moves, config.target_difficulty);

        // Apply the move
        let next_pos = get_next_grid_position(current_pos, current_dir);
        let next_dir = get_exit_direction(current_dir, segment);

        layout.push(segment);
        visited.insert(next_pos);
        current_pos = next_pos;
        current_dir = next_dir;
        path.push((current_pos, current_dir));
    }

    None // Failed to close loop within max segments
}

/// Check if we can close the loop from current position back to origin
fn can_close_loop(
    current_pos: IVec2,
    current_dir: Direction,
    visited: &HashSet<IVec2>,
) -> Option<RoadSegmentType> {
    // The next position after moving in current_dir
    let next_pos = get_next_grid_position(current_pos, current_dir);

    // We need next_pos to be the origin
    if next_pos != IVec2::ZERO {
        return None;
    }

    // Check that the closing segment won't create parallel roads
    // For the closing move: next_pos is origin (0,0). We check if origin has adjacent
    // visited cells other than current_pos and the first cell after origin (0,1).
    let first_cell_after_origin = IVec2::new(0, 1); // Track starts going Up from origin
    let neighbors = [
        IVec2::new(0, 1),  // Up
        IVec2::new(0, -1), // Down
        IVec2::new(1, 0),  // Right
        IVec2::new(-1, 0), // Left
    ];

    for neighbor in neighbors {
        if neighbor == current_pos || neighbor == first_cell_after_origin {
            continue;
        }
        if visited.contains(&neighbor) {
            return None; // Would create parallel roads at the closing point
        }
    }

    // We need a segment that, when entered from current_dir, exits as Up
    // (because the track starts heading Up from origin)
    for segment in [
        RoadSegmentType::Straight,
        RoadSegmentType::CornerLeft,
        RoadSegmentType::CornerRight,
    ] {
        if get_exit_direction(current_dir, segment) == Direction::Up {
            return Some(segment);
        }
    }

    None
}

/// Get the next grid position when moving in a direction
fn get_next_grid_position(pos: IVec2, dir: Direction) -> IVec2 {
    match dir {
        Direction::Up => IVec2::new(pos.x, pos.y + 1),
        Direction::Down => IVec2::new(pos.x, pos.y - 1),
        Direction::Left => IVec2::new(pos.x - 1, pos.y),
        Direction::Right => IVec2::new(pos.x + 1, pos.y),
    }
}

/// Get all valid segment choices from current state
fn get_valid_moves(
    current_pos: IVec2,
    current_dir: Direction,
    visited: &HashSet<IVec2>,
    half_width: i32,
    half_height: i32,
) -> Vec<RoadSegmentType> {
    let next_pos = get_next_grid_position(current_pos, current_dir);

    // Check bounds
    if next_pos.x.abs() > half_width || next_pos.y.abs() > half_height {
        return Vec::new();
    }

    // Check if next position is already visited (except origin for closing)
    if visited.contains(&next_pos) && next_pos != IVec2::ZERO {
        return Vec::new();
    }

    // Check if next position would be adjacent to any visited cell (except current_pos)
    // This prevents parallel road segments that players could drive across
    if has_adjacent_visited(&next_pos, current_pos, visited) {
        return Vec::new();
    }

    // All segments are valid if we can move to next_pos
    vec![
        RoadSegmentType::Straight,
        RoadSegmentType::CornerLeft,
        RoadSegmentType::CornerRight,
    ]
}

/// Check if a position has any adjacent visited cells (excluding the previous position)
/// This prevents parallel road segments
fn has_adjacent_visited(pos: &IVec2, previous_pos: IVec2, visited: &HashSet<IVec2>) -> bool {
    let neighbors = [
        IVec2::new(pos.x + 1, pos.y), // Right
        IVec2::new(pos.x - 1, pos.y), // Left
        IVec2::new(pos.x, pos.y + 1), // Up
        IVec2::new(pos.x, pos.y - 1), // Down
    ];

    for neighbor in neighbors {
        // Skip the cell we came from
        if neighbor == previous_pos {
            continue;
        }
        // Skip origin (allowed for closing the loop)
        if neighbor == IVec2::ZERO {
            continue;
        }
        // If any other neighbor is visited, reject this move
        if visited.contains(&neighbor) {
            return true;
        }
    }

    false
}

/// Choose a segment type based on difficulty preference
fn choose_segment<R: Rng>(
    rng: &mut R,
    valid_moves: &[RoadSegmentType],
    target_difficulty: f32,
) -> RoadSegmentType {
    // Higher difficulty = higher chance of corners
    // Difficulty 0.0: 80% straight, 10% left, 10% right
    // Difficulty 0.5: 50% straight, 25% left, 25% right
    // Difficulty 1.0: 20% straight, 40% left, 40% right

    let straight_chance = 0.8 - 0.6 * target_difficulty;
    let roll: f32 = rng.random();

    if roll < straight_chance {
        if valid_moves.contains(&RoadSegmentType::Straight) {
            return RoadSegmentType::Straight;
        }
    }

    // Pick randomly from corners
    let corner_roll: f32 = rng.random();
    if corner_roll < 0.5 {
        if valid_moves.contains(&RoadSegmentType::CornerLeft) {
            return RoadSegmentType::CornerLeft;
        }
        if valid_moves.contains(&RoadSegmentType::CornerRight) {
            return RoadSegmentType::CornerRight;
        }
    } else {
        if valid_moves.contains(&RoadSegmentType::CornerRight) {
            return RoadSegmentType::CornerRight;
        }
        if valid_moves.contains(&RoadSegmentType::CornerLeft) {
            return RoadSegmentType::CornerLeft;
        }
    }

    // Fallback
    valid_moves[rng.random_range(0..valid_moves.len())]
}

/// Finalize the track by computing starting point and metrics
fn finalize_track(layout: Vec<RoadSegmentType>) -> GeneratedTrack {
    // Calculate the bounding box of the track path
    let mut current_pos = IVec2::ZERO;
    let mut current_dir = Direction::Up;
    let mut min_x = 0i32;
    let mut max_x = 0i32;
    let mut min_y = 0i32;
    let mut max_y = 0i32;

    for &segment in &layout {
        current_pos = get_next_grid_position(current_pos, current_dir);
        min_x = min_x.min(current_pos.x);
        max_x = max_x.max(current_pos.x);
        min_y = min_y.min(current_pos.y);
        max_y = max_y.max(current_pos.y);
        current_dir = get_exit_direction(current_dir, segment);
    }

    // Center the track: calculate offset to center the bounding box
    let center_x = (min_x + max_x) as f32 / 2.0;
    let center_y = (min_y + max_y) as f32 / 2.0;

    // Starting point is at grid origin (0,0) minus the centering offset
    // Converted to world coordinates
    let starting_point = Vec2::new(
        -center_x * ROAD_SEGMENT_LENGTH,
        -center_y * ROAD_SEGMENT_LENGTH,
    );

    // Calculate metrics
    let segment_count = layout.len();
    let left_turns = layout
        .iter()
        .filter(|&&s| s == RoadSegmentType::CornerLeft)
        .count();
    let right_turns = layout
        .iter()
        .filter(|&&s| s == RoadSegmentType::CornerRight)
        .count();
    let corner_count = left_turns + right_turns;
    let turn_density = corner_count as f32 / segment_count as f32;

    // Difficulty based on turn density and track length
    // Short tracks with many turns are harder
    let length_factor = 1.0 - (segment_count as f32 / 60.0).min(1.0) * 0.3;
    let difficulty = (turn_density * length_factor).clamp(0.0, 1.0);

    GeneratedTrack {
        layout,
        starting_point,
        metrics: TrackMetrics {
            segment_count,
            corner_count,
            left_turns,
            right_turns,
            turn_density,
            difficulty,
        },
    }
}
