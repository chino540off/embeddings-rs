mod bert;
mod factory;
mod utils;

pub use bert::Bert;
pub use factory::builder;

pub trait Embedder {
    fn embed(&self, sentences: Vec<String>) -> anyhow::Result<Vec<Vec<f32>>>;
}
