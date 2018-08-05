
#[derive(Default)]
struct WordGraph {}


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
            let expected = vec!["CAT","COT","COG","DOG"].into_iter().map(Word::from).collect();
            assert_eq!(result, expected)
        }
    }

}
