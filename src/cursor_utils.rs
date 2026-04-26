use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_enhanced_input::prelude::*;

pub(crate) struct CursorUtilsPlugin;

impl Plugin for CursorUtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(lock_cursor).add_observer(unlock_cursor);
    }
}

pub(crate) fn lock_cursor(_: On<Fire<LockCursor>>, mut cursor: Single<&mut CursorOptions>) {
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

pub(crate) fn unlock_cursor(_: On<Fire<UnlockCursor>>, mut cursor: Single<&mut CursorOptions>) {
    cursor.grab_mode = CursorGrabMode::None;
    cursor.visible = true;
}

#[derive(InputAction)]
#[action_output(bool)]
pub struct LockCursor;

#[derive(InputAction)]
#[action_output(bool)]
pub struct UnlockCursor;
