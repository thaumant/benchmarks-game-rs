//! Changes:
//! 1. Use `Triple` instead of `[f64; 3]`.
//! 2. Replace index access with iterators.

use std::f64::consts::PI;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Triple(f64, f64, f64);

impl Triple {
    #[inline]
    pub fn sum_squared(self) -> f64 {
        self.0 * self.0
            + self.1 * self.1
            + self.2 * self.2
    }

    #[inline]
    pub fn scale(self, n: f64) -> Self {
        Triple(
            self.0 * n,
            self.1 * n,
            self.2 * n
        )
    }
}

impl Add for Triple {
    type Output = Triple;
    #[inline]
    fn add(self, other: Self) -> Self::Output {
        Triple(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2
        )
    }
}

impl Sub for Triple {
    type Output = Triple;
    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Triple(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2
        )
    }
}

/// State of a single body (sun or planet) in the solar system.
#[derive(Clone, Debug)]
pub struct Body {
    position: Triple,
    velocity: Triple,
    mass: f64,
}

/// Number of bodies modeled in the simulation.
pub const BODIES_COUNT: usize = 5;

pub const SOLAR_MASS: f64 = 4. * PI * PI;
pub const DAYS_PER_YEAR: f64 = 365.24;

/// Number of body-body interactions.
pub const INTERACTIONS: usize = BODIES_COUNT * (BODIES_COUNT - 1) / 2;

/// Initial state of the simulation.
pub const STARTING_STATE: [Body; BODIES_COUNT] = [
    // Sun
    Body {
        mass: SOLAR_MASS,
        position: Triple(0., 0., 0.),
        velocity: Triple(0., 0., 0.),
    },
    // Jupiter
    Body {
        position: Triple(
            4.841_431_442_464_72e0,
            -1.160_320_044_027_428_4e0,
            -1.036_220_444_711_231_1e-1,
        ),
        velocity: Triple(
            1.660_076_642_744_037e-3 * DAYS_PER_YEAR,
            7.699_011_184_197_404e-3 * DAYS_PER_YEAR,
            -6.904_600_169_720_63e-5 * DAYS_PER_YEAR,
        ),
        mass: 9.547_919_384_243_266e-4 * SOLAR_MASS,
    },
    // Saturn
    Body {
        position: Triple(
            8.343_366_718_244_58e0,
            4.124_798_564_124_305e0,
            -4.035_234_171_143_214e-1,
        ),
        velocity: Triple(
            -2.767_425_107_268_624e-3 * DAYS_PER_YEAR,
            4.998_528_012_349_172e-3 * DAYS_PER_YEAR,
            2.304_172_975_737_639_3e-5 * DAYS_PER_YEAR,
        ),
        mass: 2.858_859_806_661_308e-4 * SOLAR_MASS,
    },
    // Uranus
    Body {
        position: Triple(
            1.289_436_956_213_913_1e1,
            -1.511_115_140_169_863_1e1,
            -2.233_075_788_926_557_3e-1,
        ),
        velocity: Triple(
            2.964_601_375_647_616e-3 * DAYS_PER_YEAR,
            2.378_471_739_594_809_5e-3 * DAYS_PER_YEAR,
            -2.965_895_685_402_375_6e-5 * DAYS_PER_YEAR,
        ),
        mass: 4.366_244_043_351_563e-5 * SOLAR_MASS,
    },
    // Neptune
    Body {
        position: Triple(
            1.537_969_711_485_091_1e1,
            -2.591_931_460_998_796_4e1,
            1.792_587_729_503_711_8e-1,
        ),
        velocity: Triple(
            2.680_677_724_903_893_2e-3 * DAYS_PER_YEAR,
            1.628_241_700_382_423e-3 * DAYS_PER_YEAR,
            -9.515_922_545_197_159e-5 * DAYS_PER_YEAR,
        ),
        mass: 5.151_389_020_466_114_5e-5 * SOLAR_MASS,
    },
];

/// Steps the simulation forward by one time-step.
pub fn advance(bodies: &mut [Body; BODIES_COUNT]) {
    // Compute point-to-point vectors between each unique pair of bodies.
    // Note: this array is large enough that computing it mutable and returning
    // it from a block, as I did with magnitudes below, generates a memcpy.
    // Sigh. So I'll leave it mutable.
    let mut position_deltas = [Triple(0., 0., 0.); INTERACTIONS];
    {
        let mut k = 0;
        for (i, body1) in bodies.iter().enumerate() {
            for body2 in &bodies[i + 1 ..] {
                position_deltas[k] = body1.position - body2.position;
                k += 1;
            }
        }
    }

    // Compute the `1/d^3` magnitude between each pair of bodies.
    let magnitudes = {
        let mut magnitudes = [0.; INTERACTIONS];
        for (mag, pos_delta) in magnitudes.iter_mut().zip(position_deltas.iter()) {
            let mag_delta = pos_delta.sum_squared();
            *mag = 0.01 / (mag_delta * mag_delta.sqrt());
        }
        magnitudes
    };

    // Apply every other body's gravitation to each body's velocity.
    {
        let mut k = 0;
        for i in 0 .. BODIES_COUNT - 1 {
            let (body1, rest) = bodies[i..].split_first_mut().unwrap();
            for body2 in rest {
                let mag        = magnitudes[k];
                let pos_delta  = position_deltas[k];
                body1.velocity = body1.velocity - pos_delta.scale(body2.mass * mag);
                body2.velocity = body2.velocity + pos_delta.scale(body1.mass * mag);
                k += 1;
            }
        }
    }

    // Update each body's position.
    for body in bodies {
        body.position = body.position + body.velocity.scale(0.01);
    }
}

/// Adjust the Sun's velocity to offset system momentum.
pub fn offset_momentum(bodies: &mut [Body; BODIES_COUNT]) {
    let (sun, planets) = bodies.split_first_mut().unwrap();
    sun.velocity = Triple(0., 0., 0.);
    for planet in planets {
        sun.velocity = sun.velocity - planet.velocity.scale(planet.mass / SOLAR_MASS);
    }
}

/// Print the system energy.
pub fn compute_energy(bodies: &mut [Body; BODIES_COUNT]) -> f64 {
    let mut energy = 0.;
    for (i, body1) in bodies.iter().enumerate() {
        // Add the kinetic energy for each body.
        energy += 0.5
            * body1.mass
            * body1.velocity.sum_squared();
        // Add the potential energy between this body and every other body.
        for body2 in &bodies[i + 1 ..] {
            let pos_delta = body1.position - body2.position;
            energy -= body1.mass * body2.mass / pos_delta.sum_squared().sqrt();
        }
    }

    energy
}
