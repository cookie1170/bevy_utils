use std::any::type_name;
use std::convert::Infallible;
use std::ops::{Deref, DerefMut};
use std::time::Duration;

use bevy::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct RandomTimer {
    pub min_time: f32,
    pub max_time: f32,
    pub mode: TimerMode,
    timer: Timer,
}

impl RandomTimer {
    pub fn new(min: f32, max: f32, mode: TimerMode) -> Self {
        let timer = Timer::from_seconds(min, mode);
        let mut result = Self {
            min_time: min,
            max_time: max,
            mode,
            timer,
        };
        result.randomize_duration();
        result
    }

    pub fn tick(&mut self, delta: Duration) -> &mut Self {
        if self.timer.tick(delta).just_finished() && self.mode == TimerMode::Repeating {
            self.randomize_duration();
        }

        self
    }

    pub fn reset(&mut self) {
        self.randomize_duration();
        self.timer.reset();
    }

    pub fn randomize_duration(&mut self) {
        let duration = rand::random_range(self.min_time..=self.max_time);

        self.timer.set_duration(Duration::from_secs_f32(duration));
    }
}

impl Deref for RandomTimer {
    type Target = Timer;

    fn deref(&self) -> &Self::Target {
        &self.timer
    }
}

impl DerefMut for RandomTimer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.timer
    }
}

pub trait TransformExt {
    fn from_xy(x: f32, y: f32) -> Self;
}

impl TransformExt for Transform {
    fn from_xy(x: f32, y: f32) -> Self {
        Self::from_xyz(x, y, 0.0)
    }
}

pub trait ExtraResultSeverityExt<T, E> {
    fn context(self, context: impl Into<String>) -> Result<T>;
    fn ignore(self) -> Result<T>;
    fn trace(self) -> Result<T>;
    fn debug(self) -> Result<T>;
    fn info(self) -> Result<T>;
    fn warn(self) -> Result<T>;
    fn error(self) -> Result<T>;
    fn panic(self) -> Result<T>;
}

impl<T, E> ExtraResultSeverityExt<T, E> for Result<T, E>
where
    E: Into<BevyError>,
{
    fn context(self, context: impl Into<String>) -> Result<T> {
        self.map_err(move |e| {
            let e = e.into();
            BevyError::new(e.severity(), format!("{}: {}", context.into(), e))
        })
    }

    fn ignore(self) -> Result<T> {
        self.with_severity(Severity::Ignore)
    }

    fn trace(self) -> Result<T> {
        self.with_severity(Severity::Trace)
    }

    fn debug(self) -> Result<T> {
        self.with_severity(Severity::Debug)
    }

    fn info(self) -> Result<T> {
        self.with_severity(Severity::Info)
    }

    fn warn(self) -> Result<T> {
        self.with_severity(Severity::Warning)
    }

    fn error(self) -> Result<T> {
        self.with_severity(Severity::Error)
    }

    fn panic(self) -> Result<T> {
        self.with_severity(Severity::Panic)
    }
}

fn option_err<T>(val: Option<T>, severity: Severity) -> Result<T> {
    val.ok_or_else(|| {
        BevyError::new(
            severity,
            format!("`Option<{}>` is `None`", type_name::<T>()),
        )
    })
}

impl<T> ExtraResultSeverityExt<T, Infallible> for Option<T> {
    fn context(self, message: impl Into<String>) -> Result<T> {
        self.ok_or_else(|| BevyError::new(Severity::Panic, message.into()))
    }

    fn ignore(self) -> Result<T> {
        option_err(self, Severity::Ignore)
    }

    fn trace(self) -> Result<T> {
        option_err(self, Severity::Trace)
    }

    fn debug(self) -> Result<T> {
        option_err(self, Severity::Debug)
    }

    fn info(self) -> Result<T> {
        option_err(self, Severity::Info)
    }

    fn warn(self) -> Result<T> {
        option_err(self, Severity::Warning)
    }

    fn error(self) -> Result<T> {
        option_err(self, Severity::Error)
    }

    fn panic(self) -> Result<T> {
        option_err(self, Severity::Panic)
    }
}
