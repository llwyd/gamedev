use nannou::prelude::*;

#[derive(Copy, Clone)]
pub struct Bird{
    xy: Point2,
    angle: f32,
    sep_angle: f32,
    align_angle: f32,
    coh_angle: f32,
}

impl Bird{
    const MOV_INC:f32 = 0.02;
    const BIRD_HEIGHT:f32 = 30.0;
    const BIRD_WIDTH_2:f32 = 10.0;
    const BIRD_REGION_RADIUS:f32 = 90.0;
    
    const BIRD_SEPARATION_RADIUS:f32 = 45.0;

    pub fn new(position:Point2) -> Bird{
        Bird{
            xy: position,
            angle: deg_to_rad(90.0),
            sep_angle: deg_to_rad(90.0),
            align_angle: deg_to_rad(90.0),
            coh_angle: deg_to_rad(90.0),
        }
    }

    pub fn angle(&self) -> f32{
        self.angle
    }

    pub fn set_rotation(&mut self, new_rotation:f32){
        self.angle = new_rotation;
    }
    
    pub fn set_separation(&mut self, new_rotation:f32){
        self.sep_angle = new_rotation;
    }
    
    pub fn set_alignment(&mut self, new_rotation:f32){
        self.align_angle = new_rotation;
    }
    
    pub fn set_cohesion(&mut self, new_rotation:f32){
        self.coh_angle = new_rotation;
    }

    pub fn radius(&self) -> f32{
        Self::BIRD_REGION_RADIUS
    }
    
    pub fn separation_radius(&self) -> f32{
        Self::BIRD_SEPARATION_RADIUS
    }

    pub fn draw_region(&self, draw: &Draw)
    {
        draw.ellipse()
            .color(GREY)
            .x_y(self.xy.x, self.xy.y)
            .w(Self::BIRD_REGION_RADIUS * 2.0)
            .h(Self::BIRD_REGION_RADIUS * 2.0);
    }
    
    pub fn draw_sep_region(&self, draw: &Draw)
    {
        draw.ellipse()
            .color(CYAN)
            .x_y(self.xy.x, self.xy.y)
            .w(Self::BIRD_SEPARATION_RADIUS * 2.0)
            .h(Self::BIRD_SEPARATION_RADIUS * 2.0);
    }

    pub fn draw(&self, draw: &Draw)
    {
        draw.tri()
            .points(pt2(0.0,Self::BIRD_HEIGHT / 2.0),pt2(-Self::BIRD_WIDTH_2, -Self::BIRD_HEIGHT / 2.0),pt2(Self::BIRD_WIDTH_2, -Self::BIRD_HEIGHT / 2.0))
            .x_y(self.xy.x, self.xy.y)
            .rotate(self.angle)
            .color(WHITE);
    }

    pub fn update(&mut self, win: &Rect<f32>)
    {
        let sep = pt2(-Self::MOV_INC * self.sep_angle.sin(), Self::MOV_INC * self.sep_angle.cos());
        let align = pt2(-Self::MOV_INC * self.align_angle.sin(), Self::MOV_INC * self.align_angle.cos());
        let coh = pt2(-Self::MOV_INC * self.coh_angle.sin(), Self::MOV_INC * self.coh_angle.cos());


        /* Add new vectors */
        let mut new_xy = pt2(0.0, 0.0);
        new_xy.x = self.xy.x + sep.x + align.x + coh.x;
        new_xy.y = self.xy.y + sep.y + align.y + coh.y;


        self.angle = new_xy.y.atan2(new_xy.x) - self.xy.y.atan2(self.xy.x);

//        self.angle = (self.sep_angle + self.align_angle + self.coh_angle) / 1.0;

        self.xy = new_xy;
//        self.xy.x += sep.x + align.x + coh.x;
//        self.xy.y += sep.y + align.y + coh.y;
        println!("{:?}", self.xy);
        //self.xy.x += -Self::MOV_INC * self.angle.sin();
        //self.xy.y += Self::MOV_INC * self.angle.cos();

        if self.xy.x >= win.right() as f32{
            self.xy.x -= win.wh().x;
        }
        else if self.xy.x <= win.left() as f32{
            self.xy.x += win.wh().x;
        }
        
        if self.xy.y >= win.top() as f32{
            self.xy.y -= win.wh().y;
        }
        else if self.xy.y <= win.bottom() as f32{
            self.xy.y += win.wh().y;
        } 
    }

    pub fn position(&self) -> Point2{
        self.xy
    }
}
