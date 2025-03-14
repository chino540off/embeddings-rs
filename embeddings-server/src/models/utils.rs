pub fn apply_max_pooling(embeddings: &candle_core::Tensor) -> anyhow::Result<candle_core::Tensor> {
    Ok(embeddings.max(1)?)
}

pub fn l2_normalize(embeddings: &candle_core::Tensor) -> anyhow::Result<candle_core::Tensor> {
    Ok(embeddings.broadcast_div(&embeddings.sqr()?.sum_keepdim(1)?.sqrt()?)?)
}
