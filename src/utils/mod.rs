use error_union::ErrorUnion;

pub mod usize_rect;

pub trait EngineCreate<R> {
    fn create(self) -> R;
}

#[derive(ErrorUnion, Debug)]
pub enum BufferError {
    Vertex(glium::vertex::BufferCreationError),
    Index(glium::index::BufferCreationError),
}
