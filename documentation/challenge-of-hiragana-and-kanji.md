# The challenge of hiragana and kanji

In Japanese, every kanji character can be written in terms of hiragana. There’s no one-to-one 
mapping for a given kanji to its hiragana representation. This is a big difference between kanji 
and their Chinese counterparts – in the latter, a character retains its pronunciation no matter 
where it is used. There are some characters that have multiple pronunciations, but this is also 
accompanied by a complete change in meaning.

This means that the mapping from kanji to hiragana is one-way only. Further complicating matters 
is the fact that hiragana are also used to indicate grammatical elements (such as the は in 私は, 
which serves as the subject marker for 私, meaning _I_ or _me_).

This has implications for the current exercise. If we count term frequencies at the _character 
level_, the numbers will generally be inaccurate. Whether or not a word/character is represented 
in kanji or hiragana comes down to individual preference. The person making the subtitles could 
represent all the content words with kanji, or go to the opposite extreme and represent all of 
them with hiragana.

Now, one kanji could have multiple hiragana characters (again, see 私, which is pronounced 
_watashi_ and represented as わたし in hiragana), and this is why the term frequencies would be 
inaccurate.

However, this is supposed to be a simple natural language processing exercise, and hence it’s 
fine to take the characters at face value. I wouldn’t want to place too much interpretation on 
the results. It’s also hard to interpret them without a meaningful comparison with another 
source of characters, such as another episode or an episode from a different series.
