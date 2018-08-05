
#[derive(Debug,PartialEq)]
struct Word {
    inner: u64
}

impl Word {
    pub fn from(_s: &str) -> Self {
        Word { inner: 0 }

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

}
