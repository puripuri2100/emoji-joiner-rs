//! 特殊な木構造を扱うためのライブラリ
//! リストを渡されたら、重複しているところをうまく重ねてデータを保持し、データ量を削減する
//! また、検索の際の時間も削減する
//! 二分探索木などとは違い、子要素を大量に持てるようにしている

use std::vec;
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum V<T> {
  Node(T, Vec<V<T>>),
  End,
}

impl<T> Default for V<T>
where
  T: Clone + Eq + PartialEq + core::hash::Hash + PartialOrd + Ord + core::fmt::Debug,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<T> V<T>
where
  T: Clone + Eq + PartialEq + core::hash::Hash + PartialOrd + Ord + core::fmt::Debug,
{
  pub fn new() -> Self {
    V::End
  }

  pub fn lst_to_v(lst: &[T]) -> Self {
    let mut v: V<T> = V::End;
    for t in lst.iter().rev().cloned() {
      v = V::Node(t, vec![v])
    }
    v
  }

  pub fn insert(&self, lst: &[T]) -> Self {
    match self.insert_sub(lst, 0) {
      None => V::lst_to_v(lst),
      Some((v, _)) => v,
    }
  }
  pub fn insert_sub(&self, lst: &[T], index: usize) -> Option<(Self, bool)> {
    match self {
      V::End => Some((
        V::lst_to_v(&lst.iter().skip(index).cloned().collect::<Vec<_>>()),
        false,
      )),
      V::Node(t, children) => {
        if let Some(new_t) = lst.get(index) {
          // 追加するリストの先があるので、さらに奥に進む
          if t == new_t {
            let mut new_children = Vec::new();
            let mut is_insert = false;
            for v in children.iter() {
              match v.insert_sub(lst, index + 1) {
                None => {
                  // 更新なし
                  new_children.push(v.clone());
                }
                Some((v2, false)) => {
                  // 更新なし
                  // データを追加する
                  new_children.push(v.clone());
                  if !is_insert {
                    new_children.push(v2);
                    is_insert = true;
                  }
                }
                Some((v2, true)) => {
                  // 更新あり
                  new_children.push(v2);
                }
              }
            }
            Some((V::Node(t.clone(), new_children), true))
          } else {
            // タグが別になるので、ここで枝を生成してchildrenに追加する^
            Some((
              V::lst_to_v(&lst.iter().skip(index).cloned().collect::<Vec<_>>()),
              false,
            ))
          }
        } else {
          // 追加するリストのほうが短かった場合は、Endを追加する
          Some((V::End, false))
        }
      }
    }
  }

  /// 引数として与えるリストの先頭からいくつかの要素が木構造の中に含まれるかどうかを判定する
  /// もし存在しないならNoneを返し、
  /// もし存在するならばその存在する要素分をリストから取り除いたものを返す。
  /// 貪欲マッチする。つまり、複数のマッチする要素があったばあい、一番長いものを消費する
  pub fn search(&self, lst: &[T]) -> Option<(Vec<T>, Vec<T>)> {
    let v = self.search_sub(0, lst);
    v.map(|(v1, v2, _)| (v1, v2))
  }
  fn search_sub(&self, index: usize, lst: &[T]) -> Option<(Vec<T>, Vec<T>, bool)> {
    match self {
      V::End => Some((
        lst.iter().take(index).cloned().collect::<Vec<_>>(),
        lst.iter().skip(index).cloned().collect::<Vec<_>>(),
        true,
      )),
      V::Node(t, children) => {
        if let Some(new_t) = lst.get(index) {
          if t == new_t {
            match children
              .iter()
              .map(|v| v.search_sub(index + 1, lst))
              .filter(|v| v.is_some()) //.collect::<Vec<_>>().len()
              .max_by(|x, y| {
                let x_len = x.clone().unwrap().0.len();
                let y_len = y.clone().unwrap().0.len();
                x_len.cmp(&y_len)
              })
              .flatten()
            {
              None => None,
              Some((take, other, is_discover)) => {
                if is_discover {
                  Some((take, other, true))
                } else {
                  None
                }
              }
            }
          } else {
            None
          }
        } else {
          None
        }
      }
    }
  }

  pub fn delete(&mut self, _lst: &[T]) {
    todo!()
  }
}
