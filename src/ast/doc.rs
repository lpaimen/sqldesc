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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Doc {
  lines: Vec<String>
}

impl Doc {
  pub fn new() -> Self {
    Doc { lines: Vec::new() }
  }

  pub fn parse_whitespace(&mut self, whitespace: &Whitespace) {
    match whitespace {
      SingleLineComment(text) => self.parse_single_line_comment(text),
      MultiLineComment(text) => self.parse_multi_line_comment(text),
      _ => (),
    }
  }

  fn parse_single_line_comment(&mut self, text: &String) {
    if text.starts_with("*") {
      self.lines.push(Doc::parse_comment_line(text));
    }
  }

  fn parse_multi_line_comment(&mut self, text: &String) {
    if text.starts_with("*") {
      for line in text.lines() {
        self.lines.push(Doc::parse_comment_line(&line.to_string()))
      }
    }
  }

  fn parse_comment_line(line: &String) -> String {
    let trimmed = line.trim();
    if trimmed.starts_with("*") {
      return trimmed[1..].trim().to_string()
    } else {
      return trimmed.to_string()
    }
  }

  pub fn doc_string(&self) -> String {
    self.lines.join("\n")
  }

  pub fn is_useful(&self) -> bool {
    self.lines.len() != 0
  }

}
