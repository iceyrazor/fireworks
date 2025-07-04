// followed this video [Coding Challenge #27: Fireworks](https://youtu.be/CKeyIbT3vXI)
// but instead of p5 javascript. i did it in sdl2 rust
extern crate sdl2;

use sdl2::{gfx::primitives::DrawRenderer, rect::Rect};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::WindowCanvas;
use std::time::Duration;
use rand::{rng, Rng};


//HSV to rgv.
//i did not make this
//only translated it from my c version i found from a stranger
struct HSV {
    h: f64,
    s: f64,
    v: f64,
}
impl HSV{
    fn new(h: f64, s: f64, v: f64) -> Self{
        Self{
            h: h,
            s: s,
            v: v,
        }
    }
}

#[derive(Clone,Copy)]
struct IRGB {
    r: f64,
    g: f64,
    b: f64,
}
impl IRGB {
    fn new(r: f64, g: f64, b: f64) -> Self{
        Self{
            r: r,
            g: g,
            b: b,
        }
    }

    fn hsvtorgb(&mut self, hsv: &mut HSV){
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        if hsv.s == 0.0 {
            r = hsv.v;
            g = hsv.v;
            b = hsv.v;
        } else {
            let mut i: f64 = 0.0;
            let mut f: f64 = 0.0;
            let mut p: f64 = 0.0;
            let mut q: f64 = 0.0;
            let mut t: f64 = 0.0;

            if hsv.h == 360.0 {
                hsv.h = 0.0;
            } else {
                hsv.h = hsv.h / 60.0;

                i = hsv.h.trunc();
                f = hsv.h - i;

                p = hsv.v * (1.0 - hsv.s);
                q = hsv.v * (1.0 - (hsv.s * f));
                t = hsv.v * (1.0 - (hsv.s * (1.0 - f)));

                match i {
                    0.0=>{
                        r = hsv.v;
                        g = t;
                        b = p;
                    },
                    1.0=>{
                        r = q;
                        g = hsv.v;
                        b = p;
                    },
                    2.0=>{
                        r = p;
                        g = hsv.v;
                        b = t;
                    },
                    3.0=>{
                        r = p;
                        g = q;
                        b = hsv.v;
                    },
                    4.0=>{
                        r = t;
                        g = p;
                        b = hsv.v;
                    },
                    _=>{
                        r = hsv.v;
                        g = p;
                        b = q;
                    },
                }
            }
        }

        self.r = r * 255.0;
        self.g = g * 255.0;
        self.b = b * 255.0;
    }
}

fn magnitude(x: f64,y: f64) -> f64 {
    ((x).powi(2) + (y).powi(2)).sqrt()
}

struct PointG{
    x: f64,
    y: f64,
}
impl PointG{
    fn new(x: f64, y: f64) -> Self {
        Self{
            x: x,
            y: y,
        }
    }

    fn add(&mut self, addvec: &PointG){
        self.x = self.x + addvec.x;
        self.y = self.y + addvec.y;
    }

    fn mul(&mut self, addvec: &PointG){
        self.x = self.x * addvec.x;
        self.y = self.y * addvec.y;
    }

    fn set_mag(&mut self, mag: &f64){
        let getmag: f64 = magnitude(self.x, self.y);
        self.x = (self.x / getmag) * mag;
        self.y = (self.y / getmag) * mag;
    }
}

struct Gvars{
    mouse: PointG,
    screen_size: PointG,
}
impl Gvars{
    fn new(sw: f64, sh: f64) -> Self{
        Self {
            mouse: PointG::new(0.0,0.0),
            screen_size: PointG::new(sw,sh),
        }
    }
}

// Particle
struct Particle{
    pos: PointG,
    vel: PointG,
    acc: PointG,
    firework: bool,
    lifespan: i16,
    hue: IRGB,
}

impl Particle{
    fn new(x: f64, y: f64, hu: IRGB, firework: bool) -> Self{
        let mut initvel=PointG::new(0.0,rng().random_range(-18.0..-9.0));
        if firework==true {
            initvel=PointG::new(rng().random_range(-1.0..1.0),rng().random_range(-1.0..1.0));
            initvel.set_mag(&rng().random_range(1.0..10.0));
        }
        Self {
            pos: PointG::new(x,y),
            vel: initvel,
            acc: PointG::new(0.0,0.0),
            firework: firework,
            lifespan: 255,
            hue: hu,
        }
    }

    fn apply_force(&mut self, force: &PointG){
        self.acc.add(&force);
    }

    fn done(&mut self) -> bool{
        if self.lifespan <= 0 {
            return true
        }
        false
    }

    fn update(&mut self){
        if self.firework {
            self.vel.mul(&PointG::new(0.9,0.9));
            self.lifespan = self.lifespan - 4;

            if self.lifespan <= 0 {
                self.lifespan = 0;
            }
        }
        self.vel.add(&self.acc);
        self.pos.add(&self.vel);

        self.acc.x = 0.0;
        self.acc.y = 0.0;
    }

    fn show(&mut self, canvas: &mut WindowCanvas, gvars: &mut Gvars){
        if self.firework {
            canvas.filled_circle(self.pos.x as i16, self.pos.y as i16, 1, Color::RGBA(self.hue.r as u8,self.hue.g as u8,self.hue.b as u8,self.lifespan as u8)).err();
        } else {
            canvas.filled_circle(self.pos.x as i16, self.pos.y as i16, 3, Color::RGBA(self.hue.r as u8,self.hue.g as u8,self.hue.b as u8,255)).err();
        }
    }
}

// Firework
struct Firework{
    firework: Particle,
    exploded: bool,
    particles: Vec<Particle>,
    hue: IRGB,
}

impl Firework{
    fn new(x: f64, y: f64) -> Self{
        let mut hu = IRGB::new(0.0,0.0,0.0);
        hu.hsvtorgb(&mut HSV::new(rng().random_range(0.0..360.0),1.0,1.0));
        Self {
            firework: Particle::new(x,y,hu,false),
            exploded: false,
            particles: Vec::new(),
            hue: hu,
        }
    }

    fn done(&mut self) -> bool{
        if self.exploded && self.particles.len() == 0{
            return true
        }
        false
    }

    fn update(&mut self){
        if !self.exploded {
            self.firework.apply_force(&PointG::new(0.0,0.2));
            self.firework.update();

            if self.firework.vel.y >= 0.0 {
                self.exploded=true;
                self.explode();
            }
        }

        for i in (0..self.particles.len()).rev() {
            self.particles[i].apply_force(&PointG::new(0.0,0.2));
            self.particles[i].update();
            if self.particles[i].done() {
                self.particles.remove(i);
            }
        }
    }

    fn explode(&mut self){
        for _ in 0..100{
            let p = Particle::new(self.firework.pos.x,self.firework.pos.y,self.hue,true);
            self.particles.push(p);
        }
    }

    fn show(&mut self, canvas: &mut WindowCanvas, gvars: &mut Gvars){
        if !self.exploded {
            self.firework.show(canvas,gvars);
        }

        for obj in &mut self.particles{
            obj.show(canvas,gvars);
        }
    }
}


// Canvas Draw
fn draw(canvas: &mut WindowCanvas, obj_arr: &mut Vec<Firework>, gvars: &mut Gvars){
    canvas.set_draw_color(Color::RGBA(0,0,0,50));

    //swap these lines out if screen is glitchy
    canvas.fill_rect(Rect::new(0,0,800,800)).err();
    //canvas.clear();


    if rng().random_range(0.0..1.0) < 0.08 {
        let obj = Firework::new(rng().random_range(0.0..gvars.screen_size.x as f64), gvars.screen_size.y as f64);
        obj_arr.push(obj);
    }

    for i in (0..obj_arr.len()).rev() {
        obj_arr[i].update();
        obj_arr[i].show(canvas,gvars);
        if obj_arr[i].done() {
            obj_arr.remove(i);
        }
    }

    canvas.present();
}



fn main() {
    let sdl2_context = sdl2::init().unwrap();
    let video_subsystem = sdl2_context.video().unwrap();


    let width: u32=800;
    let height: u32=800;

    let window = video_subsystem.window("fireworks", width, height)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.default_pixel_format();
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

    let mut gvars = Gvars::new(width as f64, height as f64);

    let mut obj_arr: Vec<Firework> = Vec::new();


    let obj = Firework::new(rng().random_range(0.0..width as f64), height as f64);
    obj_arr.push(obj);


    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();

    let mut event_pump = sdl2_context.event_pump().unwrap();
    draw(&mut canvas, &mut obj_arr, &mut gvars);

    'running: loop {
        draw(&mut canvas, &mut obj_arr, &mut gvars);
        let state = event_pump.mouse_state();
        gvars.mouse.x=state.x() as f64;
        gvars.mouse.y=state.y() as f64;
        //draw(&mut canvas, &mut obj, &state.x(), &state.y());
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

// *~challenges
// shaped fireworks such as a heart
// variable amount of particles
