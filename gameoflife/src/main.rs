use bevy::prelude::*;

fn setup(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>){

    //meshes.add(Circle::new(50.0));
    let square = Rectangle::new(50.0, 50.0);
    let colour = Color::hsl(1.0,1.0,1.0);
    //materials.add(colour);

    commands.spawn(Camera2d);
    commands.spawn((Mesh2d(meshes.add(square)),MeshMaterial2d(materials.add(colour)),Transform::from_xyz(0.0,0.0,0.0)));
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin{
        primary_window: { 
            Some(
                Window{
                    title: "Conway's game of life".into(),
                    resolution: (800., 600.).into(),
                    resize_constraints: WindowResizeConstraints{min_width: 800., min_height: 600., max_width: 800., max_height: 600.},
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
