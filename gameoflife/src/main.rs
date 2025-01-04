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
    app.add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}
