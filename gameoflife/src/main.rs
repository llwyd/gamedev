use bevy::prelude::*;

const WINDOW_SIZE: (f32, f32) = (640., 480.);
const SQUARE_SIZE: f32 = 80.;
const NUM_COLS: u32 = (WINDOW_SIZE.0 / SQUARE_SIZE) as u32;
const NUM_ROWS: u32 = (WINDOW_SIZE.1 / SQUARE_SIZE) as u32;
const NUM_SQUARES: u32 = NUM_COLS * NUM_ROWS;
const ALIVE_COLOUR: Color = Color::hsl(1.0,1.0,1.0);
const DEAD_COLOUR: Color = Color::hsl(0.5,0.5,0.5);


fn setup(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>){

    println!("Columns: {:}", NUM_COLS);
    println!("   Rows: {:}", NUM_ROWS);
    println!("Squares: {:}", NUM_SQUARES);

    let square = Rectangle::new(SQUARE_SIZE, SQUARE_SIZE);
    let colour = ALIVE_COLOUR;

    commands.spawn(Camera2d);
    commands.spawn((Mesh2d(meshes.add(square)),MeshMaterial2d(materials.add(colour)),Transform::from_xyz(-320.0 + (SQUARE_SIZE / 2.0),0.0,0.0)));
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.0,0.0,0.0)));
    app.add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: { 
            Some(
                Window{
                    title: "Conway's game of life".into(),
                    resolution: (WINDOW_SIZE.0, WINDOW_SIZE.1).into(),
                    resize_constraints: WindowResizeConstraints{min_width: WINDOW_SIZE.0, min_height: WINDOW_SIZE.1, max_width: WINDOW_SIZE.0, max_height: WINDOW_SIZE.1},
                    resizable: false,
                    ..default()
                }
            )},
            ..default()
        })
        )
        .add_systems(Startup, setup)
        .run();
}
