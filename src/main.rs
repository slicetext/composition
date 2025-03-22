use std::{collections::BTreeMap, fs::{self}, path::PathBuf};
use clap::Parser;

struct ReplaceTable {
    values: BTreeMap<String, (bool, String, String, bool)>,
    endl_tag: (String, String),
}
impl ReplaceTable {
    fn insert(&mut self, key: &str, start_val: &str, endl: bool, end_val: &str) {
        self.values.insert(key.to_string(), (endl, start_val.to_string(), end_val.to_string(),false));
    }
    fn new() -> Self {
        let mut val = ReplaceTable { values: BTreeMap::new(), endl_tag: (String::from(""),String::from("")) };
        val.insert("<", "&lt", false, "");
        val.insert(">", "&gt", false, "");
        val.insert("+", "<ul><li>", true, "</li></ul>");
        val.insert("*", "<em>", false, "</em>");
        val.insert("**", "<strong>", false, "</strong>");
        val.insert("_", "<em>", false, "</em>");
        val.insert("__", "<strong>", false, "</strong>");
        val.insert("~~", "<del>", false, "</del>");
        val.insert("#", "<h1>", true, "</h1>");
        val.insert("##", "<h2>", true, "</h2>");
        val.insert("###", "<h3>", true, "</h3>");
        val.insert("####", "<h4>", true, "</h4>");
        val.insert("#####", "<h5>", true, "</h5>");
        val.insert("######", "<h6>", true, "</h6>");
        val.insert("`", "<code>", false, "</code>");
        val.insert("```", "<pre><code>", false, "</code></pre>");
        return val;
    }
    fn replace(&mut self, markdown: &str) -> String {
        let mut html: String = String::from("<!DOCTYPE html><html>\n\
        <link rel='stylesheet' href='https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css'>
<script src='https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js'></script><script>hljs.highlightAll();</script>");
        let mut symbol: String = String::new();
        for i in (markdown.to_owned()+" ").chars() {
            let mut result_count=0;
            for (k, _) in self.values.iter() {
                if k.contains(&(symbol.clone()+i.to_string().as_str())) { result_count += 1;
                }
            }
            if result_count > 0 {
                symbol.push(i);
            } else if symbol.len() == 0 {
                if i != '\n' {
                    html = html + i.to_string().as_str();
                }
            } else {
                if self.values[&symbol].3 == false {
                    html = html + &self.values[&symbol].1;
                    if self.values[&symbol].0 {
                        self.endl_tag = (symbol.clone(),self.values[&symbol].clone().2);
                    }
                } else {
                    html = html + &self.values[&symbol].2;
                }
                if &self.values[&symbol].2 != "" {
                    if let Some(x) = self.values.get_mut(&symbol) {
                        x.3 = !x.3;
                    }
                }
                symbol = String::from("");
                if i != '\n' {
                    html = html + i.to_string().as_str();
                }
            }
            if i=='\n' {
                if self.endl_tag.1 == "" {
                    html = html + "<br>";
                } else {
                    html = html + &self.endl_tag.1;
                }
                if let Some(x) = self.values.get_mut(&self.endl_tag.0) {
                    x.3 = !x.3;
                }
                self.endl_tag = (String::from(""),String::from(""));
                continue;
            }
        }
        return html + "</html>";
    }
}
#[derive(Parser,Debug)]
#[command(about="Composition, a markdown to html compiler",long_about = None,version)]
struct Args {
    /// Markdown file to compile
    markdown: PathBuf,
    /// Name of Output HTML file
    output: Option<String>,
}
const COLOR_YELLOW: &str = "\x1b[1;93m";
const COLOR_RESET: &str = "\x1b[0m";
fn main() {
    let args = Args::parse();
    let mut rtable: ReplaceTable = ReplaceTable::new();
    let md = args.markdown;
    let md_contents = fs::read_to_string(md)
        .expect("Failed to read Markdown File");
    let html = rtable.replace(md_contents.as_str());
    let html_fp;
    if let Some(fp) = args.output.as_deref() {
        html_fp = fp.to_owned();
    } else {
        html_fp = String::from("md_rs_output.html");
        println!("{}Writing to md_rs_output.html{}",COLOR_YELLOW,COLOR_RESET);
    }
    let _ = fs::write(html_fp, html).expect("Failed to write file");
}
