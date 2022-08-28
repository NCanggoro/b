use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        let is_too_long = s.graphemes(true).count() > 256;

        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];

        let contains_forbidden_characters = s
          .chars()
          .any(|g| forbidden_characters.contains(&g));
        
        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
          Err(format!("{}, is not valid name", s))
        } else {
          Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
  fn as_ref(&self) -> &str {
      &self.0
  }
}

#[cfg(test)]
mod tests {
  use crate::domain::subscriber_name::SubscriberName;
  use claim::{assert_err, assert_ok};

  #[test]
  fn a_256_grapheme_long_name_is_valid() {
    let name = "a".repeat(256);
    assert_ok!(SubscriberName::parse(name));
  }

  #[test]
  fn more_than_256_grapheme_long_name_is_invalid() {
    let name = "a".repeat(257);
    assert_err!(SubscriberName::parse(name));
  }

  #[test]
  fn whitespace_only_name_is_invalid() {
    let name = " ".to_string();
    assert_err!(SubscriberName::parse(name));
  }

  #[test]
  fn empty_string_is_invalid() {
    let name = "".to_string();
    assert_err!(SubscriberName::parse(name));
  }

  #[test]
  fn names_containing_invalid_characters_are_rejected() {
    for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
      let name = name.to_string();
      assert_err!(SubscriberName::parse(name));
    }
  }

  #[test]
  fn valid_names_accepted() {
    let name = "nc no cap".to_string();
    assert_ok!(SubscriberName::parse(name));
  }

}