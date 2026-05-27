//! 9-point pedal curve that is used to map raw input to calibrated and configured output.
//!
//! Pedals and handbrake share the same curve design and feature report layout.
//! Both input and output use the 0-4096 range.
//! Point layout and other documentation can be found from `docs/pedals.md`.

/// Max logical value for both x and y axis.
pub const MAX_VALUE: u16 = 4096;

/// Number of control points.
pub const POINT_COUNT: usize = 9;

/// Number of configurable points (indices 2-6).
const MIDDLE_COUNT: usize = 5;

/// Numeber of evenly spaced intervals between points 1 and 7.
const INTERVALS: u16 = 6;

/// Default output values for the 5 middle points (linear curve).
const LINEAR_Y: [u16; MIDDLE_COUNT] = [682, 1365, 2048, 2730, 3413];

/// A single control point on the curve.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

/// The 9-point curve.
///
/// `points[0].x == points[1].x` at creation, point 1 increases with bottom deadzone.
/// `points[7].x == points[8].x` at creation, point 7 decreases with top deadzone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Curve {
    points: [Point; POINT_COUNT],
}

impl Default for Curve {
    /// Use linear curve as default curve type.
    fn default() -> Self {
        Self::linear()
    }
}

impl Curve {
    /// Creates a default linear curve without calibration or deadzone offsets.
    pub fn linear() -> Self {
        Self::with_calibration(0, MAX_VALUE)
    }

    /// Creates a linear curve with calibrated bottom and top.
    /// This is the state after calibration before any deadzone or curve changes.
    pub fn with_calibration(bottom: u16, top: u16) -> Self {
        let mut curve = Self {
            points: [Point { x: 0, y: 0 }; POINT_COUNT],
        };

        curve.set_extremes(bottom, top);
        curve.set_middle_y(&LINEAR_Y);
        curve
    }

    /// Returns the control points (read only).
    pub fn points(&self) -> &[Point; POINT_COUNT] {
        &self.points
    }

    /// Returns the calibrated bottom value (no deadzone offset)
    pub fn bottom(&self) -> u16 {
        self.points[0].x
    }

    /// Returns the calibrated top value (no deadzone offset)
    pub fn top(&self) -> u16 {
        self.points[8].x
    }

    /// Returns the current bottom deadzone as a percentage (0-100)
    pub fn bottom_dz_percentage(&self) -> u8 {
        let range: u32 = self.calibrated_range().into();

        if range == 0 {
            return 100;
        }

        let dz: u32 = (self.points[1].x - self.points[0].x).into();
        (dz * 100 / range).min(100) as u8
    }

    /// Returns the current top deadzone as a percentage (0-100)
    pub fn top_dz_percentage(&self) -> u8 {
        let range: u32 = self.calibrated_range().into();

        if range == 0 {
            return 0;
        }

        let dz: u32 = (self.points[8].x - self.points[7].x).into();
        (100 - dz * 100 / range).min(100) as u8
    }

    /// Sets the calibrated top and bottom, resets deadzones and recomputes middle x-values.
    pub fn set_extremes(&mut self, bottom: u16, top: u16) {
        let bottom = bottom.min(MAX_VALUE);
        let top = top.min(MAX_VALUE).max(bottom);

        // Points 0 and 1 share the bottom
        self.points[0] = Point { x: bottom, y: 0 };
        self.points[1] = Point { x: bottom, y: 0 };

        // Points 7 and 8 share the top
        self.points[7] = Point {
            x: top,
            y: MAX_VALUE,
        };

        self.points[8] = Point {
            x: top,
            y: MAX_VALUE,
        };

        self.recompute_middle_x();
    }

    /// Sets the bottom deadzone as a percentage (0-100).
    /// Recomputes middle values for x-axis to maintain even spacing.
    pub fn set_bottom_dz(&mut self, dz: u8) {
        let dz = dz.min(100);
        let range: u32 = self.calibrated_range().into();
        let offset: u16 = ((range * dz as u32) / 100) as u16;

        self.points[1].x = (self.points[0].x + offset).min(self.points[7].x);
        self.recompute_middle_x();
    }

    /// Sets tho top deadzone as a percentage (0-100).
    /// Recomputes middle values for x-axis to maintain even spacing.
    pub fn set_top_dz(&mut self, dz: u8) {
        let dz = (dz as u16).min(100);
        let range: u32 = self.calibrated_range().into();
        let offset: u16 = ((range * dz as u32) / 100) as u16;

        self.points[7].x = (self.points[8].x - offset).max(self.points[1].x);
        self.recompute_middle_x();
    }

    /// Sets the y-values of the 5 middle points (2-6).
    /// Values are clamped to 0-4096 and forced to be non-decreasing.
    pub fn set_middle_y(&mut self, y_values: &[u16; MIDDLE_COUNT]) {
        let mut prev = 0u16;

        for (i, &y) in y_values.iter().enumerate() {
            let y = y.min(MAX_VALUE).max(prev);
            self.points[i + 2].y = y;
            prev = y;
        }
    }

    /// Interpolates the output for a given raw input value.
    /// This is the same function the microcontroller applies.
    pub fn interpolate(&self, input: u16) -> u16 {
        let input = input.min(MAX_VALUE);

        // Below the first point -> output 0 (x is in the bottom deadzone)
        if input <= self.points[1].x {
            return 0;
        }

        // Above the last point -> output max value (x is in the top deadzone)
        if input >= self.points[7].x {
            return MAX_VALUE;
        }

        // Linearly interpolate
        for i in 2..=7 {
            if input > self.points[i].x {
                continue;
            }

            let p0 = &self.points[i - 1];
            let p1 = &self.points[i];
            let dx = (p1.x - p0.x) as u32;

            if dx == 0 {
                return p1.y;
            }

            let t = (input - p0.x) as u32;
            let dy = (p1.y - p0.y) as u32;

            return (p0.y as u32 + (t * dy) / dx).min(MAX_VALUE as u32) as u16;
        }

        MAX_VALUE
    }

    /// Serializes the curve inte the 32 x u16 feature report payload.
    /// `data_id` is the channelf identifier (e.g. 0xF101 for clutch).
    pub fn to_report(&self, data_id: u16) -> [u16; 32] {
        let mut out = [0u16; 32];
        out[0] = data_id;

        for (i, p) in self.points.iter().enumerate() {
            out[1 + i * 2] = p.x;
            out[2 + i * 2] = p.y;
        }

        out
    }

    /// Deserializes a curve from the 32 × u16 feature report payload.
    /// Returns `None` if the payload is shorter than 19 entries
    /// (data_id + 9 x,y pairs).
    pub fn from_report(data: &[u16]) -> Option<Self> {
        if data.len() < 19 {
            return None;
        }

        let mut points = [Point { x: 0, y: 0 }; POINT_COUNT];

        for i in 0..POINT_COUNT {
            points[i] = Point {
                x: data[1 + i * 2],
                y: data[2 + i * 2],
            };
        }

        Some(Self { points })
    }

    /// The distance on x-axis from point 0 to 8 (calibrated range)
    fn calibrated_range(&self) -> u16 {
        self.points[8].x - self.points[0].x
    }

    /// Recomputes the x-axis values of middle points (2-6) so they are evenly spaced between
    /// points 1 and 7.
    fn recompute_middle_x(&mut self) {
        let start = self.points[1].x as u32;
        let end = self.points[7].x as u32;

        // Handle edge case
        if end <= start {
            for i in 0..MIDDLE_COUNT {
                self.points[i + 2].x = self.points[1].x;
            }

            return;
        }

        let span = end - start;
        for i in 0..MIDDLE_COUNT {
            let x = start + (span * (i as u32 + 1)) / INTERVALS as u32;
            self.points[i + 2].x = x.min(MAX_VALUE as u32) as u16;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn linear_curve_is_correct() {
        let c = Curve::linear();

        assert_eq!(c.interpolate(0), 0);
        assert_eq!(c.interpolate(2048), 2048);
        assert_eq!(c.interpolate(4096), 4096);
    }

    #[test]
    fn deadzone_clamps_bottom() {
        let mut c = Curve::linear();
        c.set_bottom_dz(10);
        let boundary = c.points()[1].x;

        assert_eq!(c.interpolate(0), 0);
        assert!(c.interpolate(boundary) <= 1);
        assert!(c.interpolate(2048) > 0);
    }

    #[test]
    fn deadzone_clamps_top() {
        let mut c = Curve::linear();
        c.set_top_dz(90);
        let top = c.points()[7].x;

        assert_eq!(c.interpolate(4096), MAX_VALUE);
        assert_eq!(c.interpolate(top), MAX_VALUE);
    }

    #[test]
    fn calibration_sets_bottom_and_top() {
        let c = Curve::with_calibration(100, 800);

        assert_eq!(c.bottom(), 100);
        assert_eq!(c.top(), 800);
        assert_eq!(c.points()[0].x, 100);
        assert_eq!(c.points()[8].x, 800);
    }

    #[test]
    fn middle_points_evenly_spaced() {
        let c = Curve::linear();
        let p = c.points();
        let start = p[1].x as u32;
        let span = (p[7].x - p[1].x) as u32;

        for i in 0..5 {
            let expected = start + (span * (i as u32 + 1)) / 6;
            assert_eq!(p[i + 2].x, expected as u16);
        }
    }

    #[test]
    fn identical_report() {
        let c = Curve::with_calibration(113, 803);
        let report = c.to_report(0xF102);
        assert_eq!(report[0], 0xF102);

        let decoded = Curve::from_report(&report).unwrap();
        assert_eq!(c, decoded);
    }

    #[test]
    fn set_middle_y_enforces_monotonic() {
        let mut c = Curve::linear();
        c.set_middle_y(&[3000, 2000, 1000, 500, 100]);
        let p = c.points();

        for i in 3..=6 {
            assert!(p[i].y >= p[i - 1].y);
        }
    }

    #[test]
    fn brake_real_data() {
        // Actual brake feature report captured while reverse-engineering the HID protocol
        let data: [u16; 32] = [
            61698, 113, 0, 140, 0, 251, 682, 361, 1365, 471, 2048, 582, 2730, 692, 3413, 803, 4096,
            803, 4096, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        let c = Curve::from_report(&data).unwrap();

        assert_eq!(c.bottom(), 113);
        assert_eq!(c.top(), 803);
        assert_eq!(c.interpolate(113), 0);
        assert_eq!(c.interpolate(803), MAX_VALUE);
    }
}
