use cssparser::*;
use crate::traits::{Parse, ToCss};
use crate::printer::Printer;
use std::fmt::Write;
use super::calc::Calc;

impl Parse for f32 {
  fn parse<'i, 't>(input: &mut Parser<'i, 't>) -> Result<Self, ParseError<'i, ()>> {
    if let Ok(number) = input.try_parse(|input| input.expect_number()) {
      return Ok(number)
    }

    Err(input.new_error_for_next_token())
  }
}

impl ToCss for f32 {
  fn to_css<W>(&self, dest: &mut Printer<W>) -> std::fmt::Result where W: std::fmt::Write {
    serialize_number(*self, dest)
  }
}

pub fn serialize_number<W>(number: f32, dest: &mut Printer<W>) -> std::fmt::Result where W: std::fmt::Write {
  use cssparser::ToCss;
  let int_value = if number.fract() == 0.0 {
    Some(number as i32)
  } else {
    None
  };
  let tok = Token::Number {
    has_sign: number < 0.0,
    value: number,
    int_value
  };
  if number != 0.0 && number.abs() < 1.0 {
    let mut s = String::new();
    tok.to_css(&mut s)?;
    if number < 0.0 {
      dest.write_char('-')?;
      dest.write_str(s.trim_start_matches("-0"))
    } else {
      dest.write_str(s.trim_start_matches('0'))
    }
  } else {
    tok.to_css(dest)
  }
}
