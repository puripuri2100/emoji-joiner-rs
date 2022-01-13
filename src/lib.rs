use std::collections::HashMap;

mod emoji_sequences_data_parser;
mod tree;

pub fn emoji_joiner_latest(str: &str) -> Vec<Vec<char>> {
  let mut v = Vec::new();
  let mut tree_map = HashMap::new();
  tree_map = emoji_sequences_data_parser::emoji_sequences_data_to_tree(tree_map, "");
  let mut chars = str.chars().collect::<Vec<char>>();
  loop {
    if let Some(c) = chars.get(0) {
      if let Some(Some((take, last))) = tree_map.get(c).map(|tree| tree.search(&chars)) {
        v.push(take);
        chars = last;
      } else {
        v.push(vec![*c]);
        chars = chars.iter().skip(1).cloned().collect::<Vec<_>>();
      }
    } else {
      break;
    }
  }
  v
}
