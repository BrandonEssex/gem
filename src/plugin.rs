pub trait Plugin {
    fn on_start(&mut self) {}
    fn on_input(&mut self, _input: &str) {}
    fn on_render(&mut self) {}
    fn on_exit(&mut self) {}
}
