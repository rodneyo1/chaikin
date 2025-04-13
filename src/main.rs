use nannou::prelude::*;
mod chaikin;
mod model;

use chaikin::ChaikinAlgorithm;
use model::{AnimationState, Model};

fn main() {
    nannou::app(model).update(update).run();
}

// Model function to create the initial state of the application(called at start)
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(800, 600)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model { 
        window_id,
        points: Vec::new(),
        algorithm: ChaikinAlgorithm::new(),
        animation_state: AnimationState::Drawing,
        animation_step: 0,
        animation_timer: 0.0,
        steps_computed: false,
    }
}

// Update function to handle animation. This function is called every frame and is responsible for updating the model's state
fn update(_app: &App, model: &mut Model, update: Update) {
    // Only process if we're in animation state
    if model.animation_state == AnimationState::Animating {
        // Update timer
        model.animation_timer += update.since_last.as_secs_f32();

        // Change step every second
        if model.animation_timer >= 1.0 {
            model.animation_timer = 0.0;
            model.animation_step += 1;

            // Restart animation after 7 steps
            if model.animation_step > 7 {
                model.animation_step = 0;
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    // Clear the background
    draw.background().color(BLACK);

    // Draw based on current state
    match model.animation_state {
        AnimationState::Drawing => {
            // Draw the control points
            for point in &model.points {
                draw.ellipse().xy(*point).radius(5.0).color(WHITE);
            }

            // Connect points with lines if more than 1 point
            if model.points.len() > 1 {
                for i in 0..model.points.len() - 1 {
                    draw.line()
                        .start(model.points[i])
                        .end(model.points[i + 1])
                        .weight(1.0)
                        .color(LIGHTGRAY);
                }
            }

            // Display instructions
            draw.text("Click to add points. Press Enter to start animation. Escape to quit.")
                .xy(Point2::new(0.0, -app.window_rect().h() / 2.0 + 20.0))
                .color(WHITE);

            if model.points.is_empty() {
                draw.text("No points added. Please add at least 2 points.")
                    .xy(Point2::new(0.0, app.window_rect().h() / 2.0 - 40.0))
                    .color(YELLOW);
            }
        }
        AnimationState::Animating => {
            // Show original points as small circles
            for point in &model.points {
                draw.ellipse().xy(*point).radius(3.0).color(GRAY);
            }

            // Connect original points with thin gray lines
            if model.points.len() > 1 {
                for i in 0..model.points.len() - 1 {
                    draw.line()
                        .start(model.points[i])
                        .end(model.points[i + 1])
                        .weight(0.5)
                        .color(DARKGRAY);
                }
            }

            // Special case for 1 or 2 points
            if model.points.len() == 1 {
                // Just show the point
                draw.ellipse().xy(model.points[0]).radius(5.0).color(RED);

                draw.text("Only one point - no curve to generate")
                    .xy(Point2::new(0.0, app.window_rect().h() / 2.0 - 40.0))
                    .color(YELLOW);
            } else if model.points.len() == 2 {
                // Draw a line between the two points
                draw.line()
                    .start(model.points[0])
                    .end(model.points[1])
                    .weight(2.0)
                    .color(RED);

                draw.text("Only two points - showing a straight line")
                    .xy(Point2::new(0.0, app.window_rect().h() / 2.0 - 40.0))
                    .color(YELLOW);
            } else {
                // Get curve for current animation step
                let curve = model
                    .algorithm
                    .get_curve_at_step(&model.points, model.animation_step);

                // Draw the current state of the curve
                if curve.len() > 1 {
                    // Draw lines connecting the curve points
                    for i in 0..curve.len() - 1 {
                        draw.line()
                            .start(curve[i])
                            .end(curve[i + 1])
                            .weight(2.0)
                            .color(RED);
                    }

                    // Draw points at curve vertices
                    for point in &curve {
                        draw.ellipse().xy(*point).radius(2.0).color(YELLOW);
                    }
                }
            }

            // Display step information
            draw.text(&format!(
                "Chaikin's Algorithm - Step {}/7",
                model.animation_step
            ))
            .xy(Point2::new(0.0, -app.window_rect().h() / 2.0 + 20.0))
            .color(WHITE);

            draw.text("Press Escape to quit. Press C to clear and draw new points.")
                .xy(Point2::new(0.0, -app.window_rect().h() / 2.0 + 40.0))
                .color(WHITE);
        }
    }

    draw.to_frame(app, &frame).unwrap(); // Render the frame
}

// Handle mouse presses: Add points to the model
fn mouse_pressed(app: &App, model: &mut Model, button: MouseButton) {
    if app.window_id() == model.window_id // Ensure we're in the correct window
        && button == MouseButton::Left
        && model.animation_state == AnimationState::Drawing
    {
        let position = app.mouse.position();
        model.points.push(position);
    }
}

// Handle key presses: Escape to quit, Enter to start animation, C to clear points
fn key_pressed(app: &App, model: &mut Model, key: Key) {
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
