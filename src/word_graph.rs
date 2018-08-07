use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
pub struct Word {
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

#[derive(Debug,PartialEq,Eq)]
enum WordStatus { Unknown, Unmarked, Target, NextTo(Word) }

#[derive(Default,Debug)]
pub struct WordGraph {
    container : HashMap<Word,WordStatus>
}

use self::WordStatus::*;
impl WordGraph {
    pub fn add_word(&mut self, word : Word) {
        self.container.insert(word, Unmarked);
    }

    fn get_word(&self, word : Word) -> WordStatus {
        match self.container.get(&word) {
            Some(&Unmarked) => Unmarked,
            Some(&NextTo(w)) => NextTo(w),
            Some(&Target) => Target,
            _ => Unknown

        }
    }

    fn target(&mut self, word:Word) {
        assert!(self.container.get(&word) == Some(&Unmarked));
        if let Some(val) = self.container.get_mut(&word) {
            *val = Target
        }
    }

    fn link(&mut self, word:Word, other:Word) {
        assert!(self.container.get(&word) == Some(&Unmarked));
        if let Some(val) = self.container.get_mut(&word) {
            *val = NextTo(other)
        }
    }

    fn path(&self, word:Word) -> Vec<Word> {
        assert!(self.container.get(&word) != None);
        match self.container.get(&word) {
            Some(&Target) => vec![word],
            Some(&NextTo(next)) => [vec![word],self.path(next)].concat(),
            _ => vec![]
        }
    }

    fn unmark_all(&mut self) {
        for val in self.container.values_mut() {
            *val = Unmarked
        }
    }

    fn adjacents(&self, word:Word) -> Vec<Word> {
        self.container.keys().filter(|w| w.is_adjacent(word)).map(|w| *w).collect()
    }

    fn unvisited_adjacents(&self, word:Word) -> Vec<Word> {
        self.adjacents(word).into_iter().filter(|w| self.get_word(*w) == Unmarked).collect()
    }

    fn search(&mut self, target:Word, origin:Word) {
        let mut to_visit:VecDeque<Word> = VecDeque::default();
        self.unmark_all();
        self.target(target);
        to_visit.push_back(target);
        while let Some(word) = to_visit.pop_front() {
            if word == origin {
                return
            } else {
                for next in self.unvisited_adjacents(word) {
                    if self.get_word(next) == Unmarked {
                        to_visit.push_back(next);
                        self.link(next, word)
                    }
                }
            }
        }
    }
    pub fn ladder(&mut self, origin : Word, target: Word ) -> Vec<Word> {
        if self.get_word(origin) == Unknown
            || self.get_word(target) == Unknown {
            return vec![]
        }
        self.unmark_all();
        self.target(target);
        self.search(target, origin);
        self.path(origin)
    }
}
impl FromIterator<Word> for WordGraph {
    fn from_iter<W: IntoIterator<Item=Word>>(iter: W) -> Self {
        let mut graph = WordGraph::default();

        for word in iter {
            graph.add_word(word)
        }

        graph
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    mod ladder {
        use super::*;

        #[test]
        fn should_find_a_ladder_between_two_words() {
            let words = ["cat","cat","bat","bag","cog","cot","dog"].into_iter().map(|s| Word::from(s));
            let mut graph:WordGraph = words.collect();
            let result = graph.ladder(Word::from("cat"), Word::from("dog"));
            let expected:Vec<Word> = vec!["cat","cot","cog","dog"].into_iter().map(Word::from).collect();
            assert_eq!(result, expected)
        }
        #[test]
        fn should_return_an_empty_list_if_origin_word_is_not_present() {
            let words = ["cat","cat","bat","bag","cog","cot","dog"].into_iter().map(|s| Word::from(s));
            let mut graph:WordGraph = words.collect();
            let result = graph.ladder(Word::from("foo"), Word::from("dog"));
            assert_eq!(result, vec![])
        }
        #[test]
        fn should_return_an_empty_list_if_target_word_is_not_present() {
            let words = ["cat", "cat", "bat", "bag", "cog", "cot", "dog"].into_iter().map(|s| Word::from(s));
            let mut graph: WordGraph = words.collect();
            let result = graph.ladder(Word::from("cat"), Word::from("qux"));
            assert_eq!(result, vec![])
        }
        #[test]
        fn should_return_an_empty_list_if_no_path_can_be_found() {
            let words = ["cat", "cat", "bat", "bag", "cog", "cot", "qux"].into_iter().map(|s| Word::from(s));
            let mut graph: WordGraph = words.collect();
            let result: Vec<String> = graph.ladder(Word::from("cat"), Word::from("qux"))
                .into_iter().map(|w| w.to_string()).collect();
            let expected:Vec<String> = vec![];
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

        #[test]
        fn should_not_contain_a_word_when_empty() {
            let graph = WordGraph::default();
            let dog = Word::from("dog");
            assert_eq!(Unknown, graph.get_word(dog))
        }
        #[test]
        fn should_contain_an_unmarked_word_that_was_added() {
            let dog = Word::from("dog");
            let cat = Word::from("cat");
            let mut graph: WordGraph = WordGraph::default();
            graph.add_word(dog);
            assert_eq!(Unmarked, graph.get_word(dog));
            assert_eq!(Unknown, graph.get_word(cat))
        }
        #[test]
        fn should_mark_a_word_as_the_target() {
            let dog = Word::from("dog");
            let fog = Word::from("fog");
            let mut graph: WordGraph = WordGraph::default();
            graph.add_word(dog);
            graph.add_word(fog);
            graph.target(dog);
            assert_eq!(Target, graph.get_word(dog));
        }
        #[test]
        fn should_mark_a_word_as_linked_to_another() {
            let dog = Word::from("dog");
            let fog = Word::from("fog");
            let mut graph: WordGraph = WordGraph::default();
            graph.add_word(dog);
            graph.add_word(fog);
            graph.link(fog, dog);
            assert_eq!(NextTo(dog), graph.get_word(fog))
        }
        #[test]
        fn should_find_a_one_step_path_to_a_target_word() {
            let dog = Word::from("dog");
            let mut graph: WordGraph = WordGraph::default();
            graph.add_word(dog);
            graph.target(dog);
            assert_eq!(vec![dog], graph.path(dog))
        }
        #[test]
        fn should_find_a_two_step_path_to_a_target_word() {
            let dog = Word::from("dog");
            let fog = Word::from("fog");
            let mut graph: WordGraph = WordGraph::default();
            graph.add_word(dog);
            graph.add_word(fog);
            graph.target(dog);
            graph.link(fog, dog);
            assert_eq!(vec![fog, dog], graph.path(fog))
        }
        #[test]
        fn should_unmark_all_the_words_before_a_search() {
            let dog = Word::from("dog");
            let fog = Word::from("fog");
            let mut graph:WordGraph = vec!["dog","fog","cog","cot","cat"]
                .into_iter().map(Word::from).collect();
            graph.link(dog, fog);
            graph.target(fog);
            graph.unmark_all();
            assert_eq!(Unmarked, graph.get_word(dog));
            assert_eq!(Unmarked, graph.get_word(fog));
        }
        #[test]
        fn should_search_the_graph_starting_from_a_target_until_origin_is_found() {
            let dog = Word::from("dog");
            let cog = Word::from("cog");
            let cot = Word::from("cot");
            let fog = Word::from("fog");
            let cat = Word::from("cat");
            let mut graph:WordGraph = vec![dog,fog,cog,cot,cat]
                .into_iter().collect();
            graph.search(cat, dog);
            assert_eq!(Target, graph.get_word(cat));
            assert_eq!(NextTo(cat), graph.get_word(cot));
            assert_eq!(NextTo(cot), graph.get_word(cog));
            assert_eq!(NextTo(cog), graph.get_word(dog));
        }
        #[test]
        fn should_not_mark_nodes_to_a_path_when_there_is_not_a_path() {
            let dog = Word::from("dog");
            let cog = Word::from("cog");
            let cot = Word::from("cot");
            let fog = Word::from("fog");
            let cat = Word::from("cat");
            let qux = Word::from("qux");
            let mut graph:WordGraph = vec![dog,fog,cog,cot,cat,qux]
                .into_iter().collect();
            graph.search(qux, dog);
            assert_eq!(Target, graph.get_word(qux));
            assert_eq!(Unmarked, graph.get_word(cat));
            // assert_eq!(Unmarked, graph.get_word(cog));
            // assert_eq!(Unmarked, graph.get_word(dog));
        }
    }
}
