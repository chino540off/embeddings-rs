use base64::Engine;

pub trait Decode {
    fn decode(&self) -> Vec<f32>;
}

impl Decode for Vec<f32> {
    fn decode(&self) -> Vec<f32> {
        self.clone()
    }
}

impl Decode for String {
    fn decode(&self) -> Vec<f32> {
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(self.as_str())
            .expect("Cannot decode base64");
        bytes
            .chunks(4)
            .map(|a| f32::from_ne_bytes(a.try_into().unwrap()))
            .collect()
    }
}

pub trait Encode<T> {
    fn encode(embeddings: Vec<f32>) -> T;
}

impl Encode<Vec<f32>> for Vec<f32> {
    fn encode(embeddings: Vec<f32>) -> Vec<f32> {
        embeddings
    }
}

impl Encode<String> for String {
    fn encode(embeddings: Vec<f32>) -> String {
        base64::engine::general_purpose::STANDARD.encode(
            embeddings
                .iter()
                .flat_map(|&f: &f32| f.to_ne_bytes())
                .collect::<Vec<_>>(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[rstest::rstest]
    #[case(vec![1.2, 3.4, 5.6])]
    #[case("mpmZP5qZWUAzM7NA".to_string())]
    fn decode(#[case] encoded: impl Decode) {
        assert_eq!(encoded.decode(), vec![1.2, 3.4, 5.6]);
    }

    #[rstest::rstest]
    #[case("mpmZP5qZWUAzM7NA".to_string())]
    #[case(vec![1.2,3.4,5.6])]
    fn encode<T: Encode<T> + Debug + PartialEq>(#[case] expected: T) {
        assert_eq!(T::encode(vec![1.2, 3.4, 5.6]), expected);
    }
}
