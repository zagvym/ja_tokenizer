# ja_tokenizer

tokenizer for Japanese text using [vibrato](https://github.com/daac-tools/vibrato).

## Requirements

- rust
- [vibrato](https://github.com/daac-tools/vibrato)

## System Dictionary

https://github.com/daac-tools/vibrato/tree/main#1-dictionary-preparation
> You can also compile or train system dictionaries from your own resources. See the [docs](https://github.com/daac-tools/vibrato/blob/main/docs) for more advanced usage.

### from [mecab-ipadic-NEologd](https://github.com/neologd/mecab-ipadic-neologd)

```sh
# clone and build NEologd dictionary
# See: https://github.com/neologd/mecab-ipadic-neologd#how-to-installupdate-mecab-ipadic-neologd
$ git clone --depth 1 git@github.com:neologd/mecab-ipadic-neologd.git
$ cd mecab-ipadic-neologd
$ ./bin/install-mecab-ipadic-neologd -n

# merge lexicon files of neologd-dict
$ cd build/mecab-ipadic-xxxx # depends on neologd's version
$ cat *.csv > lex.csv

# convert to vibrato's dict
# See: https://github.com/daac-tools/vibrato/blob/main/docs/compile.md
$ git clone git@github.com:daac-tools/vibrato.git
$ cd vivrato
$ ln -s path/to/mecab-ipadic-neologd/build/mecab-ipadic-xxxx neologd
$ cargo run --release -p compile -- \
    -l neologd/lex.csv \
    -m neologd/matrix.def \
    -u neologd/unk.def \
    -c neologd/char.def \
    -o system.dic.zst
```

## Usage

```sh
$ echo '本とカレーの街神保町へようこそ。' | cargo run -- -i path/to/system.dic.zst
# ...
num_tokens: 10
本: 名詞,一般,*,*,*,*,本,ホン,ホン
と: 助詞,並立助詞,*,*,*,*,と,ト,ト
カレー: 名詞,固有名詞,地域,一般,*,*,カレー,カレー,カレー
の: 助詞,連体化,*,*,*,*,の,ノ,ノ
街: 名詞,一般,*,*,*,*,街,マチ,マチ
神保町: 名詞,固有名詞,地域,一般,*,*,神保町,ジンボウチョウ,ジンボーチョー
へ: 助詞,格助詞,一般,*,*,*,へ,ヘ,エ
ようこそ: 感動詞,*,*,*,*,*,ようこそ,ヨウコソ,ヨーコソ
。: 記号,句点,*,*,*,*,。,。,。

: 記号,空白,*,*,*,*,*
```


### Options

```sh
$ cargo run -- -h
# ...
simple tokenizer for Japanese text

Usage: ja_tokenizer [OPTIONS]

Options:
  -i, --sysdic-path <SYSDIC_PATH>            System dictionary (in zstd) [default: system.dic.zst]
  -S, --ignore-space                         Ignores white spaces in input strings
  -M, --max-grouping-len <MAX_GROUPING_LEN>  Maximum length of unknown words [default: 24]
  -h, --help                                 Print help (see more with '--help')
```
