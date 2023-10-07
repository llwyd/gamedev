use nannou::prelude::*;

const PLAYER_SIZE: (f32,f32) = (80.0, 25.0);
const PLAYER_SPEED: f32 = 10.0;

const BALL_SIZE:(f32,f32) = (10.0, 10.0);
const BALL_DEFAULT_SPEED: f32 = 1.5;
const BALL_SPEED_INC: f32 = 0.25;

const BRICK_SIZE: (f32,f32) = (128.0, 48.0);

const NUM_ROWS: u8 = 5;
const NUM_COLS: u8 = 5;

#[derive(Copy,Clone)]
enum State{
    Idle, // Normal game 
    GameOver,
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

struct Player{
    position: f32,
    score: u32,
}

struct Ball{
    position: Point2,
    dir: Point2,
    speed: f32,
}

struct Brick{
    position: Point2,
    colour: u8, // This is shite
}

struct Model {
    state: State,
    player: Player,
    key_press: Key,
    key_pressed: bool,
    ball: Ball,
    bricks: Vec<Brick>,
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
    
    let mut model = Model {
        state: State::Idle,
        player: Player{
            position: 0.0,
            score: 0,
        },
        key_press: Key::Up,
        key_pressed: false,
        ball: Ball{
            position: pt2(0.0,-20.0),
            dir: pt2(1.0,-1.0),
            speed: BALL_DEFAULT_SPEED,
        },
        bricks: Vec::new(),
    };

    // Populate vector with bricks
    let win = app.window_rect();
    let mut row_pos = win.left() + (BRICK_SIZE.0 / 2.0);
    for _i in 0..NUM_ROWS{
        let mut col_pos = win.top() - (BRICK_SIZE.1 / 2.0);
        for j in 0..NUM_COLS{
            let brick = Brick {
                position: pt2(row_pos, col_pos),
                colour: j,
            };
            model.bricks.push(brick);
            col_pos -= BRICK_SIZE.1;
        }
        row_pos += BRICK_SIZE.0;
    }

    model
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

fn reset(app: &App, model: &mut Model){
    model.player.position = 0.0;
    model.player.score = 0;
    model.key_press = Key::Up;
    model.key_pressed = false;
    model.ball.position = pt2(0.0,-20.0);
    model.ball.dir = pt2(1.0,-1.0);
    model.ball.speed = BALL_DEFAULT_SPEED;
    model.bricks.clear();
    
    // Populate vector with bricks
    let win = app.window_rect();
    let mut row_pos = win.left() + (BRICK_SIZE.0 / 2.0);
    for _i in 0..NUM_ROWS{
        let mut col_pos = win.top() - (BRICK_SIZE.1 / 2.0);
        for j in 0..NUM_COLS{
            let brick = Brick {
                position: pt2(row_pos, col_pos),
                colour: j,
            };
            model.bricks.push(brick);
            col_pos -= BRICK_SIZE.1;
        }
        row_pos += BRICK_SIZE.0;
    }
    
    model.state = State::Idle;
}

fn gameover_event(app: &App, model: &mut Model, event: WindowEvent)
{
    match event {
        KeyReleased(_key) => { reset(app, model) }
        _ => {}
    }
}

fn idle_event(_app: &App, model: &mut Model, event: WindowEvent)
{
    match event {
        KeyPressed(key) => { handle_keypress(key, model) }
        KeyReleased(key) => { handle_keyrelease(key, model) }
        _ => {}
    }
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent)
{
    match model.state{
        State::Idle => idle_event(app, model, event),
        State::GameOver => gameover_event(app, model, event),
    }
}

fn update(app: &App, model: &mut Model, update: Update) { 
    match model.state{
        State::Idle => idle_update(app, model, update),
        State::GameOver => gameover_update(app, model, update),
    }
}

fn gameover_update(_app: &App, _model: &mut Model, _update: Update) {

}

fn idle_update(app: &App, model: &mut Model, _update: Update) {

    let win = app.window_rect();
    // Handle ball movement
    model.ball.position.x += model.ball.speed * model.ball.dir.x;
    model.ball.position.y += model.ball.speed * model.ball.dir.y;
   
    /* Repopulate bricks if needed */
    if model.bricks.is_empty() {
        if model.ball.position.y - (BALL_SIZE.0 / 2.0 ) <= 0.0 - BRICK_SIZE.1{
            // Populate vector with bricks
            let win = app.window_rect();
            let mut row_pos = win.left() + (BRICK_SIZE.0 / 2.0);
            for _i in 0..NUM_ROWS{
                let mut col_pos = win.top() - (BRICK_SIZE.1 / 2.0);
                for j in 0..NUM_COLS{
                    let brick = Brick {
                        position: pt2(row_pos, col_pos),
                        colour: j,
                    };
                    model.bricks.push(brick);
                    col_pos -= BRICK_SIZE.1;
                }
                row_pos += BRICK_SIZE.0;
            }

        }
    }


    // Handle input
    if model.key_pressed
    {
        if model.key_press == Key::Left
        {
            model.player.position -= PLAYER_SPEED;
        }
        else if model.key_press == Key::Right 
        {
            model.player.position += PLAYER_SPEED;
        }
    }

    // Handle Collision with player

    // Is it at bottom of screen?
    if model.ball.position.y - (BALL_SIZE.0/2.0) <= (win.bottom() - (PLAYER_SIZE.1 / 2.0)){
            model.state = State::GameOver; 
    }
    else if model.ball.position.y - (BALL_SIZE.0/2.0) <= (win.bottom() + (PLAYER_SIZE.1 / 2.0))
    {
        // Has it hit the player?
        if model.ball.position.x <= ( model.player.position + (PLAYER_SIZE.0 / 2.0) )
        {
            if model.ball.position.x >= ( model.player.position - (PLAYER_SIZE.0 / 2.0) )
            {
                model.ball.dir.y *= -1.0;
                model.ball.position.y = win.bottom() + (PLAYER_SIZE.1 / 2.0) + (BALL_SIZE.1/2.0);
            }
        }
    }

    /* Side Walls */
    if model.ball.position.x >= win.right()
    {
        model.ball.dir.x *= -1.0;
        model.ball.position.x = win.right() - (BALL_SIZE.1/2.0);
    }
    if model.ball.position.x <= win.left()
    {
        model.ball.dir.x *= -1.0;
        model.ball.position.x = win.left() + (BALL_SIZE.1/2.0);
    }
    
    /* Roof */
    if model.ball.position.y >= win.top()
    {
        model.ball.dir.y *= -1.0;
    }

    /* Has it hit a brick? */
    model.bricks.retain(|i| not_collided_with_brick(i, &mut model.ball, &mut model.player));
}

fn not_collided_with_brick(brick: &Brick, ball: &mut Ball, player: &mut Player) -> bool {

    let mut ret = true;
    if ball.position.y + (BALL_SIZE.1 / 2.0) >= ( brick.position.y - (BRICK_SIZE.1 / 2.0) )
    {
        let y_diff = ball.position.y - brick.position.y;
        if ball.position.x <= ( brick.position.x + (BRICK_SIZE.0 / 2.0) )
        {
            let x_r_diff = ( brick.position.x + (BRICK_SIZE.0 / 2.0) ) - ball.position.x;
            if ball.position.x >= ( brick.position.x - (BRICK_SIZE.0 / 2.0) )
            {
                // There has been a brick collision
                let x_l_diff = ball.position.x - ( brick.position.x - (BRICK_SIZE.0 / 2.0));
                ball.speed += BALL_SPEED_INC;
                player.score += 1;
                ret = false;
               
                // clip ball to shortest side
                if y_diff < x_r_diff{
                    if y_diff < x_l_diff{
                        ball.position.y = brick.position.y - (BRICK_SIZE.1 / 2.0) - (BALL_SIZE.1 / 2.0) ;
                        ball.dir.y *= -1.0;
                    }
                }
                else if x_l_diff < x_r_diff{
                    ball.position.x = brick.position.x - (BRICK_SIZE.0 / 2.0) - (BALL_SIZE.0 / 2.0) ;
                    ball.dir.x *= -1.0;
                }
                else
                {
                    ball.position.x = brick.position.x + (BRICK_SIZE.0 / 2.0) + (BALL_SIZE.0 / 2.0) ;
                    ball.dir.x *= -1.0;
                }
            }
        }
    }

    ret
}

fn view(app: &App, model: &Model, frame: Frame){
    match model.state{
        State::Idle => idle_view(app, model, frame),
        State::GameOver => gameover_view(app, model, frame),
    }
}

fn gameover_view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);
    
    let game_over = "GAME OVER";
    draw.text(&game_over)
        .font_size(75)
        .xy(pt2(-120.0, win.top() - 100.0));
    
    /* Draw score */
    let score = format!("Score {}", model.player.score);
    draw.text(&score)
        .font_size(60)
        .xy(pt2(120.0 , win.top() - 100.0));
    
    let anykey = format!("press any key to retry");
    draw.text(&anykey)
        .font_size(20)
        .xy(pt2(0.0, -100.0));

    draw.to_frame(app, &frame).unwrap();
}

fn idle_view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);

    // Draw ball
    draw.rect()
        .xy(model.ball.position)
        .w_h(BALL_SIZE.0, BALL_SIZE.1)
        .color(WHITE);
    
    // Draw player
    draw.rect()
        .xy(pt2(model.player.position,win.bottom()))
        .w_h(PLAYER_SIZE.0, PLAYER_SIZE.1)
        .color(WHITE);
   
    // Draw brick(s)    
    let colours = [PLUM, BLUE, GREEN, YELLOW, RED];
    for brick in &model.bricks{
        draw.rect()
            .xy(brick.position)
            .w_h(BRICK_SIZE.0, BRICK_SIZE.1)
            .color(colours[brick.colour as usize]);
    }

    /* Draw score */
    let score = format!("Score {}", model.player.score);
    draw.text(&score)
        .font_size(40)
        .xy(pt2(win.right() - 80.0 , win.bottom() + 30.0));

    draw.to_frame(app, &frame).unwrap();
}

