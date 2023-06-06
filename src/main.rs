use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    winit::WinitSettings,
};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: bevy::window::PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..Default::default()
                }),
        )
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(Material2dPlugin::<TestMaterial>::default())
        .add_startup_system(setup)
        .run();
}

//create mesh struct
#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "3f1a48ee-ccf5-4e32-9725-9799157d19ff"]
pub struct TestMaterial {
    // seperator
}

//material impl definition
impl Material2d for TestMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/test_material.wgsl".into()
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<TestMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    //your eyes
    commands.spawn(Camera2dBundle::default());
    //spawn material bundle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        //(so its visible [i dont work in 2d not sure why its like that])
        transform: Transform::default().with_scale(Vec3::splat(128. * 10.0)),
        //with material
        material: materials.add(TestMaterial {}),
        ..Default::default()
    });
}
