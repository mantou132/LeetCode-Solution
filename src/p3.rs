struct Solution {}

impl Solution {
  pub fn length_of_longest_substring(s: String) -> i32 {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let mut ans = 0;
    for (index, c) in s.chars().enumerate() {
      // note: expected reference `&_`
      // found type `char`
      if let Some(prev_index) = map.get(c) {
        map.insert(c, index);
        let l = index - prev_index + 1;
        ans = if ans > l { ans } else { l };
      };
    }
    ans as i32
  }
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(
      super::Solution::length_of_longest_substring(String::from("abcabcbb")),
      3
    );
  }
}
