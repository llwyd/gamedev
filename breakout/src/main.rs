use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    player_pos: f32,
    ball_pos: f32,
    score: u32,
    key_press: Key,
    key_pressed: bool,
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
        ball_pos:0.0,
        score: 0,
        key_press: Key::Up,
        key_pressed: false,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {

}


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

fn update(_app: &App, model: &mut Model, _update: Update) {

    let increment = 4.0;
    if( model.key_pressed )
    {
        if model.key_press == Key::Left
        {
            model.player_pos -= increment;
        }
        else if model.key_press == Key::Right 
        {
            model.player_pos += increment;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);

    
    // Draw player
    draw.rect()
        .xy(pt2(model.player_pos,win.bottom()))
        .w_h(80.0,25.0)
        .color(WHITE);
    
    draw.to_frame(app, &frame).unwrap();
}

