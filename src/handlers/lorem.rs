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
            }).collect::<Vec<String>>().join("\u{001F}")
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
            }).collect::<Vec<String>>().join("\n------------\n\u{001F}")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::TestResult;
    use super::*;

    #[test]
    fn generate_ten_lorem_words() -> TestResult {
        match LoremHandler::gen_words(&LoremOptions {count: 10}) {
            Ok(result) => {
                let count = result.split(" ").map(|val| val.to_string()).collect::<Vec<String>>().len();
                if (count != 10) {
                    return Err(format!("The generated lorem has {count} words!"));
                }

                Ok(())
            },
            Err(err) => Err(String::from("Failed to generate lorem words!"))
        }
    }

    #[test]
    fn generate_ten_lorem_sentences() -> TestResult {
        match LoremHandler::gen_sentences(&LoremOptions {count: 10}) {
            Ok(result) => {
                println!("{result}");
                let count = result.split("\u{001F}").map(|val| val.to_string()).collect::<Vec<String>>().len();
                if (count != 10) {
                    return Err(format!("The generated lorem has {count} sentences!"));
                }

                Ok(())
            },
            Err(err) => Err(String::from("Failed to generate lorem sentences!"))
        }
    }

    #[test]
    fn generate_ten_lorem_paragraphs() -> TestResult {
        match LoremHandler::gen_paragraphs(&LoremOptions {count: 10}) {
            Ok(result) => {
                let count = result.split("\u{001F}").map(|val| val.to_string()).collect::<Vec<String>>().len();
                if (count != 10) {
                    return Err(format!("The generated lorem has {count} paragraphs!"));
                }

                Ok(())
            },
            Err(err) => Err(String::from("Failed to generate lorem paragraphs!"))
        }
    }
}
