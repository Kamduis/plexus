//! Entry point when starting the program.




//=============================================================================
// Crates


use chrono::Duration;
use chrono::naive::NaiveDateTime;
use bevy::prelude::*;
use bevy::core::FixedTimestep;

mod schedule;
use schedule::{Clock, ComputerSchedule};

mod computer;
use computer::{Consumer, ComputerMaterials, StatusBar, Usage, Cpu};




//=============================================================================
// Constants


const TIMESTAMP_START: i64 = 2481201120;




//=============================================================================
// Components


struct ClockWidget;


struct SpeedButton {
	multiplier: f32,
}




//=============================================================================
// Resources


struct UiMaterials {
	normal: Handle<ColorMaterial>,
	hovered: Handle<ColorMaterial>,
	pressed: Handle<ColorMaterial>,
}

impl FromWorld for UiMaterials {
	fn from_world( world: &mut World ) -> Self {
		let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
		UiMaterials {
			normal: materials.add( Color::rgb( 0.0, 0.4, 0.0 ).into() ),
			hovered: materials.add( Color::rgb( 0.0, 0.45, 0.0 ).into() ),
			pressed: materials.add( Color::rgb( 0.0, 0.6, 0.0 ).into() ),
		}
	}
}




//=============================================================================
// Plugins


pub struct ComputerPlugin;

impl Plugin for ComputerPlugin {
	fn build( &self, app: &mut AppBuilder ) {
		app.init_resource::<UiMaterials>()
			.init_resource::<ComputerMaterials>()
			.add_startup_system( setup.system() )
			.add_startup_system( spawn_ui.system() )
			.add_startup_system( computer::spawn_cpu.system() )
			.add_system( bevy::input::system::exit_on_esc_system.system() )
			.add_system( update_clock.system() )
			.add_system_set(
				SystemSet::new()
					.with_run_criteria( FixedTimestep::step( 1.0 ) )
					.with_system( jitter_cpu_load.system() ),
			)
			.add_system_set(
				SystemSet::new()
					.with_run_criteria( FixedTimestep::step( 0.01 ) )
					.with_system( refresh_cpu_load.system() ),
			)
			.add_system( observe_button.system() )
			.add_system( update_computer_usage.system() )
			.add_system( animate.system() );
	}
}




//=============================================================================
// Systems


fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	// Cameras
	commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
	commands.spawn_bundle( UiCameraBundle::default() );

	// Load sprite
	let texture_handle = asset_server.load( "Processor.png" );
	commands.spawn_bundle( OrthographicCameraBundle::new_2d() );
	commands.spawn_bundle( SpriteBundle {
		material: materials.add( texture_handle.into() ),
		..Default::default()
	} );

	// Implement clock that tracks and controls the in-game time flow.
	commands.spawn_bundle( (
		Clock {
			datetime: NaiveDateTime::from_timestamp( TIMESTAMP_START, 0 ),
			speed: 1.0,
		},
	) );

	// Implement Computer usage schedule.
	commands.spawn_bundle( (
		ComputerSchedule::from_template( "Family" ),
	) );

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


fn spawn_ui(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	materials: Res<UiMaterials>,
) {
	// Clock
	commands
		.spawn_bundle( TextBundle {
			style: Style {
				size: Size::new( Val::Px( 240.0 ), Val::Px( 10.0 ) ),
				align_self: AlignSelf::FlexEnd,
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px( 15.0 ),
					right: Val::Px( 15.0 ),
					..Default::default()
				},
				..Default::default()
			},
			text: Text::with_section(
				"YYYY-MM-DD hh:mm:ss.µµµ",
				TextStyle {
					font: asset_server.load( "fonts/Orbitron/Orbitron-Regular.ttf" ),
					font_size: 20.0,
					color: Color::WHITE,
				},
				TextAlignment {
					horizontal: HorizontalAlign::Left,
					..Default::default()
				},
			),
			..Default::default()
		} )
		.insert( ClockWidget );

	// Buttons to control the in-game time.
	commands
		.spawn_bundle( ButtonBundle {
			style: Style {
				size: Size::new( Val::Px( 50.0 ), Val::Px( 50.0 ) ),
				// The button is centerd
				margin: Rect::all( Val::Auto ),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px( 40.0 ),
					right: Val::Px( 180.0 ),
					..Default::default()
				},
				..Default::default()
			},
			material: materials.normal.clone(),
			..Default::default()
		} )
		.insert( SpeedButton {
			multiplier: 1.0,
		} )
		.with_children( |parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::with_section(
					"×1",
					TextStyle {
						font: asset_server.load( "fonts/Orbitron/Orbitron-Regular.ttf" ),
						font_size: 20.0,
						color: Color::rgb( 0.9, 0.9, 0.9 ),
					},
					Default::default(),
				),
				..Default::default()
			} );
		} );
	commands
		.spawn_bundle( ButtonBundle {
			style: Style {
				size: Size::new( Val::Px( 50.0 ), Val::Px( 50.0 ) ),
				// The button is centerd
				margin: Rect::all( Val::Auto ),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px( 40.0 ),
					right: Val::Px( 125.0 ),
					..Default::default()
				},
				..Default::default()
			},
			material: materials.normal.clone(),
			..Default::default()
		} )
		.insert( SpeedButton {
			multiplier: 16.0,
		} )
		.with_children( |parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::with_section(
					"×16",
					TextStyle {
						font: asset_server.load( "fonts/Orbitron/Orbitron-Regular.ttf" ),
						font_size: 20.0,
						color: Color::rgb( 0.9, 0.9, 0.9 ),
					},
					Default::default(),
				),
				..Default::default()
			} );
		} );
	commands
		.spawn_bundle( ButtonBundle {
			style: Style {
				size: Size::new( Val::Px( 50.0 ), Val::Px( 50.0 ) ),
				// The button is centerd
				margin: Rect::all( Val::Auto ),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px( 40.0 ),
					right: Val::Px( 70.0 ),
					..Default::default()
				},
				..Default::default()
			},
			material: materials.normal.clone(),
			..Default::default()
		} )
		.insert( SpeedButton {
			multiplier: 128.0,
		} )
		.with_children( |parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::with_section(
					"×128",
					TextStyle {
						font: asset_server.load( "fonts/Orbitron/Orbitron-Regular.ttf" ),
						font_size: 20.0,
						color: Color::rgb( 0.9, 0.9, 0.9 ),
					},
					Default::default(),
				),
				..Default::default()
			} );
		} );
	commands
		.spawn_bundle( ButtonBundle {
			style: Style {
				size: Size::new( Val::Px( 50.0 ), Val::Px( 50.0 ) ),
				// The button is centerd
				margin: Rect::all( Val::Auto ),
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px( 40.0 ),
					right: Val::Px( 15.0 ),
					..Default::default()
				},
				..Default::default()
			},
			material: materials.normal.clone(),
			..Default::default()
		} )
		.insert( SpeedButton {
			multiplier: 1024.0,
		} )
		.with_children( |parent| {
			parent.spawn_bundle(TextBundle {
				text: Text::with_section(
					"×1024",
					TextStyle {
						font: asset_server.load( "fonts/Orbitron/Orbitron-Regular.ttf" ),
						font_size: 20.0,
						color: Color::rgb( 0.9, 0.9, 0.9 ),
					},
					Default::default(),
				),
				..Default::default()
			} );
		} );
}


fn animate(time: Res<Time>, mut query: Query<&mut Transform, With<Text>>) {
	// Moving the text slowly in a circle.
	for mut transform in query.iter_mut() {
		transform.translation.x = 100.0 * time.seconds_since_startup().sin() as f32;
		transform.translation.y = 100.0 * time.seconds_since_startup().cos() as f32;
	}
}


fn update_clock(
	time: Res<Time>,
	mut query: Query<&mut Text, With<ClockWidget>>,
	mut clock_query: Query<&mut Clock>,
) {
	let mut clock = clock_query.single_mut().unwrap();
	let mut text = query.single_mut().unwrap();

	// Advance in-game time by the real time since the last frame but with the in-game multiplier.
	let time_step_msecs = time.delta_seconds() * clock.speed * 1_000_000.0;
	clock.advance( Duration::microseconds( time_step_msecs.floor() as i64 ) );

	// Write the current in-gane date and time to the game clock widget.
	text.sections[0].value = clock.datetime.format( "%Y-%m-%d %H:%M:%S%.3f" ).to_string();
}


fn observe_button(
	materials: Res<UiMaterials>,
	mut interaction_query: Query<( &SpeedButton ,&Interaction, &mut Handle<ColorMaterial> ), ( Changed<Interaction>, With<Button> )>,
	mut clock_query: Query<&mut Clock>,
) {
	let mut clock = clock_query.single_mut().unwrap();
	for ( button, interaction, mut material ) in interaction_query.iter_mut() {
		match *interaction {
			Interaction::Clicked => {
				*material = materials.pressed.clone();
				clock.speed = button.multiplier;
			}
			Interaction::Hovered => {
				*material = materials.hovered.clone();
			}
			Interaction::None => {
				*material = materials.normal.clone();
			}
		}
	}
}


fn update_computer_usage(
	mut query: Query<&mut Usage, With<StatusBar>>,
	clock_query: Query<&Clock>,
	schedule_query: Query<&ComputerSchedule>
) {
	let clock = clock_query.single().unwrap();
	let schedule = schedule_query.single().unwrap();
	for mut usage in query.iter_mut() {
		match &usage.consumer {
			Consumer::User => usage.load = schedule.load( clock.datetime.time() ),
			_ => (),
		};
	}
}


fn jitter_cpu_load( mut query: Query<&mut Usage, With<Cpu>> ) {
	for mut usage in query.iter_mut() {
		usage.jitter = usage.load + 0.2 * rand::random::<f32>() - 0.1;
	}
}


fn refresh_cpu_load( mut query: Query<( &Usage, &mut Transform )> ) {
	let step = 0.01;
	for ( usage, mut transform ) in query.iter_mut() {
		let scale_target = usage.load + usage.jitter;
		if transform.scale.y > scale_target {
			transform.scale.y -= step;
		} else if transform.scale.y < scale_target {
			transform.scale.y += step;
		}
	}
}




//=============================================================================
// Main


fn main() {
	App::build()
		.add_plugins( DefaultPlugins )
		.add_plugin( ComputerPlugin )
		.run();
}
