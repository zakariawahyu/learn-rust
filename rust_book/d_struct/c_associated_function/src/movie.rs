#[derive(Debug)]
pub struct Movie {
    title: String,
    description: String,
    rating: f32,
}

impl Movie {
    pub fn new(title: String, description: String, rating: f32) -> Self {
        Self { title, description, rating }
    }
}