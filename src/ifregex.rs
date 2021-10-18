pub(crate) use regex::Regex;

pub fn build() -> Result<Regex, String> {
  const IF_LINE_RE: &str = r"^\s*[[:alnum:]]+:(?:\s+\d+){16}$";

  if let Ok(r) = Regex::new(IF_LINE_RE) { return Ok(r) };
  Err("regular expression is invalid".to_string())
}