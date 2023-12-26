use nannou::prelude::*;
use nannou::rand;

const WINDOW_SIZE: (u32, u32) = (640, 480);
const SPACESHIP_PEAK: f32 = 16.25;
const SPACESHIP_TROUGH: f32 = 6.5;
const SPACESHIP_WIDTH: f32 = 30.0;
const SPACESHIP_HEIGHT: f32 = 39.0;
const SPACESHIP_SPEED: f32 = 3.0;
const ANGLE_INC: f32 = 3.6;
const MAX_PROJECTILES: u32 = 20;
const MISSILE_SPEED: f32 = 8.0;
const MISSILE_SIZE: f32 = 4.0;

/* Can have more than this, for example when a big asteroid explodes into little ones
 * however, this is used to prevent the game generating more */
const MAX_ASTEROIDS: u32 = 5;
const ASTEROID_MAX_SIZE: f32 = 80.0;
const ASTEROID_MIN_SIZE: f32 = 40.0;
const ASTEROID_MAX_SPEED: f32 = 4.0;
const ASTEROID_MIN_SPEED: f32 = -4.0;
const ASTEROID_WIGGLE: f32 = ASTEROID_MAX_SIZE + 20.0;


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
    thrust_rotation: f32,
    thrust_counter: u32,
    missile: Vec<Projectile>,
}

struct Asteroid{
    position: Point2,
    rotation: f32,
    rotation_speed: f32,
    size: f32,
    num_points: u32,
    fragment: bool, // Whether it is a fragment or not
    thickness: f32,
    points: Vec<Point2>
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
                thrust_rotation: 0.0,
                thrust_counter: 0,
                missile: Vec::new(),
        },
        asteroid: Vec::new(),
        last_event: KeyReleased(Key::Escape),
        state: state_idle,
    };

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

    if missile.hit
    {
        has_hit = true;
    }
    
    if has_hit
    {
        println!("Removing missile from vector");
    }

    has_hit
}

fn has_missile_hit_asteroid(missiles: &mut Vec<Projectile>, asteroid: &Asteroid, score: &mut u32, fragment: &mut Vec<Asteroid>) -> bool{
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
                *score += 1;
               
                if !asteroid.fragment{
                    let new_point_l = pt2(asteroid.position.x - (asteroid.size / 2.0), asteroid.position.y);
                    let asteroid_l = generate_asteroid(new_point_l, 12, ASTEROID_MIN_SIZE / 2.0, ASTEROID_MAX_SIZE / 2.0, true);
                    
                    let new_point_r = pt2(asteroid.position.x + (asteroid.size / 2.0), asteroid.position.y);
                    let asteroid_r = generate_asteroid(new_point_r, 12, ASTEROID_MIN_SIZE / 2.0, ASTEROID_MAX_SIZE / 2.0, true);

                    fragment.push(asteroid_l);
                    fragment.push(asteroid_r); 
                }
            }
        }

    has_hit
}

fn has_ship_hit_asteroid(player: &Player, asteroids: &Vec<Asteroid>) -> bool{
    let mut has_hit = false;

    let true_rotation = player.rotation + deg_to_rad(90.0);
    let true_x = player.position.x + (SPACESHIP_PEAK * true_rotation.cos());
    let true_y = player.position.y + (SPACESHIP_PEAK * true_rotation.sin());
    
    for asteroid in asteroids{
        let left_edge:bool = (true_x) > (asteroid.position.x - (asteroid.size / 2.0));
        let right_edge:bool = (true_x) < (asteroid.position.x + (asteroid.size / 2.0));
        let top_edge:bool = (true_y) < (asteroid.position.y + (asteroid.size / 2.0));
        let bottom_edge:bool = (true_y) > (asteroid.position.y - (asteroid.size / 2.0));

        if left_edge && right_edge && top_edge && bottom_edge
        {
            println!("CRASH!");
            has_hit = true;
        }
    }

    has_hit
}

fn new_point(player: &Player, asteroids: &Vec<Asteroid>) -> Point2{
    let mut valid_position = false;

    let mut new_x = 0.0;
    let mut new_y = 0.0;

    while !valid_position{
        let mut valid_spaceship_pos = false;
        while !valid_spaceship_pos{
            new_x = random_range((WINDOW_SIZE.0 as f32 / -2.0) + ASTEROID_MAX_SIZE, (WINDOW_SIZE.0 as f32 / 2.0) - ASTEROID_MAX_SIZE);
            new_y = random_range((WINDOW_SIZE.1 as f32 / -2.0) + ASTEROID_MAX_SIZE, (WINDOW_SIZE.1 as f32 / 2.0) - ASTEROID_MAX_SIZE);
        
            let left_edge:bool = new_x < player.position.x - ASTEROID_MAX_SIZE;
            let right_edge:bool = new_x > player.position.x + ASTEROID_MAX_SIZE;
            let top_edge:bool = new_y > player.position.y + ASTEROID_MAX_SIZE;
            let bottom_edge:bool = new_y < player.position.y - ASTEROID_MAX_SIZE;
            if left_edge || right_edge
            {
                if top_edge || bottom_edge {
                    valid_spaceship_pos = true;
                }
            }
        }

        if asteroids.len() < 1{
            valid_position = true;
        }
        else
        {
            valid_position = true;
            for asteroid in asteroids{
                let left_edge:bool = new_x > asteroid.position.x - ASTEROID_WIGGLE;
                let right_edge:bool = new_x < asteroid.position.x + ASTEROID_WIGGLE;
                let top_edge:bool = new_y < asteroid.position.y + ASTEROID_WIGGLE;
                let bottom_edge:bool = new_y > asteroid.position.y - ASTEROID_WIGGLE;
                if left_edge && right_edge && top_edge && bottom_edge{
                    valid_position = false;
                    break;
                }
            }
        }
    }

    pt2(new_x, new_y)
}

fn generate_asteroid(position: Point2, num_points: u32, min_size: f32, max_size: f32, fragment: bool) -> Asteroid{
    let new_size = random_range(min_size, max_size);
    
    let new_speed = random_range(ASTEROID_MIN_SPEED, ASTEROID_MAX_SPEED);

    let mut thickness = 5.0;
    if fragment{
        thickness *= 0.4;
    }

    let mut asteroid = Asteroid{
        position: position,
        rotation: 0.0,
        rotation_speed: deg_to_rad(new_speed),
        size: new_size,
        points: Vec::new(),
        num_points: 8,
        thickness: thickness,
        fragment: fragment
    };
    let angle_inc:f32 = (360 / asteroid.num_points) as f32;
    for i in 0..asteroid.num_points{
        let angle = deg_to_rad(i as f32 * angle_inc);
        let radius = asteroid.size / 2.0;

        let real_radius = random_range( radius * 0.80, radius * 1.20);

        let x = angle.sin() * real_radius;
        let y = angle.cos() * real_radius;
        
        asteroid.points.push(pt2(x,y));
    }

    /* Connect the last dot */
    asteroid.points.push(asteroid.points[0]);
    asteroid
}

fn update(app: &App, model: &mut Model, update: Update) {
    let win = app.window_rect();

    /* First, has the model crashed into anything? */
    let crashed = has_ship_hit_asteroid(&model.player, &model.asteroid);

    model.player.rotation += model.player.rotation_inc;
    if model.player.thrust{
        model.player.thrust_rotation = model.player.rotation;
        model.player.thrust_counter = 0;
    }

    /* Handle wrapping across boundaries */
    let true_rotation = model.player.rotation + deg_to_rad(90.0);
    if model.player.position.x + (SPACESHIP_PEAK * true_rotation.cos()) > (win.right()){
        let new_pos_x = model.player.position.x - WINDOW_SIZE.0 as f32;
        model.player.position.x = new_pos_x;
    }
    else if model.player.position.x + (SPACESHIP_PEAK * true_rotation.cos()) < (win.left()){
        let new_pos_x = model.player.position.x + WINDOW_SIZE.0 as f32;
        model.player.position.x = new_pos_x;
    }

    if model.player.position.y + (SPACESHIP_PEAK * true_rotation.sin()) > win.top(){
        let new_pos_y = model.player.position.y - WINDOW_SIZE.1 as f32;
        model.player.position.y = new_pos_y;
    }
    else if model.player.position.y + (SPACESHIP_PEAK * true_rotation.sin()) < win.bottom(){
        let new_pos_y = model.player.position.y + WINDOW_SIZE.1 as f32;
        model.player.position.y = new_pos_y;
    }

    //let exp = (model.player.thrust_counter as f32 * -0.012).exp();
    let exp = (model.player.thrust_counter as f32 * -0.5).exp() + 0.2;
    model.player.position.x += -SPACESHIP_SPEED * model.player.thrust_rotation.sin() * exp;
    model.player.position.y += SPACESHIP_SPEED * model.player.thrust_rotation.cos() * exp;
    if model.player.thrust_counter < u32::MAX{
        model.player.thrust_counter += 1;
    }

    for asteroid in &mut model.asteroid{
        asteroid.rotation += asteroid.rotation_speed;
    }
    
    let mut fragments:Vec<Asteroid> = Vec::new();
    assert_eq!(fragments.len(), 0);
    model.asteroid.retain(|asteroids| !has_missile_hit_asteroid(&mut model.player.missile, asteroids, &mut model.player.score, &mut fragments));

    for asteroid in fragments{
        model.asteroid.push(asteroid);
    }

    model.player.missile.retain(|missiles| !has_missile_hit_edge(missiles, app.window_rect()));
    
    for missile in &mut model.player.missile{
        missile.position.x += -MISSILE_SPEED * missile.rotation.sin();
        missile.position.y += MISSILE_SPEED * missile.rotation.cos();
    }

    /* Generate new asteroid if needed */
    if model.asteroid.len() < MAX_ASTEROIDS as usize
    {
        let new_pt =  new_point(&model.player, &model.asteroid);
        let asteroid = generate_asteroid(new_pt, 8, ASTEROID_MIN_SIZE, ASTEROID_MAX_SIZE, false);

        model.asteroid.push(asteroid);
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
    

    if model.player.thrust{
        let point5 = pt2(-7.0,-10.0);
        let point6 = pt2(7.0,-10.0);
        let point7 = pt2(0.0,-40.0);
        draw.tri()
            .points(point5,point6,point7)
            .x_y(model.player.position.x, model.player.position.y)
            .rotate(model.player.rotation)
            .color(RED);
        let point8 = pt2(-6.0,-10.0);
        let point9 = pt2(6.0,-10.0);
        let point10 = pt2(0.0,-30.0);
        draw.tri()
            .points(point8,point9,point10)
            .x_y(model.player.position.x, model.player.position.y)
            .rotate(model.player.rotation)
            .color(YELLOW);
    }
    let point1 = pt2(-(SPACESHIP_WIDTH / 2.0), -(SPACESHIP_PEAK + SPACESHIP_TROUGH));
    let point2 = pt2(0.0, -SPACESHIP_PEAK);
    let point3 = pt2((SPACESHIP_WIDTH / 2.0), -(SPACESHIP_PEAK + SPACESHIP_TROUGH));
    let point4 = pt2(0.0, SPACESHIP_PEAK);

    draw.quad()
        .points(point1,point2,point3,point4)
        .x_y(model.player.position.x, model.player.position.y)
        .rotate(model.player.rotation)
        .color(WHITE);

    let true_rotation = model.player.rotation + deg_to_rad(90.0 + 180.0);
    if model.player.position.x + (SPACESHIP_PEAK * true_rotation.cos()) > (win.right()){
        let new_pos_x = model.player.position.x - WINDOW_SIZE.0 as f32;
        draw.quad()
            .points(point1,point2,point3,point4)
            .x_y(new_pos_x, model.player.position.y)
            .rotate(model.player.rotation)
            .color(GREEN);
    }
    else if model.player.position.x + (SPACESHIP_PEAK * true_rotation.cos()) < (win.left()){
        let new_pos_x = model.player.position.x + WINDOW_SIZE.0 as f32;
        draw.quad()
            .points(point1,point2,point3,point4)
            .x_y(new_pos_x, model.player.position.y)
            .rotate(model.player.rotation)
            .color(GREEN);
    }
    
    if model.player.position.y + (SPACESHIP_PEAK * true_rotation.sin()) > (win.top()){
        let new_pos_y = model.player.position.y - WINDOW_SIZE.1 as f32;
        draw.quad()
            .points(point1,point2,point3,point4)
            .x_y(model.player.position.x, new_pos_y)
            .rotate(model.player.rotation)
            .color(GREEN);
    }
    else if model.player.position.y + (SPACESHIP_PEAK * true_rotation.sin()) < (win.bottom()){
        let new_pos_y = model.player.position.y + WINDOW_SIZE.1 as f32;
        draw.quad()
            .points(point1,point2,point3,point4)
            .x_y(model.player.position.x, new_pos_y)
            .rotate(model.player.rotation)
            .color(GREEN);
    }

    for missile in &model.player.missile{
        draw.rect()
            .xy(missile.position)
            .w_h(MISSILE_SIZE, MISSILE_SIZE)
            .color(WHITE);
    }

    for asteroid in &model.asteroid{ 
        draw.polyline()
            .xy(asteroid.position)
            .weight(asteroid.thickness)
            .color(WHITE)
            .rotate(asteroid.rotation)
            .points(asteroid.points.clone());
    }

    let score = format!("Score: {}", model.player.score);
    draw.text(&score)
        .font_size(40)
        .xy(pt2(win.right() - 80.0 , win.bottom() + 30.0));

    draw.to_frame(app, &frame).unwrap();
}
