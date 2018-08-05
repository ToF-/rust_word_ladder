
#[derive(Debug,PartialEq)]
struct Word {
    inner: u64
}

impl Word {
    pub fn from(s: &str) -> Self {
        Word { inner: s.as_bytes()
                       .iter()
                       .fold(0, |acc,c| (acc << 8) + *c as u64 ) }
    }

    pub fn to_string(&self) -> String {
        let mut result:String = String::new();
        let mut n = self.inner;
        while n > 0 {
            let c:u8 = (n & 255) as u8;
            result.insert(0,c as char);
            n = n >> 8
        }
        result.clone()
    }

    fn is_adjacent(&self, other:Word) -> bool {
        false
    }
}

#[derive(Default)]
struct WordGraph {}

impl WordGraph {
    pub fn add_word(&self, _word : Word) {
    }

    pub fn ladder(&self, _origin: Word, _target: Word) -> Vec<Word> {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod ladder {
        use super::*;

        #[test]
        #[ignore]
        fn should_find_a_ladder_between_two_words() {
            let word_graph : WordGraph = WordGraph::default();
            for s in ["CAT","CAT","BAT","BAG","COG","COT","DOG"].into_iter() {
                word_graph.add_word(Word::from(s));
            }
            let result = word_graph.ladder(Word::from("CAT"),
                                           Word::from("DOG"));
            let expected:Vec<Word> = vec!["CAT","COT","COG","DOG"].into_iter().map(Word::from).collect();
            assert_eq!(result, expected)
        }
    }

    mod word {
        use super::*;

        #[test]
        fn should_be_created_from_a_string_and_converted_into_a_string() {
            assert_eq!(String::from("DOG"),Word::from("DOG").to_string());
            assert_eq!(String::from("CAT"),Word::from("CAT").to_string())
        }
        #[test]
        fn should_be_comparable_for_equality() {
            let w1 = Word::from("DOG");
            let w2 = Word::from("DOG");
            let w3 = Word::from("CAT");
            assert_eq!(w1,w2);
            assert_ne!(w1,w3)
        }

        mod is_adjacent {
            use super::*;

            #[test]
            fn should_be_false_if_words_are_identical() {
                let w = Word::from("DOG");
                assert_eq!(false, w.is_adjacent(Word::from("DOG")))
            }
        }
    }

}
