#![feature(let_else)]
use std::collections::{HashSet, HashMap};

use bevy::{prelude::*, core::CorePlugin, input::{InputPlugin, keyboard::KeyboardInput}, app::ScheduleRunnerPlugin, asset::AssetPlugin};
use bevy::asset::AssetLoader;
use my_bevy_game::loader::MetadataPlugin;

mod loader;

fn load_people(
    mut commands: Commands,
    query: Query<(Entity, &Name), With<Person>>,
    assets: Res<Assets<my_bevy_game::loader::Metadata>>,
    mut ev_metadata: EventReader<AssetEvent<my_bevy_game::loader::Metadata>>,
) {
    let mut crowd: HashMap<String, _> = query.iter().map(|xs| (xs.1.0.to_string(), xs.0)).collect();
    let crowd_keys: HashSet<_> = crowd.keys().map(|xs| xs.to_string()).collect();

    // get all people, somehow
    // find or update them
    if let Some(ev) = ev_metadata.iter().last() {
        let Some(metadata) = assets.get(match ev {
            AssetEvent::Created { handle } => handle,
            AssetEvent::Modified { handle } => handle,
            _ => return
        }) else {
            return
        };

        if let Some(people) = metadata.0.get("people").and_then(|v| v.as_array()) {
            let people: HashSet<String> = people
                .into_iter()
                .filter_map(|v| v.as_str())
                .map(|v| v.to_string())
                .collect();

            for name in people.difference(&crowd_keys) {
                commands.spawn().insert(Person).insert(Name(name.clone()));
            }

            for name in crowd_keys.difference(&people) {
                let n = crowd.remove(name).unwrap();

                commands.entity(n).remove::<Person>().remove::<Name>();
            }

        }

    }
//    commands.spawn().insert(Person).insert(Name("Octavia Butler".to_string()));
//    commands.spawn().insert(Person).insert(Name("Iain M. Banks".to_string()));
}

struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(MetadataPlugin)
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(load_metadata)
            .add_system(load_people)
            .add_system(greet_people);
    }
}

struct MetadataResource(Handle<my_bevy_game::loader::Metadata>);

fn load_metadata(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    asset_server.watch_for_changes().expect("failed to watch for asset changes");
    let handle: Handle<my_bevy_game::loader::Metadata> = asset_server.load("test.json");
    commands.insert_resource(MetadataResource(handle));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("Hello, {}!", name.0);
        }
    }
}

fn recv_keypresses(
    mut ev_keypress: EventReader<KeyboardInput>,
) {
    for ev in ev_keypress.iter() {
        eprintln!("ev {:?} {:?}!", ev.key_code, ev.state);
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugin(CorePlugin)
        .add_plugin(AssetPlugin)
        .add_plugin(ScheduleRunnerPlugin)
        .add_plugin(HelloPlugin)
        .run();
}

