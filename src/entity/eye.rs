use crate::ent;
use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle};

ent! {EyePlugin {
    fn init(app:&mut App) {
        app
        .add_plugins(Material2dPlugin::<EyeMaterial>::default())
        .init_resource::<EyeProps>();
    }
}}

#[derive(Default, Resource)]
struct EyeProps {}

impl EyePlugin {
    fn init(
        mut cmd: Commands,
        ass: Res<AssetServer>,
        mut meshs: ResMut<Assets<Mesh>>,
        mut mat: ResMut<Assets<EyeMaterial>>,
        // mut cmat: ResMut<Assets<ColorMaterial>>,
    ) {
        // cmd.spawn(SceneBundle {
        //     scene: (ass.load("res/model/eye.gltf#Scene0")),
        //     transform: Transform::from_rotation(euler!(90.0, 0.0, 0.0)),
        //     // material: mat.add(EyeMaterial::default()),
        //     ..default()
        // });
        cmd.spawn(MaterialMesh2dBundle {
            mesh: meshs.add(Rectangle::default()).into(),
            transform: Transform::default()
                .with_translation(vec3(0.0, 0.0, -5.0))
                .with_scale(vec3(2.0, 2.0, 1.0) * Vec3::splat(1024.0)),
            // material: cmat.add(ColorMaterial {
            //     color: Color::rgba(1.0, 0.0, 1.0, 1.0),
            //     ..default()
            // }),
            material: mat
                .add(EyeMaterial {
                    color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                    color_texture: Some(ass.load("res/texture/eye_mask.png")),
                })
                .into(),
            ..default()
        });
        // cmd.spawn(SpriteBundle {
        //     transform: Transform::from_translation(vec3(0.0, 0.0, -10.0)),
        //     texture: ass.load("res/texture/eye_mask.png").into(),
        //     ..default()
        // });
    }
    fn tick() {}
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct EyeMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material2d for EyeMaterial {
    fn fragment_shader() -> ShaderRef {
        "res/shaders/eye.wgsl".into()
    }

    // fn alpha_mode(&self) -> AlphaMode {
    //     self.alpha_mode
    // }
}
