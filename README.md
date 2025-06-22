# Examine one anime episode’s subtitles

A first attempt at reading in, cleaning, processing and summarising the subtitles of one episode of an anime series. The subtitles are in Japanese, by the way.

The episode may, on occasion, also be referred to as a **corpus**.

## Subtitles

For this test project, we retrieve subtitles for the first episode of [_Psycho-Pass_ (season 1)](https://en.wikipedia.org/wiki/Psycho-Pass), available on `jimaku.cc`. [This is the download link.](https://jimaku.cc/entry/1407/download/Psycho-Pass.S01E01.WEBRip.Netflix.ja%5Bcc%5D.srt)

[Wikipedia](https://en.wikipedia.org/wiki/Psycho-Pass) describes _Psycho-Pass_ as a ‘cyberpunk psychological thriller’ set in a dystopian Japanese society. The series is particularly violent and, being rather serious (or depressive?) in tone, doesn’t contain much of the exaggerated speech patterns that Japanese anime series are famed for.

Perhaps there might be _some_ of such speech from the protagonist in the first episode, as she’s a rookie, but – mild spoiler alert here – as the series progresses, and in the latter series, all that new-hire enthusiasm is replaced by a brooding seriousness. Quite similar to real life, then.

## Structure of a subtitle file

A subtitle file has the `.srt` extension and has a very simple structure. Here’s an example of the first 10 subtitle units of the _Psycho-Pass_ episode we’ll look at:

```text
1
00:00:12,846 --> 00:00:24,899
♪～

2
00:00:46,921 --> 00:00:47,839
（狡噛(こうがみ)）フゥ～…

3
00:01:10,361 --> 00:01:11,112
（狡噛）うっ…！

4
00:01:14,324 --> 00:01:15,283
（狡噛）くそっ！

5
00:01:42,644 --> 00:01:47,440
（足音）

6
00:01:47,565 --> 00:01:50,235
（槙島(まきしま)）その傷で よくやるもんだ

7
00:01:52,654 --> 00:01:54,280
（朱(あかね)）きっと彼らは―

8
00:01:54,405 --> 00:01:56,157
一目 見て
分かったはずだ―

9
00:01:57,242 --> 00:01:59,994
２人は
初めて出会うより 以前から…―

10
00:02:00,161 --> 00:02:01,955
ああなる運命だったんだろう―
```

Each subtitle unit has three components: an index, timestamps and subtitles. The first two components occupy one line each. The subtitles may occupy more than one line (such as when two characters are talking at the same time). Simple formatting is also possible by way of HTML-style tags. One could display subtitle text in bold or italics, for instance.

## Data sources

As mentioned above, we will use the `.srt` file for S01E01 of _Psycho-Pass_ as our main data source. To make things a little more interesting, I also include the following collections of Japanese words to make cross-references and comparisons:

- Hiragana (syllabary for grammar and pronunciations of kanji characters)
- Katakana (syllabary for loan words and certain expressions of emotion)
- _Jōyō_ kanji (list of 2,136 kanji characters that students should have learnt upon high school graduation)
- _Kyōiku_ kanji (list of 1,026 kanji characters that students learn in primary school; for each of the six years of primary school, there is a set list of kanji that students must learn, and the JSON file containing the _kyōiku_ kanji reflects this in its structure)
- _Jinmeiyō_ kanji (list of characters for use in personal names)

The kana syllabaries also contain variations of the characters. These have diacritics or are part of digraphs. The _jōyō_ and _jinmeiyō_ kanji lists are compiled by the Japanese Ministry of Education.

There is an **additional list of kanji characters** that is defined by its absence. The _hyōgai_ kanji refers to characters that are found in neither the _jōyō_ nor _jinmeiyō_ kanji lists.

This set of kanji characters, if considered in its entirety, contains over 40,000 characters. A reasonable assumption one might make is that, the higher the percentage of _hyōgai_ characters in the subtitles (and thus the overall dialogue), the more adult-orientated an anime series is likely to be.
