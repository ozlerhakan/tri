use clap::Parser;
use dotenv::dotenv;
use reqwest;
use serde_json;
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(name = "tri")]
#[command(bin_name = "tri")]
#[command(author = "Hakan Özler <ozler.hakan@gmail.com>")]
#[command(version = "0.0.1")]
#[command(about = "translate a given text from English to Turkish using Google Translate API", long_about = None)]
#[command(next_line_help = true)]
struct Args {
    #[clap(value_parser = clap::value_parser!(String))]
    text: String,
    #[clap(short, long)]
    verbose: bool,
    #[clap(env = "TRI_GTAPI_KEY")]
    key: String,
}

fn main() {
    dotenv().ok();
    let args = Args::parse();

    let source = "en";
    let target = "tr";

    let output = translate(args, source, target);
    println!("{}", output.unwrap());
}

fn translate(args: Args, source: &str, target: &str) -> Result<String, Box<dyn std::error::Error>> {
    let text = args.text.replace('\n', "");
    let url = format!(
        "https://translation.googleapis.com/language/translate/v2?key={}&q={}&source={}&target={}",
        args.key, text, source, target
    );

    let response: Value = reqwest::blocking::get(&url)?.json()?;
    let translated_text = response["data"]["translations"][0]["translatedText"]
        .as_str()
        .unwrap();

    Ok(translated_text.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    #[test]
    fn test_parser() {
        let args = Args {
            text: "Hello World".to_string(),
            verbose: false,
            key: env::var("TRI_GTAPI_KEY").unwrap(),
        };
        let t = translate(args, "en", "tr");
        let expected = t.unwrap();
        assert_eq!(expected, "Selam Dünya")
    }
}
