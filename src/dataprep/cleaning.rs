use std::collections::BTreeSet;

pub fn helper_dedupe_and_sort(xs: &str) {
    //! Deduplicates a string and sorts its characters, then **prints the result**.
    //!
    //! Useful for identifying unwanted characters to add to a blacklist. Uses
    //! the `BTreeSet` data structure.
    //!
    //! Unwanted characters are defined as follows:
    //!
    //! - Punctuation and spaces
    //! - Latin or non-Japanese characters
    //! - [_Chōonpu_](https://en.wikipedia.org/wiki/Ch%C5%8Donpu) (a symbol that
    //!   indicates the prolonged sound of the katakana character immediately
    //!   preceding it)
    //!
    //! _Chōonpu_ usually come after katakana characters and rarely after
    //! hiragana ones. I remove them because they convey no additional
    //! information about existing characters in a subtitle unit.
    //!
    //! The point of sorting is to (hopefully!) cluster unwanted characters
    //! away from kanji, hiragana and katakana.

    let deduped_and_sorted: String = xs
        .chars()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    println!("{deduped_and_sorted}");
}