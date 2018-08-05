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
        if let Some(val) = self.container.get_mut(&word) {
            *val = Linked(other)
        }
    }

    fn path(&self, word:Word) -> Vec<Word> {
        assert!(self.container.get(&word) != None);
        match self.container.get(&word) {
            Some(&Unmarked) => vec![word],
            Some(&Linked(next)) => [vec![word],self.path(next)].concat(),
            _ => vec![]
        }
    }

    fn unmark_all(&mut self) {
        for val in self.container.values_mut() {
            *val = Unmarked
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
            let mut graph : WordGraph = WordGraph::default();
            for s in ["cat","cat","bat","bag","cog","cot","dog"].into_iter() {
                graph.add_word(Word::from(s));
            }
            let result = graph.ladder(Word::from("cat"),
                                           Word::from("dog"));
            let expected:Vec<Word> = vec!["cat","cot","cog","dog"].into_iter().map(Word::from).collect();
            assert_eq!(result, expected)
        }
    }

    mod word {
        use super::*;

        #[test]
        fn should_be_created_from_a_string_and_converted_into_a_string() {
            let dog = Word::from("dog");
            let cat = Word::from("cat");
            assert_eq!(String::from("dog"),dog.to_string());
            assert_eq!(String::from("cat"),cat.to_string())
        }
        #[test]
        fn should_be_comparable_for_equality() {
            let dog1= Word::from("dog");
            let dog2 = Word::from("dog");
            let cat = Word::from("cat");
            assert_eq!(dog1,dog2);
            assert_ne!(dog1,cat)
        }

        mod is_adjacent {
            use super::*;

            #[test]
            fn should_be_false_if_words_are_identical() {
                let dog = Word::from("dog");
                assert_eq!(false, dog.is_adjacent(dog))
            }
            #[test]
            fn should_be_true_if_words_are_different_by_their_last_letter() {
                let dog = Word::from("dog");
                let dot = Word::from("dot");
                assert_eq!(true, dog.is_adjacent(dot));
            }
            #[test]
            fn should_be_true_if_words_are_different_by_only_a_letter() {
                let dog = Word::from("dog");
                let dig = Word::from("dig");
                let fog = Word::from("fog");
                assert_eq!(true, dog.is_adjacent(dig));
                assert_eq!(true, dog.is_adjacent(fog));
            }
            #[test]
            fn should_be_false_if_words_are_different_by_more_than_one_letter() {
                let dog = Word::from("dog");
                let dib = Word::from("dib");
                let got = Word::from("got");
                assert_eq!(false, dog.is_adjacent(dib));
                assert_eq!(false, dog.is_adjacent(got));
            }
            #[test]
            fn should_be_false_if_words_have_different_size() {
                let dog = Word::from("dog");
                let doge = Word::from("doge");
                let bat = Word::from("bat");
                let bath = Word::from("bath");
                assert_eq!(false, doge.is_adjacent(dog));
                assert_eq!(false, bat.is_adjacent(bath));
            }
        }
    }

    mod word_graph {
        use super::*;
        use WordStatus::*;

        #[test]
        fn should_not_contain_a_word_when_empty() {
            let graph = WordGraph::default();
            let dog = Word::from("dog");
            assert_eq!(Unknown, graph.get(dog))
        }
        #[test]
        fn should_contain_an_unmarked_word_that_was_added() {
            let dog = Word::from("dog");
            let cat = Word::from("cat");
            let mut graph: WordGraph = WordGraph::default();
            graph.add_word(dog);
            assert_eq!(Unmarked, graph.get(dog));
            assert_eq!(Unknown, graph.get(cat))
        }
        #[test]
        fn should_mark_a_word_as_linked_to_another() {
            let dog = Word::from("dog");
            let fog = Word::from("fog");
            let mut graph: WordGraph = WordGraph::default();
            graph.add_word(dog);
            graph.add_word(fog);
            graph.link(fog,dog);
            assert_eq!(Linked(dog), graph.get(fog))
        }
        #[test]
        fn should_find_a_one_step_path_to_an_unmarked_word() {
            let dog = Word::from("dog");
            let mut graph: WordGraph = WordGraph::default();
            graph.add_word(dog);
            assert_eq!(vec![dog], graph.path(dog))
        }
        #[test]
        fn should_find_a_two_step_path_to_a_unmarked_word() {
            let dog = Word::from("dog");
            let fog = Word::from("fog");
            let mut graph: WordGraph = WordGraph::default();
            graph.add_word(dog);
            graph.add_word(fog);
            graph.link(fog,dog);
            assert_eq!(vec![fog,dog],graph.path(fog))
        }
        #[test]
        fn should_unmark_all_the_words_before_a_search() {
            let dog = Word::from("dog");
            let fog = Word::from("fog");
            let mut graph = WordGraph::default();
            graph.add_word(dog);
            graph.add_word(fog);
            graph.link(dog,fog);
            graph.link(fog,dog);
            assert_eq!(Linked(dog), graph.get(fog));
            assert_eq!(Linked(fog), graph.get(dog));
            graph.unmark_all();
            assert_eq!(Unmarked, graph.get(dog));
            assert_eq!(Unmarked, graph.get(fog));
        }
    }

}
