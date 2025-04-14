use crate::chaikin::ChaikinAlgorithm;
use nannou::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum AnimationState {
    Drawing,   // User is drawing control points
    Animating, // Animation is playing
}

pub struct Model {
    pub window_id: WindowId,
    pub points: Vec<Point2>,
    pub algorithm: ChaikinAlgorithm,
    pub animation_state: AnimationState,
    pub animation_step: usize,
    pub animation_timer: f32,
    pub steps_computed: bool, // Flag to track if we've precomputed the steps
    pub dragging_point_index: Option<usize>, // Index of point being dragged, if any
    pub mouse_position: Point2, // Current mouse position
}