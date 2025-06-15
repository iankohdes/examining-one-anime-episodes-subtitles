# Examine one anime episode’s subtitles

A first attempt at reading in, cleaning, processing and summarising the subtitles of one episode of an anime series. The subtitles are in Japanese, by the way.

The episode may, on occasion, also be referred to as a **corpus**.

## Subtitles

For this test project, we retrieve subtitles for the first episode of [_Psycho-Pass_ (season 1)](https://en.wikipedia.org/wiki/Psycho-Pass), available on `jimaku.cc`. [This is the download link.](https://jimaku.cc/entry/1407/download/Psycho-Pass.S01E01.WEBRip.Netflix.ja%5Bcc%5D.srt)

[Wikipedia](https://en.wikipedia.org/wiki/Psycho-Pass) describes _Psycho-Pass_ as a ‘cyberpunk psychological thriller’ set in a dystopian Japanese society. The series is particularly violent (I would venture to say that it’s needlessly so) and doesn’t contain much – or any – of the exaggerated speech patterns that Japanese anime series are known for.

Perhaps there might be _some_ of such speech from the protagonist in the first episode, as she’s a rookie, but – mild spoiler alert here – as the series progresses, and in the latter series, all that new-hire joy is replaced by a brooding seriousness. Quite similar to real life, then.

## Structure of a subtitle file

A subtitle file has the `.srt` extension and has a very simple structure. Here’s an example of the first five subtitle units of the _Psycho-Pass_ episode we’ll look at:

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
```

Each subtitle unit has three components: an index, timestamps and subtitles. The first two components occupy one line each. The subtitles may occupy more than one line (such as when two characters are talking at the same time). Simple formatting is also possible by way of HTML-style tags. One could display subtitle text in bold or italics, for instance.
