use nannou::prelude::*;

mod bird;
pub use crate::bird::Bird;

struct Model {
    bird:Vec<Bird>,
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
        bird: Vec::new(),
    };

    model.bird.push(Bird::new(pt2(0.0, 0.0)));
    model.bird.push(Bird::new(pt2(0.0, 50.0)));
    model.bird.push(Bird::new(pt2(0.0, -50.0)));
    model.bird.push(Bird::new(pt2(0.0, 75.0)));
    
    model
}

fn window_event(app: &App, model: &mut Model, event: WindowEvent)
{
}

fn event(_app: &App, _model: &mut Model, _event: Event) { }

fn is_bird_nearby(bird: &Bird, other_bird: &Bird) -> bool{
    let bird_radius = bird.radius();

    let dx_2:f32 = (other_bird.position().x - bird.position().x).pow(2);
    let dy_2:f32 = (other_bird.position().y - bird.position().y).pow(2);
    let other_bird_radius = (dx_2 + dy_2).sqrt();
    (other_bird_radius <= bird_radius)
}

fn separation(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{

    /* Calculate angles */
    let num_bird = other_birds.len();

    let mut average = pt2(0.0, 0.0);

    for i in 0..num_bird{
        average.x += other_birds[i].position().x;
        average.y += other_birds[i].position().y;
    }

    average.x /= num_bird as f32;
    average.y /= num_bird as f32;

    let angle = average.y.atan2(average.x);

    //println!("Avg:{:?} Angle:{}", average, rad_to_deg(angle));

    angle - std::f32::consts::PI
}

fn alignment(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{

    /* Calculate angles */
    let num_bird = other_birds.len();

    let mut average = 0.0;

    for i in 0..num_bird{
        average += other_birds[i].angle();
    }

    average /= num_bird as f32;
    average
}

fn cohesion(bird: &mut Bird, other_birds: &Vec <Bird>)->f32{
    
    /* Calculate angles */
    let num_bird = other_birds.len();

    let mut average = pt2(0.0, 0.0);

    for i in 0..num_bird{
        average.x += other_birds[i].position().x;
        average.y += other_birds[i].position().y;
    }

    average.x /= num_bird as f32;
    average.y /= num_bird as f32;

    let angle = average.y.atan2(average.x);

    //println!("Avg:{:?} Angle:{}", average, rad_to_deg(angle));

    angle
}

fn update(app: &App, model: &mut Model, update: Update) { 
    let win = app.window_rect();

    let num_bird = model.bird.len();
    for i in 0..num_bird{

        /* Collect nearby birds */
        let mut nearby:Vec<Bird> = Vec::new();
        for j in 0..num_bird{
            if(i != j)
            {
                if is_bird_nearby(&model.bird[i], &model.bird[j])
                {
                    nearby.push(model.bird[j]);
                }
            }
        }
        /* Handle Separation */
        let sep_angle = separation(&mut model.bird[i], &nearby);
    
        /* Handle Alignment */
        let align_angle = alignment(&mut model.bird[i], &nearby);

        /* Handle Cohesion */
        let coh_angle = cohesion(&mut model.bird[i], &nearby);
    }


    for bird in &mut model.bird{
        bird.update(&win);
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let win = app.window_rect();
    let draw = app.draw();

    for bird in &model.bird{
        //bird.draw_region(&draw);
    }
    model.bird[1].draw_region(&draw);

    for bird in &model.bird{
        bird.draw(&draw);
    }

    draw.background().color(BLACK);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .view(view)
        .run();
}

