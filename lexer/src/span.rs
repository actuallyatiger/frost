use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Create a new span with the given start and end positions.
    /// Note that the end position is exclusive.
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    /// Get the length of the span.
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    /// Check if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}..{}]", self.start, self.end)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_span_creation() {
        let span = Span::new(5, 10);
        assert_eq!(span.start, 5);
        assert_eq!(span.end, 10);
    }

    #[test]
    fn test_span_length() {
        let span = Span::new(5, 10);
        assert_eq!(span.len(), 5);
    }

    #[test]
    fn test_span_display() {
        let span = Span::new(5, 10);
        assert_eq!(format!("{}", span), "[5..10]");
    }
}
