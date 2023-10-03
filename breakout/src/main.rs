use nannou::prelude::*;

const PLAYER_SIZE: (f32,f32) = (80.0, 25.0);
const PLAYER_SPEED: f32 = 5.0;

const BALL_SIZE:(f32,f32) = (10.0, 10.0);
const BALL_SPEED: f32 = 1.5;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

struct Player{}
struct Ball{}

struct Model {
    player_pos: f32,
    ball_pos: Point2,
    score: u32,
    key_press: Key,
    key_pressed: bool,
    ball_dir_x: f32,
    ball_dir_y: f32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(640,480)
        .min_size(640,480)
        .max_size(640,480)
        //.decorations(false)
        .resizable(false)
        .event(window_event)
        .build()
        .unwrap();
    
    Model {
        player_pos:0.0,
        ball_pos:pt2(0.0,0.0),
        score: 0,
        key_press: Key::Up,
        key_pressed: false,
        ball_dir_x: 1.0,
        ball_dir_y: -1.0,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) { }

fn handle_keypress( key: Key, model: &mut Model )
{
       if key != model.key_press
       {
            println!("Key Press");
            model.key_pressed = true;
            model.key_press = key;
       }
}

fn handle_keyrelease( key: Key, model: &mut Model )
{
       if key == model.key_press
       {
            println!("Key Release");
            model.key_pressed = false;
            model.key_press = Key::Up;
       }
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent)
{
    match event {
        KeyPressed(key) => { handle_keypress(key, model) }
        KeyReleased(key) => { handle_keyrelease(key, model) }
        _ => {}
    }
}


fn update(app: &App, model: &mut Model, _update: Update) {

    let win = app.window_rect();
    // Handle ball movement
    model.ball_pos.x += BALL_SPEED * model.ball_dir_x;
    model.ball_pos.y += BALL_SPEED * model.ball_dir_y;
    
    // Handle input
    if model.key_pressed
    {
        if model.key_press == Key::Left
        {
            model.player_pos -= PLAYER_SPEED;
        }
        else if model.key_press == Key::Right 
        {
            model.player_pos += PLAYER_SPEED;
        }
    }

    // Handle Collision with player

    // Is it at bottom of screen?
    if model.ball_pos.y <= (win.bottom() + PLAYER_SIZE.1 - 5.0)
    {
        // Has it hit the player?
        if model.ball_pos.x <= ( model.player_pos + (PLAYER_SIZE.0 / 2.0) )
        {
            if model.ball_pos.x >= ( model.player_pos - (PLAYER_SIZE.0 / 2.0) )
            {
                model.ball_dir_y *= -1.0;
            }
        }
    }

    /* Side Walls */
    if model.ball_pos.x >= win.right() || model.ball_pos.x <= win.left()
    {
        model.ball_dir_x *= -1.0;
    }
    
    /* Roof */
    if model.ball_pos.y >= win.top()
    {
        model.ball_dir_y *= -1.0;
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);

    // Draw ball
    draw.rect()
        .xy(model.ball_pos)
        .w_h(BALL_SIZE.0, BALL_SIZE.1)
        .color(WHITE);
    
    // Draw player
    draw.rect()
        .xy(pt2(model.player_pos,win.bottom()))
        .w_h(PLAYER_SIZE.0, PLAYER_SIZE.1)
        .color(WHITE);
    
    draw.to_frame(app, &frame).unwrap();
}

