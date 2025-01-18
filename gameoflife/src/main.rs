use bevy::prelude::*;
use rand::Rng;

const WINDOW_SIZE: (f32, f32) = (640., 480.);
const SQUARE_SIZE: f32 = 10.;
const NUM_COLS: u32 = (WINDOW_SIZE.0 / SQUARE_SIZE) as u32;
const NUM_ROWS: u32 = (WINDOW_SIZE.1 / SQUARE_SIZE) as u32;
const NUM_SQUARES: u32 = NUM_COLS * NUM_ROWS;
const ALIVE_COLOUR: Color = Color::hsl(1.0,1.0,1.0);
const DEAD_COLOUR: Color = Color::hsl(0.5,0.5,0.5);


#[derive(Component)]
struct IntPosition
{
    x: u32,
    y: u32,
}

#[derive(Component)]
struct Cell
{
    alive: bool,
    rect: Rectangle,
}

#[derive(Resource)]
struct Grid
{
    cell: Vec<Cell>,
}


fn setup(mut commands: Commands,
    mut game: ResMut<Grid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>){

    let mut rng = rand::thread_rng();
    
    commands.spawn(Camera2d);
    //commands.spawn((Mesh2d(meshes.add(square)),MeshMaterial2d(materials.add(colour)),Transform::from_xyz(-320.0 + (SQUARE_SIZE / 2.0),0.0,0.0)));

    for i in 0..NUM_SQUARES as usize{
        let alive: bool = rng.gen();
        println!("{:}", alive);
        game.cell.push(
            Cell{
                alive: alive,
                rect:Rectangle::new(SQUARE_SIZE, SQUARE_SIZE)
            });
        let colour = if game.cell[i].alive { ALIVE_COLOUR } else { DEAD_COLOUR };
        commands.spawn((Mesh2d(meshes.add(game.cell[i].rect)),MeshMaterial2d(materials.add(colour)),Transform::from_xyz(-320.0 + (SQUARE_SIZE / 2.0),0.0,0.0)));
    }
    assert!(game.cell.len() as u32 == NUM_SQUARES);
    
    println!("Columns: {:}", NUM_COLS);
    println!("   Rows: {:}", NUM_ROWS);
    println!("Squares: {:}", NUM_SQUARES);
}

fn update_loop(mut game: ResMut<Grid>){
    println!("Num Cells: {:}", game.cell.len());
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgb(0.0,0.0,0.0)));
    app.insert_resource(Grid{ cell: Vec::new() });
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
        .add_systems(Update, update_loop)
        .run();
}
