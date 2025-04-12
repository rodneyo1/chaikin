use nannou::prelude::*;
use std::cell::RefCell;

pub struct ChaikinAlgorithm {
    // Use RefCell for interior mutability
    cached_results: RefCell<Vec<Vec<Point2>>>,
}

impl ChaikinAlgorithm {
    pub fn new() -> Self {
        ChaikinAlgorithm {
            cached_results: RefCell::new(Vec::new()),
        }
    }

    // Get the curve at a particular step
    pub fn get_curve_at_step(&self, original_points: &[Point2], step: usize) -> Vec<Point2> {
        // Handle special cases
        if original_points.is_empty() {
            return Vec::new();
        }

        if original_points.len() <= 2 || step == 0 {
            return original_points.to_vec();
        }

        // Ensure cache is computed up to the requested step
        self.ensure_computed(original_points, step);

        // Return the cached result for the requested step
        let cached = self.cached_results.borrow();
        if step <= cached.len() {
            return cached[step - 1].clone();
        }

        // Fallback to original points if step is out of range
        original_points.to_vec()
    }

    // Compute and cache results up to the specified step
    fn ensure_computed(&self, original_points: &[Point2], step: usize) {
        if step == 0 || original_points.len() <= 2 {
            return;
        }

        let mut cached_results = self.cached_results.borrow_mut();

        // If cache is empty, initialize with the first step
        if cached_results.is_empty() {
            let first_step = self.chaikin_step(original_points);
            cached_results.push(first_step);
        }

        // Compute each step that hasn't been cached yet
        while cached_results.len() < step {
            let next_step = self.chaikin_step(&cached_results.last().unwrap());
            cached_results.push(next_step);
        }
    }

    // Perform one step of Chaikin's algorithm
    fn chaikin_step(&self, points: &[Point2]) -> Vec<Point2> {
        if points.len() <= 2 {
            return points.to_vec();
        }

        let mut result = Vec::new();

        // Process each segment
        for i in 0..points.len() - 1 {
            let p0 = points[i];
            let p1 = points[i + 1];

            // Calculate the 1/4 point (Q)
            let q = p0 * 0.75 + p1 * 0.25;

            // Calculate the 3/4 point (R)
            let r = p0 * 0.25 + p1 * 0.75;

            // Add the points to our result
            if i == 0 {
                // For the first segment, add both the original point and Q
                result.push(p0);
                result.push(q);
            } else {
                // For subsequent segments, just add Q
                result.push(q);
            }

            // Add R for all segments
            result.push(r);

            // For the last segment, add the final point too
            if i == points.len() - 2 {
                result.push(p1);
            }
        }

        result
    }
}
