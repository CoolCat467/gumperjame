// use bevy::prelude::*;
// use bevy::window::PrimaryWindow;
// use bevy::app::AppExit;

// pub fn spawn_camera(
//     mut commands: Commands,
//     window_query: Query<&Window, With<PrimaryWindow>>,
// ) {
//     let window: &Window = window_query.get_single().unwrap();

//     commands.spawn(
//         Camera2dBundle{
//             transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
//             ..default()
//         }
//     );
// }
