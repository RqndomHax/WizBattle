use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, window::*};

use bevy_obj::*;
use bevy_fly_camera::*;


mod structures;
use structures::structures::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "bevy-introduction simple window".to_string(),
            width: 1920.,
            height: 1080.,
            present_mode: PresentMode::Immediate,
            cursor_visible: false,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(ObjPlugin)
        .add_plugin(FlyCameraPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {

    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
        material: materials.add(Color::rgb(0.9, 0.0, 0.2).into()),
        ..default()
    });

    for i in 1..100 {
        commands.spawn_bundle(HeadQuarters {
            x: i as f32 * 12.0,
            y: 0.5,
            z: 2.0,
        }.build(&asset_server, materials.as_mut()));
    
        commands.spawn_bundle(ManaGenerator {
            x: i as f32 * 12.0,
            y: 0.5,
            z: 18.0,
        }.build(&asset_server, materials.as_mut()));
    }

    commands.insert_resource(AmbientLight {
        color: Color::AQUAMARINE,
        brightness: 1.0,
    });

    // light
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            intensity: 100000.0,
            shadows_enabled: true,
            radius: 4020.0,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle::new_3d())
        .insert(FlyCamera::default());
}
