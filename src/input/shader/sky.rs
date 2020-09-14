//! Sky setup.

use arctk::{access, clone, math::Pos3};

/// Sky properties.
pub struct Sky {
    /// Sky brightness fraction.
    brightness: f64,
    /// Sun position when calculating sun shadows [m].
    sun_pos: Pos3,
    /// Sun angular radius when calculating soft shadows [rad].
    sun_rad: f64,
}

impl Sky {
    clone!(brightness, f64);
    access!(sun_pos, Pos3);
    clone!(sun_rad, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(brightness: f64, sun_pos: Pos3, sun_rad: f64) -> Self {
        debug_assert!(brightness >= 0.0);
        debug_assert!(brightness >= 0.0);
        debug_assert!(sun_rad >= 0.0);

        Self {
            brightness,
            sun_pos,
            sun_rad,
        }
    }
}
