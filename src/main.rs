use mortal::{Color, Event, Key, PrepareConfig, Screen, Style};
use std::{thread, time};
trait Updater {
    fn update(&mut self);
}

trait Renderer {
    fn render(&mut self, scrren: &Screen);
}

struct GameLoop<U, R>
where
    U: Updater,
    R: Renderer,
{
    updater: U,
    renderer: R,
    screen: Screen,
}

impl<U, R> GameLoop<U, R>
where
    U: Updater,
    R: Renderer,
{
    pub fn new(updater: U, renderer: R) -> Self {
        let screen = Screen::new(PrepareConfig {
            ..PrepareConfig::default()
        })
        .unwrap();
        Self {
            updater,
            renderer,
            screen,
        }
    }

    fn tick(&mut self) {
        self.updater.update();
        self.renderer.render(&self.screen);
    }
}

struct Update(usize);
impl Updater for Update {
    fn update(&mut self) {
        self.0 += 1;
    }
}

struct Render(usize);
impl Renderer for Render {
    fn render(&mut self, screen: &Screen) {
        self.0 += 2;
        screen.clear_screen();
        screen.write_at((0, 0), "Reading input. Press 'g' to stop.");
        screen.refresh().unwrap();
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn main() {
    let loops = 10;
    let render = Render(0);
    let update = Update(0);
    let mut game_loop = GameLoop::new(update, render);
    for i in 0..loops {
        game_loop.tick();
    }
}
