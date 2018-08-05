use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
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
        let mut n = self.inner;
        let mut m = other.inner;
        while n > 0 && m > 0 {
            let c = n & 255;
            let d = m & 255;
            n = n >> 8;
            m = m >> 8;
            if c != d {
                return n == m
            }
        }
        false
    }
}

#[derive(Debug,PartialEq)]
enum WordStatus { Unknown, Unmarked, Linked(Word) }

#[derive(Default)]
struct WordGraph {
    container : HashMap<Word,WordStatus>
}

use WordStatus::*;
impl WordGraph {
    pub fn add_word(&mut self, word : Word) {
        self.container.insert(word, Unmarked);
    }

    pub fn ladder(&self, _origin: Word, _target: Word) -> Vec<Word> {
        vec![]
    }

    fn get(&self, word : Word) -> WordStatus {
        match self.container.get(&word) {
            Some(&Unmarked) => Unmarked,
            Some(&Linked(w)) => Linked(w),
            _ => Unknown

        }
    }

    fn link(&mut self, word:Word, other:Word) {
        assert!(self.container.get(&word) == Some(&Unmarked));
        self.container.remove(&word);
        self.container.insert(word,Linked(other));
    }

    fn path(&self, word:Word) -> Vec<Word> {
        assert!(self.container.get(&word) != None);
        match self.container.get(&word) {
            Some(&Unmarked) => vec![word],
            Some(&Linked(next)) => [vec![word],self.path(next)].concat(),
            _ => vec![]
        }
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
            let mut word_graph : WordGraph = WordGraph::default();
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
            #[test]
            fn should_be_true_if_words_are_different_by_their_last_letter() {
                assert_eq!(true, Word::from("DOG").is_adjacent(Word::from("DOT")))
            }
            #[test]
            fn should_be_true_if_words_are_different_by_only_a_letter() {
                assert_eq!(true, Word::from("DOG").is_adjacent(Word::from("DIG")));
                assert_eq!(true, Word::from("DOG").is_adjacent(Word::from("FOG")));
            }
            #[test]
            fn should_be_false_if_words_are_different_by_more_than_one_letter() {
                assert_eq!(false, Word::from("DOG").is_adjacent(Word::from("DIB")));
                assert_eq!(false, Word::from("DOG").is_adjacent(Word::from("GOT")));
            }
            #[test]
            fn should_be_false_if_words_have_different_size() {
                assert_eq!(false, Word::from("DOGE").is_adjacent(Word::from("DOG")));
                assert_eq!(false, Word::from("BAT").is_adjacent(Word::from("BATH")));
            }
        }
    }

    mod word_graph {
        use super::*;
        use WordStatus::*;

        #[test]
        fn should_not_contain_a_word_when_empty() {
            let word_graph = WordGraph::default();
            assert_eq!(Unknown, word_graph.get(Word::from("DOG")))
        }
        #[test]
        fn should_contain_an_unmarked_word_that_was_added() {
            let mut word_graph: WordGraph = WordGraph::default();
            word_graph.add_word(Word::from("DOG"));
            assert_eq!(Unmarked, word_graph.get(Word::from("DOG")));
            assert_eq!(Unknown, word_graph.get(Word::from("CAT")))
        }
        #[test]
        fn should_mark_a_word_as_linked_to_another() {
            let mut word_graph: WordGraph = WordGraph::default();
            word_graph.add_word(Word::from("DOG"));
            word_graph.add_word(Word::from("FOG"));
            word_graph.link(Word::from("FOG"),Word::from("DOG"));
            assert_eq!(Linked (Word::from("DOG")), word_graph.get(Word::from("FOG")))
        }
        #[test]
        fn should_find_a_one_step_path_to_an_unmarked_word() {
            let mut word_graph: WordGraph = WordGraph::default();
            word_graph.add_word(Word::from("DOG"));
            let expected = vec![Word::from("DOG")];
            let result = word_graph.path(Word::from("DOG"));
            assert_eq!(result, expected)
        }
        #[test]
        fn should_find_a_two_step_path_to_a_unmarked_word() {
            let mut word_graph: WordGraph = WordGraph::default();
            word_graph.add_word(Word::from("DOG"));
            word_graph.add_word(Word::from("FOG"));
            word_graph.link(Word::from("FOG"),Word::from("DOG"));
            let expected = vec![Word::from("FOG"),Word::from("DOG")];
            let result = word_graph.path(Word::from("FOG"));
            assert_eq!(result, expected)
        }
    }

}
