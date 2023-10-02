use nannou::prelude::*;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

struct Model {
    rotation_angle: f32,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(640,480)
        //.decorations(false)
        .event(window_event)
        .build()
        .unwrap();
    Model {
        rotation_angle:0.0,
    }
}

fn event(_app: &App, _model: &mut Model, _event: Event) {
}

fn window_event(_app: &App, _model: &mut Model, event: WindowEvent)
{
    match event {
        KeyPressed(_key) => { println!("Key Pressed"); }
        KeyReleased(_key) => { println!("Key Released"); }
        _ => {}
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    //model.rotation_angle += 0.15;
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}

