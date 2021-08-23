//! Entry point when starting the program.




//=============================================================================
// Crates


use bevy::prelude::*;




//=============================================================================
// Components


struct Person;


struct Name( String );




//=============================================================================
// Plugins


pub struct HelloPlugin;

impl Plugin for HelloPlugin {
	fn build( &self, app: &mut AppBuilder ) {
		app.insert_resource( GreetTimer( Timer::from_seconds( 2.0, true ) ) )
			.add_startup_system( add_people.system() )
			.add_startup_system( setup.system() )
			.add_system( greet_people.system() )
			.add_system( animate.system() );
	}
}




//=============================================================================
// Resources


struct GreetTimer( Timer );




//=============================================================================
// Systems


fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	// Load sprite
	let texture_handle = asset_server.load( "Processor.png" );
	commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
	commands.spawn_bundle( SpriteBundle {
		material: materials.add( texture_handle.into() ),
		..Default::default()
	} );

	// 2D Text
	commands.spawn_bundle( Text2dBundle {
		text: Text::with_section(
			"Simple text message.",
			TextStyle {
				font: asset_server.load( "fonts/Orbitron/Orbitron-Regular.ttf" ),
				font_size: 60.0,
				color: Color::WHITE,
			},
			TextAlignment {
				vertical: VerticalAlign::Center,
				horizontal: HorizontalAlign::Center,
			},
		),
		..Default::default()
	} );
}


fn add_people( mut commands: Commands ) {
	commands.spawn().insert( Person ).insert( Name( "Elaina Proctor".to_string() ) );
	commands.spawn().insert( Person ).insert( Name( "Renzo Hume".to_string() ) );
	commands.spawn().insert( Person ).insert( Name( "Zayna Nieves".to_string() ) );
}


fn greet_people(
	time: Res<Time>,
	mut timer: ResMut<GreetTimer>,
	query: Query<&Name, With<Person>>
) {
	// update our timer with the time elapsed since the last update
	// if that caused the timer to finish, we say hello to everyone
	if timer.0.tick(time.delta()).just_finished() {
		for name in query.iter() {
			println!( "Hello {}!", name.0 );
		}
	}
}


fn animate(time: Res<Time>, mut query: Query<&mut Transform, With<Text>>) {
	// Moving the text slowly in a circle.
	for mut transform in query.iter_mut() {
		transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32;
		transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
	}
}




//=============================================================================
// Main


fn main() {
	App::build()
		.add_plugins( DefaultPlugins )
		.add_plugin( HelloPlugin )
		.run();
}
