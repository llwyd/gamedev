use nannou::prelude::*;
use nannou::rand;
use nannou::text::Font;
use nannou_audio as audio;
use nannou_audio::Buffer;
use std::time::{Duration, Instant};

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
const ASTEROID_THICKNESS: f32 = 5.0;
const ASTEROID_WIGGLE: f32 = ASTEROID_MAX_SIZE + 20.0;
const ASTEROID_SPEED: f32 = 0.5;


#[derive(Copy,Clone)]
enum State{
    Idle, // Normal game 
    GameOver,
    Menu,
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
    thrust_rotation: f32,
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
    game_state:State,
    raw_font: Vec<u8>,
    score_font: Vec<u8>,
    credit_font: Vec<u8>,
    stream: audio::Stream<Audio>,
    difficulty: Difficulty,
    tick: Instant,
    display_text: bool,
}

struct Audio{
    audio: audrey::read::BufFileReader,
    event: Vec<audrey::read::BufFileReader>,
}

struct Difficulty{
    max_asteroids:u32,
    max_asteroid_speed:f32,
    tick: Instant,
    duration: Duration,
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
    
    let audio_host = audio::Host::new();
    let theme = audrey::open("assets/space_clutter_theme.wav").expect("Not Found");
    let audio_data = Audio{ 
        audio: theme,
        event: Vec::new()};

    let stream = audio_host
        .new_output_stream(audio_data)
        .render(audio)
        .channels(2)
        .sample_rate(44_100)
        .build()
        .unwrap();

    stream.play().unwrap();
    
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
        game_state: State::Menu,
        raw_font: include_bytes!("../assets/Kenney Mini.ttf").to_vec(),
        score_font: include_bytes!("../assets/Kenney Pixel.ttf").to_vec(),
        credit_font: include_bytes!("../assets/Kenney Mini.ttf").to_vec(),
        stream: stream,
        difficulty:Difficulty{
            max_asteroids: MAX_ASTEROIDS,
            max_asteroid_speed: ASTEROID_MAX_SPEED,
            tick: Instant::now(),
            duration: Duration::new(5, 0),
        },
        tick: Instant::now(),
        display_text: true,
    };

    model
}

fn reset_audio_loop(audio: &mut Audio){
    audio.audio = audrey::open("assets/space_clutter_theme.wav").expect("Not Found");
}

fn audio(audio:&mut Audio, buffer: &mut Buffer){
    
    let file_frames = audio.audio.frames::<[f32; 2]>().filter_map(Result::ok);
    let mut frames_written = 0; 
    let mut frames_available = buffer.len_frames();
    for (frame, file_frame) in buffer.chunks_mut(2).zip(file_frames) {
        //println!("{:?}, {:?}", frame, file_frame);
        for (sample, &file_sample) in frame.iter_mut().zip(&file_frame) {
            //println!("{:?}, {:?}", sample, file_sample);
            
            //*sample = file_sample/2.0;
        }
        frames_written += 1;
    }
//    println!("{:?} : {:?}", frames_written, frames_available );

    if frames_written < frames_available{
        println!("Restart audio loop");
        reset_audio_loop(audio);
    }
    let mut event_ended = Vec::new();
    for (i, event) in audio.event.iter_mut().enumerate(){
        let file_frames = event.frames::<[f32; 2]>().filter_map(Result::ok);
        let mut frames_written = 0; 
        let mut frames_available = buffer.len_frames();
        for (frame, file_frame) in buffer.chunks_mut(2).zip(file_frames) {
            for (sample, &file_sample) in frame.iter_mut().zip(&file_frame) {
              //  *sample += file_sample /2.0;
            }
            frames_written += 1;
        }
        if frames_written < frames_available{
            println!("Pop audio event");
            event_ended.push(i);
        }
    }

    for e in event_ended{
        audio.event.remove(e);
    }
} 

fn reset(app: &App, model: &mut Model){
    model.player.position = pt2(0.0, 0.0);
    model.player.rotation = 0.0;
    model.player.rotation_inc = 0.0;
    model.player.score = 0;
    model.player.thrust = false;
    model.player.thrust_rotation = 0.0;
    model.player.thrust_counter = 0;
    model.player.missile = Vec::new();
    
    model.asteroid = Vec::new();
    model.last_event = KeyReleased(Key::Escape);
    model.game_state = State::Idle;

    model.difficulty.max_asteroids = MAX_ASTEROIDS;
    model.difficulty.max_asteroid_speed = ASTEROID_MAX_SPEED;
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
    match model.game_state{
        State::Idle => idle_event(app, model, event),
        State::GameOver => menu_event(app, model, event),
        State::Menu => menu_event(app, model, event),
    }
}

fn menu_event(app: &App, model: &mut Model, event: WindowEvent)
{
    match event {
        KeyPressed(_key) => { reset(app, model) }
        _ => {}
    }
}

fn idle_event(app: &App, model: &mut Model, event: WindowEvent)
{
    if model.last_event != event
    {
        match event {
            KeyPressed(key) => { println!("Key Pressed"); handle_event(model, keypress_to_state(key)) }
            KeyReleased(key) => { println!("Key Released");handle_event(model, keyrelease_to_state(key)) }
            _ => {}
        }
        model.last_event = event;
    }
}

fn fire_missile(model: &mut Model)
{
    println!("Firing missile");
    let missile = Projectile{
        hit: false,
        position: model.player.position,
        rotation: model.player.rotation,
    };
    model.player.missile.push(missile);

    let sound = audrey::open("assets/space_clutter_laser.wav").expect("Not Found");

    model.stream.send( move |audio| {audio.event.push(sound)}).ok();
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

fn has_missile_hit_asteroid(missiles: &mut Vec<Projectile>, asteroid: &Asteroid, score: &mut u32, fragment: &mut Vec<Asteroid>, stream: &mut audio::Stream<Audio>) -> bool{
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

                let sound = audrey::open("assets/space_clutter_boom.wav").expect("Not Found");
                stream.send( move |audio| {audio.event.push(sound)}).ok();
               
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
    
    let r_tail_x = player.position.x + ((SPACESHIP_WIDTH / 2.0) * true_rotation.cos());
    let r_tail_y = player.position.y + (-(SPACESHIP_PEAK + SPACESHIP_TROUGH) * true_rotation.sin());
    
    let l_tail_x = player.position.x - ((SPACESHIP_WIDTH / 2.0) * true_rotation.cos());
    let l_tail_y = player.position.y + (-(SPACESHIP_PEAK + SPACESHIP_TROUGH) * true_rotation.sin());

    for asteroid in asteroids{
        /* Has peak hit asteroid? */
        let left_edge:bool = (true_x) > (asteroid.position.x - (asteroid.size / 2.0));
        let right_edge:bool = (true_x) < (asteroid.position.x + (asteroid.size / 2.0));
        let top_edge:bool = (true_y) < (asteroid.position.y + (asteroid.size / 2.0));
        let bottom_edge:bool = (true_y) > (asteroid.position.y - (asteroid.size / 2.0));
        
        let l_left_edge:bool = (l_tail_x) > (asteroid.position.x - (asteroid.size / 2.0));
        let l_right_edge:bool = (l_tail_x) < (asteroid.position.x + (asteroid.size / 2.0));
        let l_top_edge:bool = (l_tail_y) < (asteroid.position.y + (asteroid.size / 2.0));
        let l_bottom_edge:bool = (l_tail_y) > (asteroid.position.y - (asteroid.size / 2.0));
        
        let r_left_edge:bool = (l_tail_x) > (asteroid.position.x - (asteroid.size / 2.0));
        let r_right_edge:bool = (l_tail_x) < (asteroid.position.x + (asteroid.size / 2.0));
        let r_top_edge:bool = (l_tail_y) < (asteroid.position.y + (asteroid.size / 2.0));
        let r_bottom_edge:bool = (l_tail_y) > (asteroid.position.y - (asteroid.size / 2.0));

        if left_edge && right_edge && top_edge && bottom_edge
        {
            println!("CRASH!");
            has_hit = true;
        }
        else if l_left_edge && l_right_edge && l_top_edge && l_bottom_edge
        {
            println!("LEFT CRASH!");
            has_hit = true;
        }
        else if r_left_edge && r_right_edge && r_top_edge && r_bottom_edge
        {
            println!("RIGHT CRASH!");
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

    let mut thickness = ASTEROID_THICKNESS;
    if fragment{
        thickness *= 0.4;
    }
    let rotation = random_range(0.0, std::f32::consts::PI * 2.0);
    let mut asteroid = Asteroid{
        position: position,
        rotation: 0.0,
        rotation_speed: deg_to_rad(new_speed),
        size: new_size,
        points: Vec::new(),
        num_points: num_points,
        thickness: thickness,
        thrust_rotation: rotation,
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
    match model.game_state{
        State::Idle => idle_update(app, model, update),
        State::GameOver => gameover_update(app, model, update),
        State::Menu => menu_update(app, model, update),
        _ => assert!(false),
    }
}

fn gameover_update(_app: &App, _model: &mut Model, _update: Update) {
}

fn menu_update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    for asteroid in &mut model.asteroid{

        let true_rotation = asteroid.rotation + deg_to_rad(90.0); 
        let asteroid_size = asteroid.size / 2.0;

        if asteroid.position.x + (asteroid_size * true_rotation.cos()) > (win.right()){
            let new_pos_x = asteroid.position.x - WINDOW_SIZE.0 as f32;
            asteroid.position.x = new_pos_x;
        }
        else if asteroid.position.x + (asteroid_size * true_rotation.cos()) < (win.left()){
            let new_pos_x = asteroid.position.x + WINDOW_SIZE.0 as f32;
            asteroid.position.x = new_pos_x;
        }
        
        if asteroid.position.y + (asteroid_size * true_rotation.sin()) > win.top(){
            let new_pos_y = asteroid.position.y - WINDOW_SIZE.1 as f32;
            asteroid.position.y = new_pos_y;
        }
        else if asteroid.position.y + (asteroid_size * true_rotation.sin()) < win.bottom(){
            let new_pos_y = asteroid.position.y + WINDOW_SIZE.1 as f32;
            asteroid.position.y = new_pos_y;
        }
        
        asteroid.rotation += asteroid.rotation_speed;
        asteroid.position.x += -ASTEROID_SPEED * asteroid.thrust_rotation.sin();
        asteroid.position.y += ASTEROID_SPEED * asteroid.thrust_rotation.cos();
    }
    
    /* Generate new asteroid if needed */
    if model.asteroid.len() < model.difficulty.max_asteroids as usize
    {
        let new_pt =  new_point(&model.player, &model.asteroid);
        let asteroid = generate_asteroid(new_pt, 12, ASTEROID_MIN_SIZE, ASTEROID_MAX_SIZE, false);

        model.asteroid.push(asteroid);
    }
    
    let current_time:Instant = Instant::now();
    let duration = Duration::from_secs(1);

    if current_time.duration_since(model.tick) > duration {
        model.display_text ^= true;
        model.tick = Instant::now();
    }
}

fn idle_update(app: &App, model: &mut Model, update: Update) {
    let win = app.window_rect();
    let current_tick:Instant = Instant::now();

    if current_tick.duration_since(model.difficulty.tick) > model.difficulty.duration{
        model.difficulty.tick = Instant::now();
        model.difficulty.max_asteroids += 1;
        model.difficulty.max_asteroid_speed += 1.0;
        println!("Difficulty Increase!");
    }

    /* First, has the model crashed into anything? */
    let crashed = has_ship_hit_asteroid(&model.player, &model.asteroid);

    if crashed{
        model.game_state = State::GameOver;
    }

    model.player.rotation += model.player.rotation_inc;
    if model.player.thrust{
        model.player.thrust_rotation = model.player.rotation;
        model.player.thrust_counter = 0;
    }

    /* Handle wrapping across boundaries for space ship */
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

        let true_rotation = asteroid.rotation + deg_to_rad(90.0); 
        let asteroid_size = asteroid.size / 2.0;

        if asteroid.position.x + (asteroid_size * true_rotation.cos()) > (win.right()){
            let new_pos_x = asteroid.position.x - WINDOW_SIZE.0 as f32;
            asteroid.position.x = new_pos_x;
        }
        else if asteroid.position.x + (asteroid_size * true_rotation.cos()) < (win.left()){
            let new_pos_x = asteroid.position.x + WINDOW_SIZE.0 as f32;
            asteroid.position.x = new_pos_x;
        }
        
        if asteroid.position.y + (asteroid_size * true_rotation.sin()) > win.top(){
            let new_pos_y = asteroid.position.y - WINDOW_SIZE.1 as f32;
            asteroid.position.y = new_pos_y;
        }
        else if asteroid.position.y + (asteroid_size * true_rotation.sin()) < win.bottom(){
            let new_pos_y = asteroid.position.y + WINDOW_SIZE.1 as f32;
            asteroid.position.y = new_pos_y;
        }
        
        asteroid.rotation += asteroid.rotation_speed;
        asteroid.position.x += -ASTEROID_SPEED * asteroid.thrust_rotation.sin();
        asteroid.position.y += ASTEROID_SPEED * asteroid.thrust_rotation.cos();
    }
    
    let mut fragments:Vec<Asteroid> = Vec::new();
    assert_eq!(fragments.len(), 0);
    model.asteroid.retain(|asteroids| !has_missile_hit_asteroid(&mut model.player.missile, asteroids, &mut model.player.score, &mut fragments, &mut model.stream));

    for asteroid in fragments{
        model.asteroid.push(asteroid);
    }

    model.player.missile.retain(|missiles| !has_missile_hit_edge(missiles, app.window_rect()));
    
    for missile in &mut model.player.missile{
        missile.position.x += -MISSILE_SPEED * missile.rotation.sin();
        missile.position.y += MISSILE_SPEED * missile.rotation.cos();
    }

    /* Generate new asteroid if needed */
    if model.asteroid.len() < model.difficulty.max_asteroids as usize
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
//        StateEvents::SpaceKeyPress => { fire_missile(player) },
        _ => { /* Do nowt */}
    }
}

fn handle_event(model: &mut Model, event:StateEvents)
{
    match event{
        StateEvents::LeftKeyPress =>{model.player.rotation_inc = deg_to_rad(ANGLE_INC)},
        StateEvents::LeftKeyRelease => {model.player.rotation_inc = deg_to_rad(0.0)},
        StateEvents::RightKeyPress => {model.player.rotation_inc = deg_to_rad(-ANGLE_INC)},
        StateEvents::RightKeyRelease => {model.player.rotation_inc = deg_to_rad(0.0)},
        StateEvents::UpKeyPress => {model.player.thrust = true},
        StateEvents::UpKeyRelease => {model.player.thrust = false},
        StateEvents::SpaceKeyPress => { fire_missile(model) },
        _ => { /* Do nowt */}
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    match model.game_state{
        State::Idle => idle_view(app, model, frame),
        State::GameOver => gameover_view(app, model, frame),
        State::Menu => menu_view(app, model, frame),
        _ => assert!(false),
    }
}

fn gameover_view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);

    let actual_font: Font = Font::from_bytes(model.raw_font.clone()).unwrap();
    
    let game_over = "GAME OVER";
    draw.text(&game_over)
        .font(actual_font)
        .no_line_wrap()
        .font_size(65)
        .xy(pt2(0.0, win.top() - 75.0)); 
    
    /* Draw score */
    let score_font: Font = Font::from_bytes(model.score_font.clone()).unwrap();
    
    let score = format!("Score: {}", model.player.score);
    draw.text(&score)
        .font(score_font)
        .no_line_wrap()
        .font_size(60)
        .xy(pt2(0.0 , win.top() - 150.0));
    
    let credit_font: Font = Font::from_bytes(model.credit_font.clone()).unwrap();
    let anykey = format!("press any key to retry");
    draw.text(&anykey)
        .font(credit_font.clone())
        .no_line_wrap()
        .font_size(20)
        .xy(pt2(0.0, win.top() -250.0));


    let credits = format!("Coding + Music by T.L. '23");
    draw.text(&credits)
        .font(credit_font.clone())
        .no_line_wrap()
        .font_size(20)
        .xy(pt2(0.0, win.bottom() + 150.0));
    
    let credits = format!("llwyd.io");
    draw.text(&credits)
        .font(credit_font.clone())
        .no_line_wrap()
        .font_size(20)
        .xy(pt2(0.0, win.bottom() + 100.0));
    draw.to_frame(app, &frame).unwrap();
}

fn menu_view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);
    
    for asteroid in &model.asteroid{ 
        draw.polyline()
            .xy(asteroid.position)
            .weight(asteroid.thickness)
            .color(WHITE)
            .rotate(asteroid.rotation)
            .points(asteroid.points.clone());
        
        let true_rotation = asteroid.rotation + deg_to_rad(90.0 + 180.0);
        let asteroid_size = asteroid.size / 2.0;
        
        if asteroid.position.x + (asteroid.size) >= (win.right()){
            let new_pos_x = asteroid.position.x - WINDOW_SIZE.0 as f32;
            draw.polyline()
                .x_y(new_pos_x, asteroid.position.y)
                .weight(asteroid.thickness)
                .color(WHITE)
                .rotate(asteroid.rotation)
                .points(asteroid.points.clone());
        }
        else if asteroid.position.x - (asteroid.size) <= (win.left()){
            let new_pos_x = asteroid.position.x + WINDOW_SIZE.0 as f32;
            draw.polyline()
                .x_y(new_pos_x, asteroid.position.y)
                .weight(asteroid.thickness)
                .color(WHITE)
                .rotate(asteroid.rotation)
                .points(asteroid.points.clone());
        }
        
        if asteroid.position.y + (asteroid.size) >= (win.top()){
            let new_pos_y = asteroid.position.y - WINDOW_SIZE.1 as f32;
            draw.polyline()
                .x_y(asteroid.position.x, new_pos_y)
                .weight(asteroid.thickness)
                .color(WHITE)
                .rotate(asteroid.rotation)
                .points(asteroid.points.clone());
        }
        else if asteroid.position.y - (asteroid.size) <= (win.bottom()){
            let new_pos_y = asteroid.position.y + WINDOW_SIZE.1 as f32;
            draw.polyline()
                .x_y(asteroid.position.x, new_pos_y)
                .weight(asteroid.thickness)
                .color(WHITE)
                .rotate(asteroid.rotation)
                .points(asteroid.points.clone());
        }
    }



    let actual_font: Font = Font::from_bytes(model.raw_font.clone()).unwrap();
    let title = format!("SPACE CLUTTER");
    draw.text(&title)
        .font(actual_font)
        .no_line_wrap()
        .font_size(40)
        .xy(pt2(0.0, win.top() - 100.0));    

    if model.display_text{
        let credit_font: Font = Font::from_bytes(model.credit_font.clone()).unwrap();
        let anykey = format!("[ press any key to start ]");
        draw.text(&anykey)
            .font(credit_font)
            .no_line_wrap()
            .font_size(20)
            .xy(pt2(0.0, -100.0));
    }
    draw.to_frame(app, &frame).unwrap();
}

fn idle_view(app: &App, model: &Model, frame: Frame){
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
            .color(WHITE);
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
        
        let true_rotation = asteroid.rotation + deg_to_rad(90.0 + 180.0);
        let asteroid_size = asteroid.size / 2.0;
        
        if asteroid.position.x + (asteroid.size) >= (win.right()){
            let new_pos_x = asteroid.position.x - WINDOW_SIZE.0 as f32;
            draw.polyline()
                .x_y(new_pos_x, asteroid.position.y)
                .weight(asteroid.thickness)
                .color(WHITE)
                .rotate(asteroid.rotation)
                .points(asteroid.points.clone());
        }
        else if asteroid.position.x - (asteroid.size) <= (win.left()){
            let new_pos_x = asteroid.position.x + WINDOW_SIZE.0 as f32;
            draw.polyline()
                .x_y(new_pos_x, asteroid.position.y)
                .weight(asteroid.thickness)
                .color(WHITE)
                .rotate(asteroid.rotation)
                .points(asteroid.points.clone());
        }
        
        if asteroid.position.y + (asteroid.size) >= (win.top()){
            let new_pos_y = asteroid.position.y - WINDOW_SIZE.1 as f32;
            draw.polyline()
                .x_y(asteroid.position.x, new_pos_y)
                .weight(asteroid.thickness)
                .color(WHITE)
                .rotate(asteroid.rotation)
                .points(asteroid.points.clone());
        }
        else if asteroid.position.y - (asteroid.size) <= (win.bottom()){
            let new_pos_y = asteroid.position.y + WINDOW_SIZE.1 as f32;
            draw.polyline()
                .x_y(asteroid.position.x, new_pos_y)
                .weight(asteroid.thickness)
                .color(WHITE)
                .rotate(asteroid.rotation)
                .points(asteroid.points.clone());
        }
    }

    let actual_font: Font = Font::from_bytes(model.score_font.clone()).unwrap();
    
    let score = format!("Score: {}", model.player.score);
    draw.text(&score)
        .font(actual_font)
        .font_size(20)
        .no_line_wrap()
        .xy(pt2(win.right() - 120.0 , win.bottom() + 30.0));

    draw.to_frame(app, &frame).unwrap();
}
