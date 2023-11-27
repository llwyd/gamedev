use nannou::prelude::*;

const WINDOW_SIZE: (u32, u32) = (640, 480);

#[derive(Copy,Clone)]
enum State{
    Idle, // Normal game 
//    GameOver,
//    Menu,
}

#[derive(Copy,Clone)]
enum StateEvents{
    Left_KeyPress,
    Left_KeyRelease,
    Right_KeyPress,
    Right_KeyRelease,
    Up_KeyPress,
    Up_KeyRelease,
}

struct Player{
    position: Point2,
    rotation: f32,
    score: u32,
}

struct Model {
    player: Player,
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
                score: 0,
        }
    };

    model
}

fn event(_app: &App, _model: &mut Model, _event: Event) { }

fn window_event(app: &App, model: &mut Model, event: WindowEvent)
{
}

fn update(app: &App, model: &mut Model, update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();
    let draw = app.draw();
    draw.background().color(BLACK);

    let point1 = pt2(-15.0, 0.0);
    let point2 = pt2(0.0, 6.5);
    let point3 = pt2(15.0, 0.0);
    let point4 = pt2(0.0, 39.0);

    draw.quad()
        .points(point1,point2,point3,point4)
        .rotate(model.player.rotation)
        .color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
