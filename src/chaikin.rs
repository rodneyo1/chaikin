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
        // Assert valid inputs
        assert!(step < 100, "Step count too high: {}. Maximum allowed is 99", step);
        
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
        assert!(!cached.is_empty(), "Cache should not be empty after ensure_computed call");
        
        if step <= cached.len() {
            return cached[step - 1].clone();
        }

        // Fallback to original points if step is out of range
        original_points.to_vec()
    }

    // Compute and cache results up to the specified step
    fn ensure_computed(&self, original_points: &[Point2], step: usize) {
        assert!(!original_points.is_empty(), "Cannot compute Chaikin's algorithm on empty points");
        
        if step == 0 || original_points.len() <= 2 {
            return;
        }

        let mut cached_results = self.cached_results.borrow_mut();

        // If cache is empty, initialize with the first step
        if cached_results.is_empty() {
            let first_step = self.chaikin_step(original_points);
            assert!(!first_step.is_empty(), "First Chaikin step should not produce empty result");
            cached_results.push(first_step);
        }

        // Compute each step that hasn't been cached yet
        while cached_results.len() < step {
            assert!(!cached_results.is_empty(), "Cached results should not be empty during computation");
            let next_step = self.chaikin_step(&cached_results.last().unwrap());
            assert!(!next_step.is_empty(), "Chaikin step should not produce empty result");
            assert!(
                next_step.len() >= cached_results.last().unwrap().len(), 
                "Chaikin step should produce more or equal number of points"
            );
            cached_results.push(next_step);
        }
    }

    // Perform one step of Chaikin's algorithm
    fn chaikin_step(&self, points: &[Point2]) -> Vec<Point2> {
        assert!(!points.is_empty(), "Cannot perform Chaikin step on empty points");
        
        if points.len() <= 2 {
            return points.to_vec();
        }

        let mut result = Vec::with_capacity(points.len() * 2);

        // Process each segment
        for i in 0..points.len() - 1 {
            let p0 = points[i];
            let p1 = points[i + 1];

            // Assert points are valid (not NaN or infinite)
            assert!(p0.x.is_finite(), "Point coordinates must be finite");
            assert!(p0.y.is_finite(), "Point coordinates must be finite");
            assert!(p1.x.is_finite(), "Point coordinates must be finite");
            assert!(p1.y.is_finite(), "Point coordinates must be finite");

            // Calculate the 1/4 point (Q)
            let q = p0 * 0.75 + p1 * 0.25;

            // Calculate the 3/4 point (R)
            let r = p0 * 0.25 + p1 * 0.75;

            // Verify calculated points are valid
            assert!(q.x.is_finite() && q.y.is_finite(), "Calculated point Q is invalid");
            assert!(r.x.is_finite() && r.y.is_finite(), "Calculated point R is invalid");

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

        // Final verification
        assert!(!result.is_empty(), "Chaikin algorithm produced empty result");
        assert!(
            result.len() >= points.len(), 
            "Chaikin algorithm should produce more or equal points than input"
        );

        result
    }
}