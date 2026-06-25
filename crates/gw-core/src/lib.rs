//! 'gw-core' holds all the actual logic: parsing references, looking up
//! verses, searching, etc. Neither the CLI crate nor the (future) TUI crate
//! should contain real logic themselves -- they should call into here.
//! That way both interfaces stay in sync automatically.

use thiserror::Error;

/// A single, resolved reference to a place in the Bible, e.g. John 3:16.
///
/// This is just enough to support a single verse lookup. Ranges (e.g. "Gen 1:1-3")
/// can be added later as a seperate variant or by extending this struct with an optional
/// 'end_verse'.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    pub book: String,
    pub chapter: u32,
    pub verse: u32,
}

/// Everything that can go wrong while resolving a reference or fetching text.
///
/// Using 'thiserror' here (rather than 'anyhow') because this is a *library*
/// crate -- callers like the CLI may want to match on specific error
/// varriants (e.g. to print a different message for "bad input" vs . "verse
/// out of range"). 'anyhow' is better suited to the CLI/TUI binaries, where
/// errors just need to be reported, not matched on.
#[derive(Debug, Error)]
pub enum BibleError {
    #[error("couldn't understand reference: '{0}'")]
    InvalidReference(String),

    #[error("'{book}' doesn't look like a known book of the Bible")]
    UnknownBook { book: String },

    #[error("{book} {chapter} doesn't have a verse {verse}")]
    VerseOutOfRange {
        book: String,
        chapter: u32,
        verse: u32,
    },
}

/// Parse a human-typed reference like "John 3:16" into a 'Reference'.
///
/// This is very much so bare-bones -- it only handles the
/// "Book Chapter:Verse" shape. Ranges, whole-chapter references (just
/// "John 3"), and abbreviations ("Jn", "Gen") are good follow-up tasks
/// once this skeleton compiles and runs.
pub fn parse_reference(input: &str) -> Result<Reference, BibleError> {
    let input = input.trim();

    // Split "John 3:16" into "John" and "3:16" on the *last* space, so that
    // multi-word book names like "1 Corinthians" still work.
    let (book, rest) = input
        .rsplit_once(' ')
        .ok_or_else(|| BibleError::InvalidReference(input.to_string()))?;

    let (chapter_str, verse_str) = rest
        .split_once(':')
        .ok_or_else(|| BibleError::InvalidReference(input.to_string()))?;
    let chapter: u32 = chapter_str
        .parse()
        .map_err(|_| BibleError::InvalidReference(input.to_string()))?;
    let verse: u32 = verse_str
        .parse()
        .map_err(|_| BibleError::InvalidReference(input.to_string()))?;

    Ok(Reference {
        book: book.to_string(),
        chapter,
        verse,
    })
}

/// Look up the text for a given reference
///
/// This is a place holder: it doesn't touch any real Bible data yet. The
/// next step here in our roadmap "Bundle KJV, ASV, WEB test data" -- once that's wired up, this
/// function should load from that data instead of returning a fixed string.  Lord-Willing!
pub fn lookup(reference: &Reference) -> Result<String, BibleError> {
    Ok(format!(
        "[placeholder test for {} {}:{} -- real lookup not wired up yet]",
        reference.book, reference.chapter, reference.verse
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_simple_reference() {
        let r = parse_reference("John 3:16").unwrap();
        assert_eq!(r.book, "John");
        assert_eq!(r.chapter, 3);
        assert_eq!(r.verse, 16);
    }

    #[test]
    fn parses_a_multi_word_book_name() {
        let r = parse_reference("1 Corinthians 13:4").unwrap();
        assert_eq!(r.book, "1 Corinthians");
        assert_eq!(r.chapter, 13);
        assert_eq!(r.verse, 4);
    }

    #[test]
    fn rejects_garbage_input() {
        assert!(parse_reference("not a reference").is_err());
    }
}
