use crate::models;

pub struct Builder {
    model_id: String,
    revision: Option<String>,
    device: candle_core::Device,
}

pub fn builder(model_id: &str) -> Builder {
    Builder {
        model_id: model_id.to_string(),
        revision: None,
        device: crate::utils::device().expect("Cannot deduce device"),
    }
}

impl Builder {
    pub fn with_revision(mut self, revision: &str) -> Self {
        self.revision = Some(revision.to_owned());
        self
    }

    pub fn build(self) -> models::Bert {
        let repo = match self.revision {
            Some(revision) => {
                hf_hub::Repo::with_revision(self.model_id, hf_hub::RepoType::Model, revision)
            }
            None => hf_hub::Repo::new(self.model_id, hf_hub::RepoType::Model),
        };
        let api = hf_hub::api::sync::ApiBuilder::new()
            .with_progress(false)
            .build()
            .expect("Cannot build HF-hub")
            .repo(repo);
        let config_path = api.get("config.json").expect("Cannot get config.json");
        let tokenizer_path = api
            .get("tokenizer.json")
            .expect("Cannot get tokenizer.json");
        let weights_path = api
            .get("model.safetensors")
            .expect("Cannot get model.safetensors");

        let config = std::fs::read_to_string(config_path).expect("Cannot read config.json");
        let config =
            serde_json::from_str(&config).expect("config.json is not compatible with BERT config");

        let mut tokenizer = tokenizers::Tokenizer::from_file(tokenizer_path).expect("");
        if let Some(pp) = tokenizer.get_padding_mut() {
            pp.strategy = tokenizers::PaddingStrategy::BatchLongest
        } else {
            let pp = tokenizers::PaddingParams {
                strategy: tokenizers::PaddingStrategy::BatchLongest,
                ..Default::default()
            };
            tokenizer.with_padding(Some(pp));
        }

        let vb = unsafe {
            candle_nn::VarBuilder::from_mmaped_safetensors(
                &[weights_path],
                candle_transformers::models::bert::DTYPE,
                &self.device,
            )
            .expect("Cannot map model.safetensors")
        };
        models::Bert {
            model: candle_transformers::models::bert::BertModel::load(vb, &config)
                .expect("Cannot load model"),
            tokenizer,
            device: self.device,
        }
    }
}
