use std::collections::HashMap;

pub mod data_text;
pub mod emoji_sequences_data_parser;
pub mod tree;

pub fn make_emoji_data_14_0() -> HashMap<char, tree::V<char>> {
  let mut tree_map = HashMap::new();
  emoji_sequences_data_parser::emoji_sequences_data_to_tree(
    &mut tree_map,
    data_text::UNICODE_EMOJI_SEQUENCES_14_0,
  );
  emoji_sequences_data_parser::emoji_sequences_data_to_tree(
    &mut tree_map,
    data_text::UNICODE_EMOJI_ZWJ_SEQUENCES_14_0,
  );
  tree_map
}

pub fn emoji_joiner_with_emoji_data(
  tree_map: &HashMap<char, tree::V<char>>,
  str: &str,
) -> Vec<Vec<char>> {
  let mut v = Vec::new();
  let mut chars = str.chars().collect::<Vec<char>>();
  while let Some(c) = chars.get(0) {
    if let Some(Some((take, last))) = tree_map.get(c).map(|tree| tree.search(&chars)) {
      v.push(take);
      chars = last;
    } else {
      v.push(vec![*c]);
      chars = chars.iter().skip(1).cloned().collect::<Vec<_>>();
    }
  }
  v
}

pub fn emoji_joiner_14_0(str: &str) -> Vec<Vec<char>> {
  let tree_map = make_emoji_data_14_0();
  emoji_joiner_with_emoji_data(&tree_map, str)
}

#[test]
fn test_emoji_joiner() {
  let emoji_data = make_emoji_data_14_0();
  assert_eq!(
    3,
    emoji_joiner_with_emoji_data(&emoji_data, "🧑‍🤝‍🧑👭👨‍👩‍👧‍👦").len()
  );
  assert_eq!(
    4,
    emoji_joiner_with_emoji_data(
      &emoji_data,
      &vec!['h', 'o', 'g', 'e'].iter().collect::<String>()
    )
    .len()
  );
  assert_eq!(
    3,
    emoji_joiner_with_emoji_data(&emoji_data, "\u{2714}\u{FE0E}\u{FE0F}").len()
  );
  assert_eq!(
    1,
    emoji_joiner_with_emoji_data(&emoji_data, "\u{1F9D1}\u{1F3FC}\u{200D}\u{2695}\u{FE0F}").len()
  );
  assert_eq!(
    vec![
      vec!['\u{1F9D1}', '\u{1F3FC}'],
      vec!['\u{200D}'],
      vec!['\u{2695}'],
      vec!['\u{FE0E}']
    ],
    emoji_joiner_with_emoji_data(&emoji_data, "\u{1F9D1}\u{1F3FC}\u{200D}\u{2695}\u{FE0E}")
  );
  assert_eq!(
    vec![
      vec![
        '\u{1F469}',
        '\u{1F3FB}',
        '\u{200D}',
        '\u{2764}',
        '\u{FE0F}',
        '\u{200D}',
        '\u{1F468}',
        '\u{1F3FB}'
      ],
      vec![
        '\u{1F469}',
        '\u{1F3FB}',
        '\u{200D}',
        '\u{2764}',
        '\u{FE0F}',
        '\u{200D}',
        '\u{1F468}',
        '\u{1F3FB}'
      ]
    ],
    emoji_joiner_with_emoji_data(
      &emoji_data,
      "\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}\u{1F3FB}\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}\u{1F3FB}"
    )
  );
  assert_eq!(
    vec![
      vec!['\u{1F469}', '\u{1F3FB}'],
      vec!['\u{200D}'],
      vec!['\u{2764}', '\u{FE0F}'],
      vec!['\u{200D}'],
      vec!['\u{1F421}'],
      vec!['\u{1F3FB}'],
      vec![
        '\u{1F469}',
        '\u{1F3FB}',
        '\u{200D}',
        '\u{2764}',
        '\u{FE0F}',
        '\u{200D}',
        '\u{1F468}',
        '\u{1F3FB}'
      ]
    ],
    emoji_joiner_with_emoji_data(
      &emoji_data,
      "\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{0023}\u{1F3FB}\u{1F469}\u{1F3FB}\u{200D}\u{2764}\u{FE0F}\u{200D}\u{1F468}\u{1F3FB}"
    )
  );
}
