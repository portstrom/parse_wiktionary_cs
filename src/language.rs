// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_language<'a>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    language_entries: &mut Vec<::LanguageEntry<'a>>,
    language: ::Language,
) -> usize {
    for entry in language_entries.iter() {
        if entry.language == language {
            ::add_warning(context, heading_node, ::WarningMessage::Duplicate);
            break;
        }
    }
    let mut node_index = 0;
    let mut etymology = None;
    let mut hyphenation = None;
    let mut pos_entries = vec![];
    let mut pronunciation = None;
    let mut variants = None;
    context.language = Some(language);
    while let Some(node) = nodes.get(node_index) {
        macro_rules! parse_section {
            ($output:tt $function:path) => {
                $function(context, node, &nodes[node_index..], &mut $output)
            };
        }
        macro_rules! parse_pos {
            ($pos:tt) => {
                ::pos::parse_pos(
                    context,
                    node,
                    &nodes[node_index..],
                    &mut pos_entries,
                    ::Pos::$pos,
                )
            };
        }
        match node {
            ::Node::Heading {
                level,
                nodes: title,
                ..
            } if *level < 4 =>
            {
                if *level < 3 {
                    break;
                }
                node_index += 1;
                node_index += match title.as_slice() {
                    [::Node::Text { value, .. }] => match *value {
                        "citoslovce" | "citoslovce (1)" | "citoslovce (2)" | "citoslovce (3)"
                        | "citoslovce (4)" | "citoslovce (5)" => parse_pos!(Interjection),
                        "dělení" => parse_section!(hyphenation::hyphenation::parse_hyphenation),
                        "etymologie" => parse_section!(etymology::etymology::parse_etymology),
                        "idiom" | "idiom (1)" | "idiom (2)" | "idiom (3)" | "idiom (4)"
                        | "idiom (5)" => parse_pos!(Idiom),
                        "podstatné jméno"
                        | "podstatné jméno (1)"
                        | "podstatné jméno (2)"
                        | "podstatné jméno (3)"
                        | "podstatné jméno (4)"
                        | "podstatné jméno (5)" => parse_pos!(Noun),
                        "předložka" | "předložka (1)" | "předložka (2)"
                        | "předložka (3)" | "předložka (4)" | "předložka (5)" => {
                            parse_pos!(Preposition)
                        }
                        "předpona" | "předpona (1)" | "předpona (2)" | "předpona (3)"
                        | "předpona (4)" | "předpona (5)" => parse_pos!(Prefix),
                        "přídavné jméno"
                        | "přídavné jméno (1)"
                        | "přídavné jméno (2)"
                        | "přídavné jméno (3)"
                        | "přídavné jméno (4)"
                        | "přídavné jméno (5)" => parse_pos!(Adjective),
                        "přípona" | "přípona (1)" | "přípona (2)" | "přípona (3)"
                        | "přípona (4)" | "přípona (5)" => parse_pos!(Suffix),
                        "příslovce" | "příslovce (1)" | "příslovce (2)"
                        | "příslovce (3)" | "příslovce (4)" | "příslovce (5)" => {
                            parse_pos!(Adverb)
                        }
                        "přísloví" | "přísloví (1)" | "přísloví (2)"
                        | "přísloví (3)" | "přísloví (4)" | "přísloví (5)" => {
                            parse_pos!(Proverb)
                        }
                        "rčení" | "rčení (1)" | "rčení (2)" | "rčení (3)"
                        | "rčení (4)" | "rčení (5)" => parse_pos!(Proverb),
                        "sloveso" | "sloveso (1)" | "sloveso (2)" | "sloveso (3)"
                        | "sloveso (4)" | "sloveso (5)" => parse_pos!(Verb),
                        "slovní spojení"
                        | "slovní spojení (1)"
                        | "slovní spojení (2)"
                        | "slovní spojení (3)"
                        | "slovní spojení (4)"
                        | "slovní spojení (5)" => parse_pos!(CompoundWord),
                        "spojka" | "spojka (1)" | "spojka (2)" | "spojka (3)" | "spojka (4)"
                        | "spojka (5)" => parse_pos!(Conjunction),
                        "varianty" => parse_section!(variants::related_terms::parse_related_terms),
                        "výslovnost" => {
                            parse_section!(pronunciation::pronunciation::parse_pronunciation)
                        }
                        "zkratka" | "zkratka (1)" | "zkratka (2)" | "zkratka (3)"
                        | "zkratka (4)" | "zkratka (5)" => parse_pos!(Abbreviation),
                        "zájmeno" | "zájmeno (1)" | "zájmeno (2)" | "zájmeno (3)"
                        | "zájmeno (4)" | "zájmeno (5)" => parse_pos!(Pronoun),
                        "částice" | "částice (1)" | "částice (2)" | "částice (3)"
                        | "částice (4)" | "částice (5)" => parse_pos!(Particle),
                        "číslovka" | "číslovka (1)" | "číslovka (2)" | "číslovka (3)"
                        | "číslovka (4)" | "číslovka (5)" => parse_pos!(Numeral),
                        _ => {
                            ::add_warning(context, node, ::WarningMessage::Unrecognized);
                            0
                        }
                    },
                    _ => {
                        ::add_warning(context, node, ::WarningMessage::Unrecognized);
                        0
                    }
                };
            }
            _ => {
                node_index += 1;
                ::add_warning(context, node, ::WarningMessage::Unrecognized);
            }
        }
    }
    if pos_entries.is_empty() {
        ::add_warning(context, heading_node, ::WarningMessage::SectionEmpty);
    }
    context.language = None;
    let pronunciation = pronunciation.unwrap_or_default().unwrap_or_default();
    language_entries.push(::LanguageEntry {
        audio: pronunciation.audio.into_iter().collect(),
        pos_entries,
        etymology: etymology.unwrap_or_default().unwrap_or_default(),
        homophones: pronunciation.homophones,
        hyphenation: hyphenation.unwrap_or_default(),
        ipa: pronunciation.ipa,
        language,
        variants: variants.unwrap_or_default().unwrap_or_default(),
    });
    node_index
}
