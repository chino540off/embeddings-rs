pub struct Client {
    url: String,
    encoding: embeddings_utils::Encoding,
}

impl Client {
    pub fn as_base64(&mut self) -> &Self {
        self.encoding = embeddings_utils::Encoding::Base64;
        self
    }

    pub fn as_float(&mut self) -> &Self {
        self.encoding = embeddings_utils::Encoding::Float;
        self
    }

    pub fn embed(&self, sentences: Vec<String>) -> Vec<Vec<f32>> {
        vec![]
    }
}
