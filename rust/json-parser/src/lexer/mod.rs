// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

mod token;

use std::borrow::Cow;

use crate::{
    error::{ErrorKind, ParseError},
    span::Span,
};

pub(crate) use token::{Token, TokenKind, TokenTag};

pub(crate) struct Lexer<'input> {
    input: &'input str,
    pos: usize,
    errors: Vec<ParseError>,
}

impl<'input> Lexer<'input> {
    pub(crate) fn new(input: &'input str) -> Self {
        Self {
            input,
            pos: 0,
            errors: Vec::new(),
        }
    }

    pub(crate) fn take_errors(&mut self) -> Vec<ParseError> {
        std::mem::take(&mut self.errors)
    }

    pub(crate) fn next_token(&mut self) -> Token<'input> {
        self.skip_whitespace();
        let start = self.pos;
        let Some(byte) = self.current_byte() else {
            return Token::new(TokenKind::Eof, Span::at_usize(self.pos));
        };

        match byte {
            b'{' => {
                self.pos += 1;
                Token::new(TokenKind::LeftBrace, Span::from(start..self.pos))
            }
            b'}' => {
                self.pos += 1;
                Token::new(TokenKind::RightBrace, Span::from(start..self.pos))
            }
            b'[' => {
                self.pos += 1;
                Token::new(TokenKind::LeftBracket, Span::from(start..self.pos))
            }
            b']' => {
                self.pos += 1;
                Token::new(TokenKind::RightBracket, Span::from(start..self.pos))
            }
            b':' => {
                self.pos += 1;
                Token::new(TokenKind::Colon, Span::from(start..self.pos))
            }
            b',' => {
                self.pos += 1;
                Token::new(TokenKind::Comma, Span::from(start..self.pos))
            }
            b'"' => self.lex_string(),
            b'-' | b'0'..=b'9' => self.lex_number(),
            b't' | b'f' | b'n' => self.lex_literal(),
            _ => {
                self.pos += 1;
                let span = Span::from(start..self.pos);
                self.errors
                    .push(ParseError::new(ErrorKind::InvalidCharacter, span));
                Token::new(TokenKind::Invalid, span)
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(byte) = self.current_byte() {
            if matches!(byte, b' ' | b'\n' | b'\r' | b'\t') {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn current_byte(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }

    fn lex_string(&mut self) -> Token<'input> {
        let start = self.pos;
        self.pos += 1;
        let mut owned: Option<String> = None;
        let mut segment_start = self.pos;

        while let Some(byte) = self.current_byte() {
            match byte {
                b'"' => {
                    let end_quote = self.pos;
                    self.pos += 1;
                    let value = if let Some(mut result) = owned {
                        result.push_str(&self.input[segment_start..end_quote]);
                        Cow::Owned(result)
                    } else {
                        Cow::Borrowed(&self.input[segment_start..end_quote])
                    };
                    return Token::new(TokenKind::String(value), Span::from(start..self.pos));
                }
                b'\\' => {
                    let result = owned.get_or_insert_with(|| {
                        // First escape means we must materialize an owned string.
                        // Start with the already-scanned prefix and reserve the
                        // remaining source slice length to avoid repeated growth.
                        let mut buffer = String::with_capacity(self.input.len() - start);
                        buffer.push_str(&self.input[segment_start..self.pos]);
                        buffer
                    });
                    if result.is_empty() {
                        result.push_str(&self.input[segment_start..self.pos]);
                    }
                    self.pos += 1;
                    result.push(self.decode_escape(start));
                    segment_start = self.pos;
                }
                0x00..=0x1F => {
                    let span = Span::from(self.pos..self.pos + 1);
                    self.errors
                        .push(ParseError::new(ErrorKind::InvalidCharacter, span));
                    let result = owned.get_or_insert_with(|| {
                        let mut buffer = String::with_capacity(self.input.len() - start);
                        buffer.push_str(&self.input[segment_start..self.pos]);
                        buffer
                    });
                    if result.is_empty() {
                        result.push_str(&self.input[segment_start..self.pos]);
                    }
                    self.pos += 1;
                    segment_start = self.pos;
                }
                _ => {
                    self.pos += 1;
                }
            }
        }

        let span = Span::from(start..self.input.len());
        self.errors
            .push(ParseError::new(ErrorKind::UnterminatedString, span));
        let value = if let Some(mut result) = owned {
            result.push_str(&self.input[segment_start..self.input.len()]);
            Cow::Owned(result)
        } else {
            Cow::Borrowed(&self.input[segment_start..self.input.len()])
        };
        Token::new(TokenKind::String(value), span)
    }

    fn decode_escape(&mut self, string_start: usize) -> char {
        let Some(byte) = self.current_byte() else {
            self.errors.push(ParseError::new(
                ErrorKind::UnterminatedString,
                Span::from(string_start..self.input.len()),
            ));
            return '\u{FFFD}';
        };

        match byte {
            b'"' => {
                self.pos += 1;
                '"'
            }
            b'\\' => {
                self.pos += 1;
                '\\'
            }
            b'/' => {
                self.pos += 1;
                '/'
            }
            b'b' => {
                self.pos += 1;
                '\u{0008}'
            }
            b'f' => {
                self.pos += 1;
                '\u{000C}'
            }
            b'n' => {
                self.pos += 1;
                '\n'
            }
            b'r' => {
                self.pos += 1;
                '\r'
            }
            b't' => {
                self.pos += 1;
                '\t'
            }
            b'u' => {
                self.pos += 1;
                self.decode_unicode_escape()
            }
            other => {
                self.pos += 1;
                self.errors.push(ParseError::new(
                    ErrorKind::InvalidEscape(char::from(other)),
                    Span::from(self.pos - 1..self.pos),
                ));
                char::from(other)
            }
        }
    }

    fn decode_unicode_escape(&mut self) -> char {
        let Some(code_point) = self.read_hex_u16() else {
            return '\u{FFFD}';
        };

        if !(0xD800..=0xDBFF).contains(&code_point) {
            return char::from_u32(u32::from(code_point)).unwrap_or('\u{FFFD}');
        }

        let save = self.pos;
        if self.current_byte() == Some(b'\\') && self.input.as_bytes().get(self.pos + 1) == Some(&b'u')
        {
            self.pos += 2;
            if let Some(low) = self.read_hex_u16()
                && (0xDC00..=0xDFFF).contains(&low)
            {
                let high_ten = u32::from(code_point) - 0xD800;
                let low_ten = u32::from(low) - 0xDC00;
                let scalar = 0x10000 + ((high_ten << 10) | low_ten);
                return char::from_u32(scalar).unwrap_or('\u{FFFD}');
            }
        }

        self.pos = save;
        self.errors.push(ParseError::new(
            ErrorKind::InvalidUnicodeEscape,
            Span::from(self.pos.saturating_sub(6)..self.pos),
        ));
        '\u{FFFD}'
    }

    fn read_hex_u16(&mut self) -> Option<u16> {
        let start = self.pos;
        let end = self.pos.saturating_add(4);
        let Some(slice) = self.input.get(start..end) else {
            self.errors.push(ParseError::new(
                ErrorKind::InvalidUnicodeEscape,
                Span::from(start..self.input.len()),
            ));
            self.pos = self.input.len();
            return None;
        };

        if !slice.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            self.errors.push(ParseError::new(
                ErrorKind::InvalidUnicodeEscape,
                Span::from(start..end),
            ));
            self.pos = end.min(self.input.len());
            return None;
        }

        self.pos = end;
        u16::from_str_radix(slice, 16).ok()
    }

    fn lex_number(&mut self) -> Token<'input> {
        let start = self.pos;

        if self.current_byte() == Some(b'-') {
            self.pos += 1;
        }

        match self.current_byte() {
            Some(b'0') => {
                self.pos += 1;
            }
            Some(b'1'..=b'9') => {
                self.pos += 1;
                while matches!(self.current_byte(), Some(b'0'..=b'9')) {
                    self.pos += 1;
                }
            }
            _ => {}
        }

        if self.current_byte() == Some(b'.') {
            self.pos += 1;
            while matches!(self.current_byte(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }

        if matches!(self.current_byte(), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.current_byte(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            while matches!(self.current_byte(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }

        Token::new(
            TokenKind::Number(Cow::Borrowed(&self.input[start..self.pos])),
            Span::from(start..self.pos),
        )
    }

    fn lex_literal(&mut self) -> Token<'input> {
        let start = self.pos;
        while matches!(self.current_byte(), Some(b'a'..=b'z' | b'A'..=b'Z')) {
            self.pos += 1;
        }

        let slice = &self.input[start..self.pos];
        let kind = match slice {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            _ => {
                self.errors
                    .push(ParseError::new(ErrorKind::InvalidLiteral, Span::from(start..self.pos)));
                TokenKind::Invalid
            }
        };
        Token::new(kind, Span::from(start..self.pos))
    }
}

pub(crate) struct TokenCursor<'input> {
    lexer: Lexer<'input>,
    peeked: Option<Token<'input>>,
}

impl<'input> TokenCursor<'input> {
    #[inline]
    pub(crate) fn new(input: &'input str) -> Self {
        Self {
            lexer: Lexer::new(input),
            peeked: None,
        }
    }

    #[inline]
    pub(crate) fn peek(&mut self) -> &Token<'input> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next_token());
        }
        self.peeked.as_ref().expect("peeked token must exist")
    }

    #[inline]
    pub(crate) fn peek_kind(&mut self) -> &TokenKind<'input> {
        &self.peek().kind
    }

    #[inline]
    pub(crate) fn peek_tag(&mut self) -> TokenTag {
        self.peek().tag
    }

    #[inline]
    pub(crate) fn peek_span(&mut self) -> Span {
        self.peek().span
    }

    #[inline]
    pub(crate) fn peek_kind_with_span(&mut self) -> (&TokenKind<'input>, Span) {
        let token = self.peek();
        (&token.kind, token.span)
    }

    #[inline]
    pub(crate) fn next(&mut self) -> Token<'input> {
        self.peeked.take().unwrap_or_else(|| self.lexer.next_token())
    }

    #[inline]
    pub(crate) fn push_back(&mut self, token: Token<'input>) {
        debug_assert!(self.peeked.is_none(), "push_back requires empty peek slot");
        self.peeked = Some(token);
    }

    #[inline]
    pub(crate) fn take_errors(&mut self) -> Vec<ParseError> {
        self.lexer.take_errors()
    }
}
