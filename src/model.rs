use crate::chaikin::ChaikinAlgorithm;
use nannou::prelude::*;

/// Represents the current state of the animation.
#[derive(PartialEq, Clone, Copy)]
pub enum AnimationState {
    /// User is drawing control points with the mouse.
    Drawing,

    /// Animation is playing using Chaikin's algorithm.
    Animating,
}

/// The main application state model.
///
/// Holds all data needed for rendering, user interaction, and animation control.
pub struct Model {
    /// ID of the application window.
    pub window_id: WindowId,

    /// List of user-defined control points.
    pub points: Vec<Point2>,

    /// Instance of the Chaikin algorithm used for curve generation.
    pub algorithm: ChaikinAlgorithm,

    /// Current animation state (drawing or animating).
    pub animation_state: AnimationState,

    /// Current step in the animation (range: 0 to 7).
    pub animation_step: usize,

    /// Timer used to control the interval between animation steps.
    pub animation_timer: f32,

    /// Indicates whether the algorithm steps have been precomputed.
    pub steps_computed: bool,
}
