pub trait ResizeHandler {
    fn on_resize(&mut self, width: u32, height: u32);
}
