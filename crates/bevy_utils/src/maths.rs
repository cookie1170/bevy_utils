use std::time::Duration;

use bevy::prelude::*;

pub trait FloatMaths {
    /// Moves `self` towards, but not past `target` by `delta`
    fn move_towards(&self, target: Self, delta: f32) -> Self;

    /// Returns -1 if the number is negative, 1 if the number is positive and 0 if the number is 0
    fn sign(&self) -> Self;
}

impl FloatMaths for f32 {
    fn move_towards(&self, target: Self, delta: f32) -> Self {
        let diff = target - self;
        let abs = diff.abs();

        if delta > abs {
            return target;
        }

        self + (diff / abs * delta)
    }

    fn sign(&self) -> Self {
        if self.abs() < f32::EPSILON {
            0.0
        } else {
            self.signum()
        }
    }
}

pub trait MiscMaths {
    /// Smoothly moves a value towards a goal over time
    /// # Parameters
    /// `target`: The target value
    ///
    /// `current_velocity`: The current velocity, should persist between calls
    ///
    /// `smooth_time`: The approximate time it takes for the current value to reach the target value
    /// The lower it is, the faster the current value reaches the target value
    ///
    /// `max_speed`: The maximum speed that shouldn't be exceeded
    ///
    /// `delta_time`: The time passed since the last call
    fn smooth_damp(
        self,
        target: Self,
        smooth_time: f32,
        max_speed: f32,
        delta_time: f32,
        current_velocity: &mut Self,
    ) -> Self;

    /// Calculates a framerate independent exponential decay (see <https://www.youtube.com/watch?v=LSNQuFEDOyQ>)
    /// # Parameters
    /// `target`: The target value
    ///
    /// `decay`: The decay constant (usually around 1 - 25 from slow to fast)
    ///
    /// `delta_time`: The time passed since the last update
    fn exp_decay(self, target: Self, decay: f32, delta_time: f32) -> Self;
}

impl MiscMaths for f32 {
    fn smooth_damp(
        self,
        target: Self,
        smooth_time: f32,
        max_speed: f32,
        delta_time: f32,
        current_velocity: &mut Self,
    ) -> Self {
        // Based on Game Programming Gems 4 Chapter 1.10
        let smooth_time = smooth_time.max(0.0001);
        let omega = 2.0 / smooth_time;

        let x = omega * delta_time;
        let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
        let change = self - target;
        let original_to = target;

        // Clamp maximum speed
        let max_change = max_speed * smooth_time;
        let change = change.clamp(-max_change, max_change);
        let target = self - change;

        let temp = (*current_velocity + change * omega) * delta_time;
        let mut new_velocity = (*current_velocity - temp * omega) * exp;
        let mut output = target + (change + temp) * exp;

        // Prevent overshooting
        if (original_to - self > 0.0) == (output > original_to) {
            output = original_to;
            new_velocity = (output - original_to) * (1.0 / delta_time);
        }

        *current_velocity = new_velocity;
        output
    }

    fn exp_decay(self, target: Self, decay: f32, delta_time: f32) -> Self {
        target + (self - target) * (-decay * delta_time).exp()
    }
}

impl MiscMaths for Vec2 {
    fn smooth_damp(
        self,
        target: Self,
        smooth_time: f32,
        max_speed: f32,
        delta_time: f32,
        current_velocity: &mut Self,
    ) -> Self {
        // Based on Game Programming Gems 4 Chapter 1.10
        let smooth_time = smooth_time.max(0.0001);
        let omega = 2.0 / smooth_time;

        let x = omega * delta_time;
        let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
        let change = self - target;
        let original_to = target;

        // Clamp maximum speed
        let max_change = max_speed * smooth_time;
        let change = change.clamp_length_max(max_change);
        let target = self - change;

        let temp = (*current_velocity + change * omega) * delta_time;
        let mut new_velocity = (*current_velocity - temp * omega) * exp;
        let mut output = target + (change + temp) * exp;

        // Prevent overshooting
        if ((original_to - self).length_squared() > 0.0)
            == (output.length_squared() > original_to.length_squared())
        {
            output = original_to;
            new_velocity = (output - original_to) * (1.0 / delta_time);
        }

        *current_velocity = new_velocity;
        output
    }

    fn exp_decay(self, target: Self, decay: f32, delta_time: f32) -> Self {
        target + (self - target) * (-decay * delta_time).exp()
    }
}

// yes i know there's probably a better way than duplicating this
// but do i care? no.
impl MiscMaths for Vec3 {
    fn smooth_damp(
        self,
        target: Self,
        smooth_time: f32,
        max_speed: f32,
        delta_time: f32,
        current_velocity: &mut Self,
    ) -> Self {
        // Based on Game Programming Gems 4 Chapter 1.10
        let smooth_time = smooth_time.max(0.0001);
        let omega = 2.0 / smooth_time;

        let x = omega * delta_time;
        let exp = 1.0 / (1.0 + x + 0.48 * x * x + 0.235 * x * x * x);
        let change = self - target;
        let original_to = target;

        // Clamp maximum speed
        let max_change = max_speed * smooth_time;
        let change = change.clamp_length_max(max_change);
        let target = self - change;

        let temp = (*current_velocity + change * omega) * delta_time;
        let mut new_velocity = (*current_velocity - temp * omega) * exp;
        let mut output = target + (change + temp) * exp;

        // Prevent overshooting
        if ((original_to - self).length_squared() > 0.0)
            == (output.length_squared() > original_to.length_squared())
        {
            output = original_to;
            new_velocity = (output - original_to) * (1.0 / delta_time);
        }

        *current_velocity = new_velocity;
        output
    }

    fn exp_decay(self, target: Self, decay: f32, delta_time: f32) -> Self {
        target + (self - target) * (-decay * delta_time).exp()
    }
}

// Shamelessly stolen from https://www.ryanjuckett.com/damped-springs/
/// Calculates a damped spring motion and returns the new value
///
/// # Parameters
/// `current`: The current position
///
/// `target`: The equilibrium position
///
/// `velocity`: A mutable reference to the current velocity
///
/// `delta_time`: The time since last update
///
/// `angular_frequency`: How fast the spring oscilates
///
/// `damping_ratio`: How fast the motion decays:
///    - `damping_ratio > 1`: over damped (approaches slower)
///    - `damping_ratio = 1`: critically damped (doesn't overshoot)
///    - `damping_ratio < 1`: under damped (overshoots)
pub fn damped_spring(
    current: f32,
    target: f32,
    velocity: &mut f32,
    delta_time: Duration,
    mut angular_frequency: f32,
    mut damping_ratio: f32,
) -> f32 {
    const EPSILON: f32 = 0.0001;

    // force values into legal range
    if damping_ratio < 0.0 {
        damping_ratio = 0.0
    };
    if angular_frequency < 0.0 {
        angular_frequency = 0.0
    };

    // if there is no angular frequency, the spring will not move and we can
    // return identity
    if angular_frequency < EPSILON {
        return current;
    }

    let delta_time = delta_time.as_secs_f32();

    let (pos_pos_coef, pos_vel_coef, vel_pos_coef, vel_vel_coef) = if damping_ratio > 1.0 + EPSILON
    {
        // over-damped
        let za = -angular_frequency * damping_ratio;
        let zb = angular_frequency * (damping_ratio * damping_ratio - 1.0).sqrt();
        let z1 = za - zb;
        let z2 = za + zb;

        let e1 = (z1 * delta_time).exp();
        let e2 = (z2 * delta_time).exp();

        let inv_two_zb = 1.0 / (2.0 * zb); // = 1 / (z2 - z1)

        let e1_over_two_zb = e1 * inv_two_zb;
        let e2_over_two_zb = e2 * inv_two_zb;

        let z1e1_over_two_zb = z1 * e1_over_two_zb;
        let z2e2_over_two_zb = z2 * e2_over_two_zb;

        let pos_pos_coef = e1_over_two_zb * z2 - z2e2_over_two_zb + e2;
        let pos_vel_coef = -e1_over_two_zb + e2_over_two_zb;

        let vel_pos_coef = (z1e1_over_two_zb - z2e2_over_two_zb + e2) * z2;
        let vel_vel_coef = -z1e1_over_two_zb + z2e2_over_two_zb;
        (pos_pos_coef, pos_vel_coef, vel_pos_coef, vel_vel_coef)
    } else if damping_ratio < 1.0 - EPSILON {
        // under-damped
        let omega_zeta = angular_frequency * damping_ratio;
        let alpha = angular_frequency * (1.0 - damping_ratio * damping_ratio).sqrt();

        let exp_term = (-omega_zeta * delta_time).exp();
        let (sin_term, cos_term) = (alpha * delta_time).sin_cos();

        let inv_alpha = 1.0 / alpha;

        let exp_sin = exp_term * sin_term;
        let exp_cos = exp_term * cos_term;
        let exp_omega_zeta_sin_over_alpha = exp_term * omega_zeta * sin_term * inv_alpha;

        let pos_pos_coef = exp_cos + exp_omega_zeta_sin_over_alpha;
        let pos_vel_coef = exp_sin * inv_alpha;

        let vel_pos_coef = -exp_sin * alpha - omega_zeta * exp_omega_zeta_sin_over_alpha;
        let vel_vel_coef = exp_cos - exp_omega_zeta_sin_over_alpha;

        (pos_pos_coef, pos_vel_coef, vel_pos_coef, vel_vel_coef)
    } else {
        // critically damped
        let exp_term = (-angular_frequency * delta_time).exp();
        let time_exp = delta_time * exp_term;
        let time_exp_freq = time_exp * angular_frequency;

        let pos_pos_coef = time_exp_freq + exp_term;
        let pos_vel_coef = time_exp;

        let vel_pos_coef = -angular_frequency * time_exp_freq;
        let vel_vel_coef = -time_exp_freq + exp_term;

        (pos_pos_coef, pos_vel_coef, vel_pos_coef, vel_vel_coef)
    };

    let old_pos = current - target; // update in equilibrium relative space
    let old_vel = *velocity;

    *velocity = old_pos * vel_pos_coef + old_vel * vel_vel_coef;
    old_pos * pos_pos_coef + old_vel * pos_vel_coef + target
}
