// Copyright (c) 2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

//! Span types for tracking source locations.

use std::ops::Range;

pub type BytePosition = u32;

#[must_use]
#[inline]
#[allow(
    clippy::as_conversions,
    reason = "BytePosition to usize is lossless on 32/64-bit"
)]
pub const fn pos_to_usize(pos: BytePosition) -> usize {
    pos as usize
}

#[must_use]
#[inline]
#[allow(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    reason = "intentional truncation for files > 4GB"
)]
pub const fn usize_to_pos(val: usize) -> BytePosition {
    val as BytePosition
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Span {
    pub start: BytePosition,
    pub end: BytePosition,
}

impl Span {
    #[must_use]
    #[inline]
    pub const fn new(range: Range<BytePosition>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }

    #[must_use]
    #[inline]
    pub const fn from_usize_range(range: Range<usize>) -> Self {
        Self {
            start: usize_to_pos(range.start),
            end: usize_to_pos(range.end),
        }
    }

    #[must_use]
    #[inline]
    pub const fn at(pos: BytePosition) -> Self {
        Self {
            start: pos,
            end: pos,
        }
    }

    #[must_use]
    #[inline]
    pub const fn at_usize(pos: usize) -> Self {
        let byte_pos = usize_to_pos(pos);
        Self {
            start: byte_pos,
            end: byte_pos,
        }
    }

    #[must_use]
    #[inline]
    pub const fn start_usize(self) -> usize {
        pos_to_usize(self.start)
    }

    #[must_use]
    #[inline]
    pub const fn end_usize(self) -> usize {
        pos_to_usize(self.end)
    }

    #[must_use]
    #[inline]
    #[allow(clippy::as_conversions, reason = "u32 difference fits in usize")]
    pub const fn len(&self) -> usize {
        (self.end.saturating_sub(self.start)) as usize
    }

    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    #[must_use]
    #[inline]
    pub fn union(self, other: Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    #[must_use]
    #[inline]
    pub const fn to_range(self) -> Range<usize> {
        pos_to_usize(self.start)..pos_to_usize(self.end)
    }
}

impl From<Range<usize>> for Span {
    #[inline]
    fn from(range: Range<usize>) -> Self {
        Self::from_usize_range(range)
    }
}

impl From<Range<BytePosition>> for Span {
    #[inline]
    fn from(range: Range<BytePosition>) -> Self {
        Self::new(range)
    }
}

impl From<Span> for Range<usize> {
    #[inline]
    fn from(span: Span) -> Self {
        span.to_range()
    }
}

pub type Spanned<T> = (T, Span);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    #[must_use]
    pub const fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, Clone)]
pub struct SourceMap {
    line_starts: Vec<usize>,
}

impl SourceMap {
    #[must_use]
    pub fn new(input: &str) -> Self {
        let mut line_starts = vec![0];
        for (index, byte) in input.bytes().enumerate() {
            if byte == b'\n' {
                line_starts.push(index + 1);
            }
        }
        Self { line_starts }
    }

    #[must_use]
    pub fn position(&self, offset: usize) -> Position {
        let line_index = self.line_starts.partition_point(|start| *start <= offset) - 1;
        let line_start = self.line_starts.get(line_index).copied().unwrap_or(0);
        Position::new(line_index + 1, offset.saturating_sub(line_start) + 1)
    }
}
