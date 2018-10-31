// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

extern crate parse_wiki_text;
extern crate parse_wiktionary_cs;

#[test]
fn main() {
    let _ = parse_wiktionary_cs::parse(
        "",
        &parse_wiktionary_cs::create_configuration().parse("").nodes,
    );
}
