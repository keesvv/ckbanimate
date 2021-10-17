pub trait Animation {
    fn start(&mut self);
    fn stop(&mut self) {}
    fn repeat(&mut self) {
        loop {
            self.start();
        }
    }
}
