struct Solution {}

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    use std::cmp::max;
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut ans = 0;
    let mut i = 0;
    for (index, e) in s.bytes().enumerate() {
      if let Some(&prev_index) = map.get(&e) {
        // 前一个发生重复的位置 + 1
        i = max(i, prev_index + 1);
      }
      if i == 0 {
        // 没有重复过
        ans = max(ans, index + 1);
      } else {
        // 两个位置差，但是要 + 1 弥补
        ans = max(ans, index - i + 1);
      }
      map.insert(e, index);
    }
    ans as i32
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn length_of_longest_substring() {
    assert_eq!(
      super::Solution::length_of_longest_substring(String::from("abcd")),
      4
    );
    assert_eq!(
      super::Solution::length_of_longest_substring(String::from("abcabcbb")),
      3
    );
    assert_eq!(
      super::Solution::length_of_longest_substring(String::from(" ")),
      1
    );
    assert_eq!(
      super::Solution::length_of_longest_substring(String::from("pwwkew")),
      3
    );
  }
}
