use std::fs::File;
use std::io;
use std::path::PathBuf;
use vibrato::{Dictionary, Tokenizer, token::Token};

use clap::Parser;


const DEFAULT_SYSDIC_PATH: &str = "system.dic.zst";

/**
 * Refer to: https://github.com/daac-tools/vibrato/blob/v0.5.1/benchmark/src/main.rs#L17-L34
 */
#[derive(Parser, Debug)]
#[clap(
    name = "tokenizer",
    about = "simple tokenizer for Japanese text"
)]
struct Args {
    /// System dictionary (in zstd).
    #[clap(short = 'i', long, default_value = DEFAULT_SYSDIC_PATH)]
    sysdic_path: PathBuf,

    /// User dictionary (csv)
    #[clap(short = 'u', long)]
    userdic_path: Option<PathBuf>,

    /// Ignores white spaces in input strings.
    #[clap(short = 'S', long)]
    ignore_space: bool,

    /// Maximum length of unknown words.
    #[clap(short = 'M', long, default_value_t = 24)]
    max_grouping_len: usize,

    /// print token details
    #[clap(short = 'v', long)]
    verbose: bool,
}

fn fmt_token(t: Token<'_, '_>, verbose: bool) -> String {
    if verbose {
        return format!("{}: {},{},{},{}", t.surface(), t.left_id(), t.right_id(), t.word_cost(), t.feature());
    }

    return format!("{}: {}", t.surface(), t.feature());
}

/**
 * Refer to: https://zenn.dev/tfutada/articles/30fcf2729da035
 */
pub fn main() {
    let args = Args::parse();

    // load vibrato's system dict
    let reader = zstd::Decoder::new(File::open(args.sysdic_path).unwrap()).unwrap();
    let mut dict = Dictionary::read(reader).unwrap();

    // load user dict if exists
    if let Some(userdic_path) = args.userdic_path {
        dict = dict.reset_user_lexicon_from_reader(Some(File::open(userdic_path).unwrap())).unwrap();
    }

    // init tokenizer
    let tokenizer = Tokenizer::new(dict)
        .ignore_space(args.ignore_space).unwrap()
        .max_grouping_len(args.max_grouping_len);

    // woker must be mutable
    let mut worker = tokenizer.new_worker();

    // read input text from stdin
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let text = &buffer;

    // set text to tokenize
    worker.reset_sentence(text);
    worker.tokenize();

    println!("num_tokens: {}", worker.num_tokens());

    // print tokens
    let fmt = |t: Token<'_, '_>| -> String { fmt_token(t, args.verbose) };
    worker.token_iter()
        // .filter(|t| { // 名詞のみ表示
        //     let words: Vec<&str> = t.feature().split(',').collect();
        //     let subwords: Vec<&str> = words[0].split('-').collect();
        //     subwords[0] == "名詞" || subwords[0] == "カスタム名詞"
        // })
        .for_each(|t| {
            println!("{}", fmt(t));
        });
}
