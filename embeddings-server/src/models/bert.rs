use crate::models::utils;
use crate::models::Embedder;

pub struct Bert {
    pub model: candle_transformers::models::bert::BertModel,
    pub tokenizer: tokenizers::Tokenizer,
    pub device: candle_core::Device,
}

impl Embedder for Bert {
    fn embed(&self, sentences: Vec<String>) -> anyhow::Result<Vec<Vec<f32>>> {
        let tokens = self
            .tokenizer
            .encode_batch(sentences, true)
            .map_err(anyhow::Error::msg)?;
        let token_ids = tokens
            .iter()
            .map(|tokens| {
                Ok(candle_core::Tensor::new(
                    tokens.get_ids().to_vec().as_slice(),
                    &self.device,
                )?)
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        let token_ids = candle_core::Tensor::stack(&token_ids, 0)?;
        let token_type_ids = token_ids.zeros_like()?;
        let embeddings = self.model.forward(&token_ids, &token_type_ids, None)?;
        let embeddings = utils::apply_max_pooling(&embeddings)?;
        let embeddings = utils::l2_normalize(&embeddings)?;
        Ok(embeddings.to_vec2::<f32>()?)
    }
}
