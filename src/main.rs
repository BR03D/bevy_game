use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Playing,
}

fn main() {
    App::new()
    // Set antialiasing to use 4 samples
    .insert_resource(Msaa { samples: 4 })
    .init_resource::<Tank>()
    // Set WindowDescriptor Resource to change title and size
    .insert_resource(WindowDescriptor {
        title: "Game!".to_string(),
        width: 1600.,
        height: 800.,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_state(GameState::Playing)
    .add_startup_system(setup)
    //.add_system_set(SystemSet::on_update(GameState::Playing).with_system(on_tick))
    .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut tank: ResMut<Tank>,
) {

        //initialize GridPx Mesh
        let grid_mesh = meshes.add(Mesh::from(shape::Plane { size: 1.0 }));
        let grid_color_r = materials.add(Color::rgb(1.,0.,0.).into());
        let grid_color_g = materials.add(Color::rgb(0.,1.,0.).into());
        let grid_color_b = materials.add(Color::rgb(0.,0.,1.).into());

        let (map, mut commands) = Map::init((40, 40), grid_mesh, (grid_color_r, grid_color_g, grid_color_b), commands);

        // Camera
        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 25.0, -25.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
        // Light
        commands.spawn_bundle(PointLightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        });

        //spawn tank
//        let tank = commands.spawn_scene(asset_server.load("Tank4.glb#Scene0"));

        tank.entity = Some( commands.spawn_bundle((
                Transform {
                    translation: Vec3::new(0.,0.,0.),
                    //rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_2),
                    ..Default::default()
                },
                GlobalTransform::identity(),
            ))
            .with_children(|cell| {
                cell.spawn_scene(asset_server.load("Tank4.glb#Scene0"));
            })
            .id());

        //move tank
}

fn on_tick(
    mut tank: ResMut<Tank>,
    mut transforms: Query<&mut Transform>,
){

    let x = tank.pos.0 + 0.1;
    tank.pos.0 = x;

    *transforms.get_mut(tank.entity.unwrap()).unwrap() = Transform {
        translation: Vec3::new(
            x,
            0.,
            0.,
        ),
        ..Default::default()
    };
}

#[derive(Default, Debug)]
struct Tank{
    pos:(f32,f32),
    entity:Option<Entity>,
}

struct Map{
    grid:Vec<Vec<GridPx>>
}

impl Map{
    fn init<'w, 's>( 
        (x,y):(usize,usize), 
        mesh: Handle<Mesh>, 
        (r,g,b):(Handle<StandardMaterial>,Handle<StandardMaterial>,Handle<StandardMaterial>),
        mut commands: Commands<'w, 's>,
    ) -> (Map, Commands<'w, 's>) {

        let mut outer = Vec::new();

        for i in 0..x{
            let mut inner = Vec::new();
            for j in 0..y{
                inner.push(GridPx::new((i,j)));

                // Plane
                commands.spawn_bundle(PbrBundle {
                    mesh: mesh.clone(),
                    material: r.clone(),
                    transform: Transform::from_xyz(i as f32, 0., j as f32),
                    ..Default::default()
                });
            }
            outer.push(inner);
        };

        (
            Map{
                grid:outer
            },
            commands
        )
    }
}

struct GridPx{
    pos:(usize,usize),
}

impl GridPx{
    fn new((x,y):(usize,usize)) -> GridPx{
        GridPx{
            pos:(x,y),
        }
    }
}