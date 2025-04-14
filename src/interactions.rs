use nannou::prelude::*;
use crate::model::{Model, AnimationState};
use crate::chaikin::ChaikinAlgorithm;

// Handle mouse presses: Add points to the model
pub fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if app.window_id() != model.window_id {
        return;
    }
    
    let position = app.mouse.position();
    model.mouse_position = position;
    
    if button == MouseButton::Left {
        // In drawing state, either add new point or start dragging existing point
        if model.animation_state == AnimationState::Drawing {
            // Check if we're clicking near an existing point
            for (i, point) in model.points.iter().enumerate() {
                let distance = position.distance(*point);
                if distance < 10.0 {  // 10 pixel threshold for selection
                    // Start dragging this point
                    model.dragging_point_index = Some(i);
                    return;
                }
            }
            
            // If not dragging, add a new point
            model.points.push(position);
        } 
        // In animation state, allow dragging points too
        else if model.animation_state == AnimationState::Animating {
            // Check if we're clicking near an existing point
            for (i, point) in model.points.iter().enumerate() {
                let distance = position.distance(*point);
                if distance < 10.0 {  // 10 pixel threshold for selection
                    // Start dragging this point
                    model.dragging_point_index = Some(i);
                    return;
                }
            }
        }
    }
}

// Handle when a point is released
pub fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.dragging_point_index = None;
}

// Handle dragging points
pub fn mouse_moved(app: &App, model: &mut Model, _event: Vec2) {
    // Update current mouse position
    model.mouse_position = app.mouse.position();
    
    // If we're dragging a point, update its position
    if let Some(index) = model.dragging_point_index {
        if index < model.points.len() {
            model.points[index] = model.mouse_position;
            
            // If we're in animation mode, recompute the steps
            if model.animation_state == AnimationState::Animating && model.points.len() > 2 {
                // Reset animation to step 0
                model.animation_step = 0;
                model.animation_timer = 0.0;
                
                // Create new algorithm instance to clear cache
                model.algorithm = ChaikinAlgorithm::new();
                
                // Force computation of all steps
                for step in 1..=7 {
                    model.algorithm.get_curve_at_step(&model.points, step);
                }
                model.steps_computed = true;
            }
        }
    }
}

// Handle key presses: Escape to quit, Enter to start animation, C to clear points
pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Escape => {
            app.quit();
        }
        Key::Return => {
            if model.points.len() >= 1 && model.animation_state == AnimationState::Drawing {
                model.animation_state = AnimationState::Animating;
                model.animation_step = 0;
                model.animation_timer = 0.0;

                // Pre-compute all steps for the animation
                if model.points.len() > 2 {
                    // Force computation of all steps
                    for step in 1..=7 {
                        model.algorithm.get_curve_at_step(&model.points, step);
                    }
                }
                model.steps_computed = true;
            }
        }
        Key::C => {
            // Clear all points and reset to drawing state
            model.points.clear();
            model.animation_state = AnimationState::Drawing;
            model.animation_step = 0;
            model.animation_timer = 0.0;
            // Create new algorithm instance to clear cache
            model.algorithm = ChaikinAlgorithm::new();
            model.steps_computed = false;
        }
        _ => {}
    }
}