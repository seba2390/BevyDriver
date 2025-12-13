use bevy::prelude::*;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RoadSegmentType {
    Straight,
    CornerLeft,
    CornerRight,
}

/// A track definition containing the layout and starting position
pub struct Track {
    /// The sequence of road segments that make up the track
    pub layout: &'static [RoadSegmentType],
    /// The starting position of the track (world coordinates)
    pub starting_point: Vec2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component)]
pub struct RoadSegment {
    pub segment_type: RoadSegmentType,
    pub direction: Direction,
}

/// Marker component indicating a road segment has been visited by the car
#[derive(Component)]
pub struct Visited;

/// Component for the start line entity
/// The direction indicates which way the car must cross to trigger the start
#[derive(Component)]
pub struct StartLine {
    pub direction: Direction,
}

/// Component for the finish line entity
/// The direction indicates which way the car must cross to trigger the finish
#[derive(Component)]
pub struct FinishLine {
    pub direction: Direction,
}
