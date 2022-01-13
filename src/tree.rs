//! 特殊な木構造を扱うためのライブラリ
//! リストを渡されたら、重複しているところをうまく重ねてデータを保持し、データ量を削減する
//! また、検索の際の時間も削減する
//! 二分探索木などとは違い、子要素を大量に持てるようにしている
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum V<T> {
  Node(T, Vec<V<T>>),
  End,
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

  pub fn insert(&mut self, lst: &[T]) {
    match self {
      V::End => {
        *self = self.insert_sub(lst, 0)[0].clone();
      }
      V::Node(t, _) => {
        if Some(&t.clone()) == lst.get(0) {
          *self = self.insert_sub(lst, 0)[0].clone();
        } else {
          panic!()
        }
      }
    }
  }
  pub fn insert_sub(&mut self, lst: &[T], index: usize) -> Vec<Self> {
    match self {
      V::End => {
        if lst.len() > index {
          vec![V::lst_to_v(
            &lst.iter().skip(index).cloned().collect::<Vec<_>>(),
          )]
        } else {
          vec![V::End]
        }
      }
      V::Node(t, children) => {
        if let Some(new_t) = lst.get(index) {
          // 追加するリストの先があるので、さらに奥に進む
          if t == new_t {
            // タグが同じなので、再帰関数をchildren全体にかける
            let new_children = children
              .iter()
              .map(|v| v.clone().insert_sub(lst, index + 1))
              .collect::<Vec<Vec<_>>>()
              .concat();
            vec![V::Node(t.clone(), new_children)]
          } else {
            // タグが別になるので、ここで枝を生成してchildrenに追加する
            vec![
              V::Node(t.clone(), children.clone()),
              V::lst_to_v(&lst.iter().skip(index).cloned().collect::<Vec<_>>()),
            ]
          }
        } else {
          // 追加するリストのほうが短かった場合は、Endを追加する
          vec![V::Node(t.clone(), children.clone()), V::End]
        }
      }
    }
  }

  /// 引数として与えるリストの先頭からいくつかの要素が木構造の中に含まれるかどうかを判定する
  /// もし存在しないならNoneを返し、
  /// もし存在するならばその存在する要素分をリストから取り除いたものを返す。
  /// 貪欲マッチする。つまり、複数のマッチする要素があったばあい、一番長いものを消費する
  pub fn search(&self, lst: &[T]) -> Option<(Vec<T>, Vec<T>)> {
    self.search_sub(&mut vec![], 0, lst)
  }
  fn search_sub(&self, temp: &mut Vec<T>, index: usize, lst: &[T]) -> Option<(Vec<T>, Vec<T>)> {
    match self {
      V::End => {
        // 終端なので、ここで返す
        if lst.len() > index {
          Some((
            temp.to_vec(),
            lst.iter().skip(index).cloned().collect::<Vec<_>>(),
          ))
        } else {
          Some((temp.to_vec(), vec![]))
        }
      }
      V::Node(t, children) => {
        if let Some(new_t) = lst.get(index) {
          if t == new_t {
            temp.push(t.clone());
            children
              .iter()
              .map(|v| v.search_sub(temp, index + 1, lst))
              .filter(|v| v.is_some())
              .max_by(|x, y| x.clone().unwrap().1.len().cmp(&y.clone().unwrap().1.len()))
              .flatten()
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
