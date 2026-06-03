use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use bevy::prelude::*;

pub struct ExitOn(pub KeyCode);

pub fn escape() -> ExitOn {
    ExitOn(KeyCode::Escape)
}

impl ExitOn {
    pub fn exit_on(key_code: KeyCode) -> ScheduleConfigs<ScheduleSystem> {
        (move |mut app_exit: MessageWriter<AppExit>, keyboard: Res<ButtonInput<KeyCode>>| {
            if keyboard.just_pressed(key_code) {
                app_exit.write(AppExit::Success);
            }
        })
        .into_configs()
    }
}

impl Plugin for ExitOn {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ExitOn::exit_on(self.0));
    }
}
