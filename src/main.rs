use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
};
use bevy_ecs_ldtk::prelude::*;

const WIDTH: f32 = 256.;
const HEIGHT: f32 = 256.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WIDTH * 2., HEIGHT * 2.).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            LdtkPlugin,
        ))
        .insert_resource(Msaa::Off)
        .insert_resource(LevelSelection::index(0))
        .add_systems(Startup, setup)
        .add_systems(Update, add_camera)
        .register_ldtk_int_cell::<TileBundle>(1)
        .run();
}

#[derive(Component)]
struct CanvasCamera;

#[derive(Component)]
struct Canvas;

#[derive(Component)]
struct FooBar;

#[derive(Component, Default)]
struct Tile;

#[derive(Bundle, Default, LdtkIntCell)]
struct TileBundle {
    tile: Tile,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    let size = Extent3d {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);
    let canvas = images.add(image);

    commands.spawn((
        SpriteBundle {
            texture: canvas.clone(),
            transform: Transform::from_scale(Vec3::splat(2.)),
            ..default()
        },
        Canvas,
        RenderLayers::layer(1),
    ));

    commands.spawn((Camera2dBundle::default(), RenderLayers::layer(1)));

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tilemap.ldtk"),
        ..default()
    });

    commands.spawn(FooBar);
}

fn add_camera(
    camera: Query<&CanvasCamera>,
    mut commands: Commands,
    level_query: Query<&LevelIid>,
    // level_query: Query<&FooBar>, // INFO: Replace the query above with this one to fix the crash
    canvas_query: Query<&Handle<Image>, With<Canvas>>,
) {
    if let Ok(iid) = level_query.get_single() {
        if camera.is_empty() {
            let canvas = canvas_query.single();
            commands.spawn((
                Camera2dBundle {
                    camera: Camera {
                        order: -1,
                        target: RenderTarget::Image(canvas.clone()),
                        ..default()
                    },
                    ..default()
                },
                CanvasCamera,
            ));
        }
    }
}
