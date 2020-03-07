// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Documentation type

use super::super::tokenizer::Whitespace;
use super::super::tokenizer::Whitespace::SingleLineComment;
use super::super::tokenizer::Whitespace::MultiLineComment;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Doc {
  lines: Vec<String>
}

impl Doc {
  pub fn new() -> Self {
    Doc { lines: Vec::new() }
  }

  pub fn of(lines: &str) -> Self {
    let mut doc = Doc::new();
    for line in lines.lines() {
      doc.lines.push(line.to_string());
    }
    doc
  }

  pub fn parse_whitespace(&mut self, whitespace: &Whitespace) {
    match whitespace {
      SingleLineComment(text) => self.parse_single_line_comment(text),
      MultiLineComment(text) => self.parse_multi_line_comment(text),
      _ => (),
    }
  }

  fn is_desc(text: &str) -> bool {
    text.starts_with('*')
  }

  fn parse_single_line_comment(&mut self, text: &str) {
    if Doc::is_desc(text) {
      self.lines.push(Doc::parse_comment_line(text));
    }
  }

  fn parse_multi_line_comment(&mut self, text: &str) {
    if Doc::is_desc(text) {
      let mut drop_last = false;
      for (lineno, line) in text.lines().enumerate() {
        if lineno == 0 {
          // Use first line only if it contains something
          let text = Doc::parse_comment_line(&line.to_string());
          if !text.is_empty() {
            self.lines.push(text);
          }
        } else {
          drop_last = true;
          self.lines.push(Doc::parse_comment_line(&line.to_string()))
        }
      }
      if drop_last {
        // Remove last line
        self.lines.pop();
      }
    }
  }

  fn parse_comment_line(line: &str) -> String {
    let trimmed = line.trim();
    if Doc::is_desc(trimmed) {
      trimmed[1..].trim().to_string()
    } else {
      trimmed.to_string()
    }
  }

  pub fn doc_string(&self) -> String {
    self.lines.join("\n")
  }

  pub fn is_useful(&self) -> bool {
    !self.lines.is_empty()
  }

}
