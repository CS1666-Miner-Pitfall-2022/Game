use bevy::{
	prelude::*,
	window::PresentMode,
};

#[derive(Component, Deref, DerefMut)]
struct PopupTimer(Timer);

fn main() {
    App::new()
		.insert_resource(WindowDescriptor {
			title: String::from("Miner Pitfall!"),
			width: 1080.,
			height: 720.,
			present_mode: PresentMode::Fifo,
			..default()
		})
		.add_plugins(DefaultPlugins)
		.add_startup_system(setup)
		.add_system(show_popup)
		.run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
	commands.spawn_bundle(OrthographicCameraBundle::new_2d());
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("monke.png"),
			..default()
		});
	commands
		.spawn_bundle(SpriteBundle {
			texture: asset_server.load("justinCredits.png"),
			transform: Transform::from_xyz(0., 0., -1.),
			..default()
		})
		.insert(PopupTimer(Timer::from_seconds(3., false)));

	info!("Hello world!");
}

fn show_popup(
	time: Res<Time>,
	mut popup: Query<(&mut PopupTimer, &mut Transform)>
) {
	for (mut timer, mut transform) in popup.iter_mut() {
		timer.tick(time.delta());
		if timer.just_finished() {
			transform.translation.z = 2.;
			info!("Actually is Linux!");
		}
	}
}
