//! Unicode提供の
//! - [ZWJを使用しないで結合する絵文字の組み合わせの一覧](https://www.unicode.org/Public/emoji/latest/emoji-sequences.txt)
//! - [ZWJ結合する絵文字の組み合わせの一覧](https://www.unicode.org/Public/emoji/latest/emoji-zwj-sequences.txt)
//! を解析してcrate::tree::V構造を生成する。

use crate::tree;
use std::collections::HashMap;

#[allow(mutable_borrow_reservation_conflict)]
pub fn emoji_sequences_data_to_tree(tree: &mut tree::V<char>, text: &str) {
  let mut index = 0;
  // charを保存していく
  let mut unicode_scalar_values_stack: Vec<char> = Vec::new();
  // trueになったら行末まで読み飛ばしてよい
  let mut is_end = false;
  // Unicodeスカラー値を表す16進法の数字を構成する文字をスタックする
  let mut hex_stack: Vec<char> = Vec::new();
  for c in text.chars() {
    match c {
      '\n' => {
        index += 1;
        //println!("loop{}", index);
        // 2文字目が存在していた場合は、1文字目でhashmap内を検索して
        // 出てきたtreeに挿入
        if unicode_scalar_values_stack.get(1).is_some() {
          if !hex_stack.is_empty() {
            let hex = u32::from_str_radix(&hex_stack.iter().collect::<String>(), 16).unwrap();
            unicode_scalar_values_stack.push(char::from_u32(hex).unwrap())
          }
          unicode_scalar_values_stack.splice(0..1, [' ']);
          //tree.insert(&unicode_scalar_values_stack);
          unicode_scalar_values_stack = Vec::new();
          hex_stack = Vec::new();
        }
        is_end = false;
      }
      '#' => {
        if !is_end {
          is_end = true;
          if !hex_stack.is_empty() {
            let hex = u32::from_str_radix(&hex_stack.iter().collect::<String>(), 16).unwrap();
            unicode_scalar_values_stack.push(char::from_u32(hex).unwrap());
            hex_stack = Vec::new();
          }
        }
      }
      ';' => {
        if !is_end {
          is_end = true;
          if !hex_stack.is_empty() {
            let hex = u32::from_str_radix(&hex_stack.iter().collect::<String>(), 16).unwrap();
            unicode_scalar_values_stack.push(char::from_u32(hex).unwrap());
            hex_stack = Vec::new();
          }
        }
      }
      ' ' => {
        if !is_end && !hex_stack.is_empty() {
          let hex = u32::from_str_radix(&hex_stack.iter().collect::<String>(), 16).unwrap();
          unicode_scalar_values_stack.push(char::from_u32(hex).unwrap());
          hex_stack = Vec::new();
        }
      }
      'A'..='Z' => {
        if !is_end {
          hex_stack.push(c)
        }
      }
      '0'..='9' => {
        if !is_end {
          hex_stack.push(c)
        }
      }
      '.' => {
        if !hex_stack.is_empty() {
          is_end = true;
          hex_stack = Vec::new();
        }
      }
      _ => {}
    }
  }
}
