mod solutions;

trait Task {
    fn run(&self) -> Result<(), &'static str>;
}