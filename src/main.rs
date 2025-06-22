fn main() {
    /*
    The bracket that doesn’t take up the space of an imaginary square is used
    for kana characters, to help with pronouncing kanji.
     */
    let left1 = '（' as u32;
    let left2 = '(' as u32;
    println!("（ as u32: {left1}\n( as u32: {left2}\n");

    /*
    Small hiragana from the a-column (i.e. consonants paired with the ‘a’ sound)
    are long vowel markers, similar to ー.
     */
    let small_a = 'ぁ' as u32;
    let big_a = 'あ' as u32;
    println!("Small ぁ: {small_a}\nBig あ: {big_a}");
}
