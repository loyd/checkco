pub struct Path {
    parent: Option<Rc<Path>>,
    prop: PathPart,
}

enum Part {
    Prop(String),
    Index(u32),
}

impl Path {
    fn new<T: Into<Part>>(part: T) -> Path {
        Path { parent: None }
    }

    fn prop<T: Into<Part>>(part: T) -> Path {
        Path {}
    }
}
