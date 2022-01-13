//! Unicode提供の
//! - [ZWJを使用しないで結合する絵文字の組み合わせの一覧](https://www.unicode.org/Public/emoji/latest/emoji-sequences.txt)
//! - [ZWJ結合する絵文字の組み合わせの一覧](https://www.unicode.org/Public/emoji/14.0/emoji-zwj-sequences.txt)
//! を解析してcrate::tree::V構造を生成する。

use crate::tree;
use std::collections::HashMap;

pub fn emoji_sequences_data_to_tree(
  tree_map: HashMap<char, tree::V<char>>,
  file_text: &str,
) -> HashMap<char, tree::V<char>> {
  tree_map
}
