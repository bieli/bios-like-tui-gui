use crate::app::BiosApp;

pub trait Renderer {
    fn run(&mut self, app: &mut BiosApp);
}

