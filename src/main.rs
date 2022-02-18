use bevy::prelude::*;
use bevy_mod_picking;

const MAPX: usize = 20;
const MAPZ: usize = 20;

const CENTER: (usize,usize) = (MAPX/2, MAPZ/2);

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
}

fn main() {
    App::new()
    // Set antialiasing to use 4 samples
    .insert_resource(Msaa { samples: 4 })
    .init_resource::<Tank>()

    .add_startup_system(init_cam)
    .add_startup_system(init_map)
    .add_startup_system(init_unit)

    // Set WindowDescriptor Resource to change title and size
    .insert_resource(WindowDescriptor {
        title: "Game!".to_string(),
        width: 1600.,
        height: 800.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_state(GameState::Playing)
    .add_system_set(SystemSet::on_update(GameState::Playing).with_system(move_tank))
    .add_system_set(SystemSet::on_update(GameState::Playing).with_system(on_click))
    .run();
}

fn init_cam(
    mut commands: Commands,
) {
        let x = CENTER.0 as f32;
        let z = CENTER.1 as f32;
        // Camera
        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(x, (x + z) / 2., z + 20. )
                .looking_at(Vec3::new(x, 0., z), Vec3::Y),
            ..Default::default()
        });
        // Light
        commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(15.0, 8.0, 15.0)),
            ..Default::default()
        });
}

fn init_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    //initialize GridPx Mesh
    let green = materials.add(Color::rgb(0.,1.,0.).into());
    let black = materials.add(Color::rgb(0.,0.,0.).into());

    let base = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 1.0 })),
        material: green,
        ..Default::default()
    };

    let border= PbrBundle {
        mesh: meshes.add(gib_mesh()),
        material: black,
        ..Default::default()
    };

    for x in 0..MAPX{
        for z in 0..MAPZ{
            let mut moved = base.clone();
            moved.transform.translation.x += x as f32;
            moved.transform.translation.z += z as f32;

            commands.spawn_bundle(moved).with_children(|parent| {
                parent.spawn_bundle(border.clone());
            });
        }
    };
}

fn init_unit(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut tank: ResMut<Tank>,

) {
    //initial tank position
    tank.x = 15.0;
    tank.z = 15.0;
    //spawn tank
    tank.entity = Some( commands.spawn_bundle((
            Transform {
                scale: Vec3::new(0.1, 0.1, 0.1),
                translation: Vec3::new(15.0, 0.0, 15.0),
                ..Default::default()
            },
            GlobalTransform::identity(),
        ))
        .with_children(|cell| {
            cell.spawn_scene(asset_server.load("Tank4.glb#Scene0"));
        })
        .id());
}

fn move_tank(
    keyboard_input: Res<Input<KeyCode>>,
    mut tank: ResMut<Tank>,
    mut transforms: Query<&mut Transform>,
){

    let mut rotation = 0.0;
    if keyboard_input.pressed(KeyCode::W) {
        if tank.z > 0. {
            tank.z -= 0.1;
        }
        rotation = -std::f32::consts::FRAC_PI_2;
    }
    if keyboard_input.pressed(KeyCode::A) {
        if tank.x > 0. {
            tank.x -= 0.1;
        }
        rotation = 0.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        if tank.z < (MAPZ - 1) as f32 {
            tank.z += 0.1;
        }
        rotation = std::f32::consts::FRAC_PI_2;
    }
    if keyboard_input.pressed(KeyCode::D) {
        if tank.x < (MAPX - 1) as f32 {
            tank.x += 0.1;
        }
        rotation = std::f32::consts::PI;
    }

    // move on the board
    *transforms.get_mut(tank.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(tank.x,0.0,tank.z),
        rotation: Quat::from_rotation_y(rotation),
        scale: Vec3::new(0.1, 0.1, 0.1),
    }
}

fn on_click(
    mut mouse_button_input_events: EventReader<bevy::input::mouse::MouseButtonInput>,
    mouse_button_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
){
    let window = windows.get_primary().unwrap();
    if let Some(position) = window.cursor_position(){
    //    info!("{:?}",position);
    }
    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }
}

#[derive(Default, Debug)]
struct Tank{
    x: f32,
    z: f32,
    entity:Option<Entity>,
}


fn gib_mesh() -> Mesh{
    let o = 0.5;
    let i = 0.4;

    let positions = vec![
        [-o, 0.1, -o],
        [-i, 0.1, -i],

        [ o, 0.1, -o],
        [ i, 0.1, -i],

        [-o, 0.1,  o],
        [-i, 0.1,  i],
        
        [ o, 0.1,  o],
        [ i, 0.1,  i],
    ];
    let uvs = vec![[0., 0.]; 8];
    let normals = vec![[0.0, 1.0, 0.0]; 8];

    //counter-clockwise !!!
    let indices = bevy::render::mesh::Indices::U32(vec![
        0, 3, 2, 0, 1, 3,
        0, 4, 5, 0, 5, 1, 
        4, 6, 5, 5, 6, 7, 
        7, 6, 2, 7, 2, 3, 
    ]);

    let mut mesh = Mesh::new(bevy::render::render_resource::PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(indices));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    mesh
}