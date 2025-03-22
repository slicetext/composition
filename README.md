# Composition

Composition is a Markdown to HTML compiler, written in Rust with ease of use in mind.

## Support

| Element | HTML| Description | Supported |
| :-----: | :--: |-| :-------: |
| \#      | \<h1>| Heading One | &check;   |
| \#\#      | \<h2>|Heading Two| &check;   |
| \#\#\#      | \<h3>|Heading Three| &check;   |
| \#\#\#\#      | \<h4>|Heading Four| &check;   |
| \#\#\#\#\#      | \<h5>|Heading Five| &check;   |
| \#\#\#\#\#\#      | \<h6>|Heading Six| &check;   |
| * or _      | \<em>|Italic| &check;   |
|** or __   | \<strong>|Bold| &check;   |
|~~   | \<del>|Strikethrough| &check;   |
|`   | \<code>|Code block| &check;   |
|```   | \<pre>\<code>|Multiline Code block| &check;   |
|<   | &lt|Left Angle Bracket| &check;   |
|>   | &gt|Right Angle Bracket| &check;   |


## Building & Installing

Clone the repo and then run `cargo install`

## Usage

```
Usage: Composition <​MARKDOWN> [OUTPUT]

Arguments:
  <​MARKDOWN>  Markdown file to compile
  [OUTPUT]    Name of Output HTML file

Options:
  -h, --help     Print help
  -V, --version  Print version
```
