use crate::models;

pub struct FactoryBuilder {
    model_id: Option<String>,
    revision: Option<String>,
    device: candle_core::Device,
}

impl Default for FactoryBuilder {
    fn default() -> Self {
        Self {
            model_id: None,
            revision: None,
            device: crate::utils::device().expect("Cannot deduce device"),
        }
    }
}

impl FactoryBuilder {
    pub fn with_model_id(mut self, model_id: &str) -> Self {
        self.model_id = Some(model_id.to_owned());
        self
    }
    pub fn with_revision(mut self, revision: &str) -> Self {
        self.revision = Some(revision.to_owned());
        self
    }
    pub fn build<'a>(self) -> Factory<'a> {
        Factory::new(
            self.model_id.expect("model_id is not set"),
            self.revision,
            self.device,
        )
    }
}

pub struct Factory<'a> {
    tokenizer: tokenizers::Tokenizer,
    config: candle_transformers::models::bert::Config,
    vb: candle_nn::VarBuilder<'a>,
    device: candle_core::Device,
}

impl Factory<'_> {
    pub fn builder() -> FactoryBuilder {
        FactoryBuilder::default()
    }

    fn new<'a>(
        model_id: String,
        revision: Option<String>,
        device: candle_core::Device,
    ) -> Factory<'a> {
        let repo = match revision {
            Some(revision) => {
                hf_hub::Repo::with_revision(model_id, hf_hub::RepoType::Model, revision)
            }
            None => hf_hub::Repo::new(model_id, hf_hub::RepoType::Model),
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

        Factory {
            tokenizer: tokenizer,
            config: serde_json::from_str(&config)
                .expect("config.json is not compatible with BERT config"),
            vb: unsafe {
                candle_nn::VarBuilder::from_mmaped_safetensors(
                    &[weights_path],
                    candle_transformers::models::bert::DTYPE,
                    &device,
                )
                .expect("")
            },
            device: device,
        }
    }

    pub fn make(self) -> models::Bert {
        models::Bert {
            model: candle_transformers::models::bert::BertModel::load(self.vb, &self.config)
                .expect("Cannot load model"),
            tokenizer: self.tokenizer,
            device: self.device,
        }
    }
}
