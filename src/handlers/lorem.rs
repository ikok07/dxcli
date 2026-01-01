use rand::{Rng};
use crate::cli::{LoremMethod, LoremOptions};
use crate::handlers::{Result};

pub struct LoremHandler {}

impl LoremHandler {
    pub fn handle_method(method: &LoremMethod) -> Result {
        match method {
            LoremMethod::Words {options} => Self::gen_words(options),
            LoremMethod::Sentences {options} => Self::gen_sentences(options),
            LoremMethod::Paragraphs {options} => Self::gen_paragraphs(options),
        }
    }

    fn gen_words(options: &LoremOptions) -> Result {
        let mut rng = rand::rng();
        return Ok(lipsum::lipsum_words_from_seed(options.count as usize, rng.random()));
    }

    fn gen_sentences(options: &LoremOptions) -> Result {
        let mut rng = rand::rng();
        return Ok(
            (1..=options.count).map(|_| {
                let num_words = rng.random_range(5..12);
                let seed = rng.random();
                return lipsum::lipsum_words_from_seed(num_words, seed);
            }).collect::<Vec<String>>().join(" ")
        )
    }

    fn gen_paragraphs(options: &LoremOptions) -> Result {
        let mut rng = rand::rng();
        return Ok(
            (1..=options.count).map(|_| {
                let num_sentences = rng.random_range(5..7);
                return (1..=num_sentences).map(|_| {
                    let num_words = rng.random_range(10..25);
                    let seed = rng.random();
                    return lipsum::lipsum_words_from_seed(num_words, seed);
                }).collect::<Vec<String>>().join(" ");
            }).collect::<Vec<String>>().join("\n------------\n")
        )
    }
}