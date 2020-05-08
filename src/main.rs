trait Updater {
    fn update(&mut self);
}

trait Renderer {
    fn render(&mut self);
}

struct GameLoop<U, R>
where
    U: Updater,
    R: Renderer,
{
    updater: U,
    renderer: R,
}

impl<U, R> GameLoop<U, R>
where
    U: Updater,
    R: Renderer,
{
    fn tick(&mut self) {
        self.updater.update();
        self.renderer.render();
    }
}

struct Update(usize);
impl Updater for Update {
    fn update(&mut self) {
        self.0 += 1;
        println!("hello {}", self.0);
    }
}

struct Render(usize);
impl Renderer for Render {
    fn render(&mut self) {
        self.0 += 2;
        println!("hello {}", self.0);
    }
}

fn main() {
    let loops = 10;
    let render = Render(0);
    let update = Update(0);

    let mut game_loop = GameLoop {
        updater: update,
        renderer: render,
    };
    for i in 0..loops {
        game_loop.tick();
    }
}
