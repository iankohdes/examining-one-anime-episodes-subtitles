/*
Processing here involves labelling (cleaned) characters as either kanji, hiragana
or katakana. Modelling involves enriching characters with metadata information,
such as their total number of occurrences in the episode, label/category,
membership status in the jōyō kanji and so on. (Use a single schema for all
categories of characters.)

Then, convert the structs created above into a dataframe as preparation to
present results.
 */