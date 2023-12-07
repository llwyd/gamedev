use nannou::prelude::*;

const WINDOW_SIZE: (u32, u32) = (640, 480);
const SPACESHIP_PEAK: f32 = 16.25;
const SPACESHIP_TROUGH: f32 = 6.5;
const SPACESHIP_WIDTH: f32 = 30.0;
const SPACESHIP_HEIGHT: f32 = 39.0;
const SPACESHIP_SPEED: f32 = 4.0;
const ANGLE_INC: f32 = 3.6;
const MAX_PROJECTILES: u32 = 20;
const MISSILE_SPEED: f32 = 8.0;
const MISSILE_SIZE: f32 = 4.0;

/* Can have more than this, for example when a big asteroid explodes into little ones
 * however, this is used to prevent the game generating more */
const MAX_ASTEROIDS: u32 = 1;
const ASTEROID_MAX_SIZE: f32 = 40.0;

#[derive(Copy,Clone)]
enum State{
    Idle, // Normal game 
//    GameOver,
//    Menu,
}

#[derive(Copy,Clone)]
enum StateEvents{
    NoneKeyPress,
    LeftKeyPress,
    LeftKeyRelease,
    RightKeyPress,
    RightKeyRelease,
    UpKeyPress,
    UpKeyRelease,
    SpaceKeyPress,
    SpaceKeyRelease,
}


struct Player{
    position: Point2,
    rotation: f32,
    rotation_inc: f32,
    score: u32,
    thrust: bool,
    missile: Vec<Projectile>,
}

struct Asteroid{
    position: Point2,
    rotation: f32,
    rotation_speed: f32,
    size: f32,
}

struct Projectile{
    hit: bool,
    position: Point2,
    rotation: f32,
}

type StateFunc = fn(&mut Player,StateEvents);

struct Model {
    player: Player,
    asteroid: Vec<Asteroid>,
    last_event: WindowEvent,
    state: StateFunc,
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WINDOW_SIZE.0, WINDOW_SIZE.1)
        .min_size(WINDOW_SIZE.0, WINDOW_SIZE.1)
        .max_size(WINDOW_SIZE.0, WINDOW_SIZE.1)
        .decorations(true)
        .resizable(false)
        .event(window_event)
        .build()
        .unwrap();
    
    let mut model = Model {
        player: Player {
                position: pt2(0.0, 0.0),
                rotation: 0.0,
                rotation_inc: 0.0,
                score: 0,
                thrust: false,
                missile: Vec::new(),
        },
        asteroid: Vec::new(),
        last_event: KeyReleased(Key::Escape),
        state: state_idle,
    };

    let asteroid = Asteroid{
        position: pt2(100.0, 100.0),
        rotation: 0.0,
        rotation_speed: 1.0,
        size: ASTEROID_MAX_SIZE
    };
    
    model.asteroid.push(asteroid);

    model
}

fn event(_app: &App, _model: &mut Model, _event: Event) { }

fn keypress_to_state(key: Key) -> StateEvents{
    match key{
        Key::Left => return StateEvents::LeftKeyPress,
        Key::Right => return StateEvents::RightKeyPress,
        Key::Up => return StateEvents::UpKeyPress,
        Key::Space => return StateEvents::SpaceKeyPress,
        _ => return StateEvents::NoneKeyPress,
    }
}

fn keyrelease_to_state(key: Key) -> StateEvents{
    match key{
        Key::Left => return StateEvents::LeftKeyRelease,
        Key::Right => return StateEvents::RightKeyRelease,
        Key::Up => return StateEvents::UpKeyRelease,
        Key::Space => return StateEvents::SpaceKeyRelease,
        _ => return StateEvents::NoneKeyPress,
    }
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent)
{
    if model.last_event != event
    {
        match event {
            KeyPressed(key) => { println!("Key Pressed"); (model.state)(&mut model.player,keypress_to_state(key)) }
            KeyReleased(key) => { println!("Key Released");(model.state)(&mut model.player,keyrelease_to_state(key)) }
            _ => {}
        }
        model.last_event = event;
    }
}

fn fire_missile(player: &mut Player)
{
    println!("Firing missile");
    let missile = Projectile{
        hit: false,
        position: player.position,
        rotation: player.rotation,
    };
    player.missile.push(missile);
}

fn has_missile_hit_edge(missile: &Projectile, win: Rect) -> bool{
    let mut has_hit = false;

    if missile.position.x > win.right()
    {
        has_hit = true;
    }
    else if missile.position.x < win.left()
    {
        has_hit = true;
    }
    else if missile.position.y > win.top()
    {
        has_hit = true;
    }
    else if missile.position.y < win.bottom()
    {
        has_hit = true;
    }

    if has_hit
    {
        println!("Removing missile from vector");
    }

    if missile.hit
    {
        println!("Asteroid hit by missile");
        has_hit = true;
    }

    has_hit
}

fn has_missile_hit_asteroid(missiles: &mut Vec<Projectile>, asteroid: &Asteroid) -> bool{
    let mut has_hit = false;

        for missile in &mut *missiles
        {
            let left_edge:bool = (missile.position.x + (MISSILE_SIZE/2.0)) > (asteroid.position.x - (asteroid.size / 2.0));
            let right_edge:bool = (missile.position.x + (MISSILE_SIZE/2.0)) < (asteroid.position.x + (asteroid.size / 2.0));
            let top_edge:bool = (missile.position.y + (MISSILE_SIZE/2.0)) < (asteroid.position.y + (asteroid.size / 2.0));
            let bottom_edge:bool = (missile.position.y + (MISSILE_SIZE/2.0)) > (asteroid.position.y - (asteroid.size / 2.0));

            if left_edge && right_edge && top_edge && bottom_edge
            {
                println!("Hit!");
                missile.hit = true;
                has_hit = true;
            }
        }

    has_hit
}



fn update(app: &App, model: &mut Model, update: Update) {
    model.player.rotation += model.player.rotation_inc;
    if model.player.thrust{
        model.player.position.x += -SPACESHIP_SPEED * model.player.rotation.sin();
        model.player.position.y += SPACESHIP_SPEED * model.player.rotation.cos();
    }

    model.asteroid.retain(|asteroids| !has_missile_hit_asteroid(&mut model.player.missile, asteroids));
    model.player.missile.retain(|missiles| !has_missile_hit_edge(missiles, app.window_rect()));
    
    for missile in &mut model.player.missile{
        missile.position.x += -MISSILE_SPEED * missile.rotation.sin();
        missile.position.y += MISSILE_SPEED * missile.rotation.cos();
    }
}

fn state_idle(player: &mut Player, event:StateEvents)
{
    match event{
        StateEvents::LeftKeyPress =>{player.rotation_inc = deg_to_rad(ANGLE_INC)},
        StateEvents::LeftKeyRelease => {player.rotation_inc = deg_to_rad(0.0)},
        StateEvents::RightKeyPress => {player.rotation_inc = deg_to_rad(-ANGLE_INC)},
        StateEvents::RightKeyRelease => {player.rotation_inc = deg_to_rad(0.0)},
        StateEvents::UpKeyPress => {player.thrust = true},
        StateEvents::UpKeyRelease => {player.thrust = false},
        StateEvents::SpaceKeyPress => { fire_missile(player) },
        _ => { /* Do nowt */}
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);

    let point1 = pt2(0.0 - (SPACESHIP_WIDTH / 2.0), 0.0 - (SPACESHIP_PEAK + SPACESHIP_TROUGH));
    let point2 = pt2(0.0, 0.0 - SPACESHIP_PEAK);
    let point3 = pt2(0.0 + (SPACESHIP_WIDTH / 2.0), 0.0 - (SPACESHIP_PEAK + SPACESHIP_TROUGH));
    let point4 = pt2(0.0, 0.0 + SPACESHIP_PEAK);

    draw.quad()
        .points(point1,point2,point3,point4)
        .x_y(model.player.position.x, model.player.position.y)
        .rotate(model.player.rotation)
        .color(WHITE);

    for missile in &model.player.missile{
        draw.rect()
            .xy(missile.position)
            .w_h(MISSILE_SIZE, MISSILE_SIZE)
            .color(WHITE);
    }

    for asteroid in &model.asteroid{
        draw.rect()
            .xy(asteroid.position)
            .w_h(asteroid.size, asteroid.size)
            .color(WHITE);
    }

    draw.to_frame(app, &frame).unwrap();
}
