use nannou::prelude::*;

const WINDOW_SIZE: (f32,f32) = (640.0, 480.0);

struct Model { }

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(640,480)
        .min_size(640,480)
        .max_size(640,480)
        .decorations(true)
        .resizable(false)
        .event(window_event)
        .build()
        .unwrap();
    
    let mut model = Model { };

    model
}

fn event(_app: &App, _model: &mut Model, _event: Event) { }

fn window_event(app: &App, model: &mut Model, event: WindowEvent)
{
}

fn update(app: &App, model: &mut Model, update: Update) {
}

fn view(app: &App, model: &Model, frame: Frame){
}
