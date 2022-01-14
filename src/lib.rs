use std::collections::HashMap;

mod data_text;
pub mod emoji_sequences_data_parser;
pub mod tree;

pub fn emoji_joiner_latest(str: &str) -> Vec<Vec<char>> {
  let mut tree = tree::V::Node(' ', vec![tree::V::End]);
  let mut v = Vec::new();
  emoji_sequences_data_parser::emoji_sequences_data_to_tree(
    &mut tree,
    data_text::UNICODE_EMOJI_SEQUENCES,
  );
  //println!("end");
  emoji_sequences_data_parser::emoji_sequences_data_to_tree(
    &mut tree,
    data_text::UNICODE_EMOJI_ZWJ_SEQUENCES,
  );
  //println!("end");
  let mut chars = str.chars().collect::<Vec<char>>();
  while let Some(c) = chars.get(0) {
    if let Some((take, last)) = tree.search(&chars) {
      v.push(take);
      chars = last;
    } else {
      v.push(vec![*c]);
      chars = chars.iter().skip(1).cloned().collect::<Vec<_>>();
    }
  }
  v
}
