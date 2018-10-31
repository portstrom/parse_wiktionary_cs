// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_pos<'a>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    pos_entries: &mut Vec<::PosEntry<'a>>,
    pos: ::Pos,
) -> usize {
    let mut antonyms = None;
    let mut compound_words = None;
    let mut definitions = None;
    let mut details = None;
    let mut etymology = None;
    let mut inflection = vec![];
    let mut node_index = 0;
    let mut phrases_and_idioms = None;
    let mut proverbs = None;
    let mut related_terms = None;
    let mut synonyms = None;
    let mut translations = None;
    let mut variants = None;
    while let Some(node) = nodes.get(node_index) {
        macro_rules! parse_section { ( $function:path, $( $output:tt )+ ) => {
            $function(context, node, &nodes[node_index..], &mut $( $output )+ )
        } }
        match node {
            ::Node::Heading {
                level,
                nodes: title,
                ..
            } if *level < 5 =>
            {
                if *level < 4 {
                    break;
                }
                if details.is_none() {
                    details = Some(vec![]);
                }
                node_index += 1;
                match title.as_slice() {
                    [::Node::Text { value, .. }] => {
                        node_index += match *value {
                            "antonyma" => parse_section!(::related_terms::parse_synonyms, antonyms),
                            "etymologie" => parse_section!(::etymology::parse_etymology, etymology),
                            "fráze a idiomy" => parse_section!(
                                ::related_terms::parse_related_terms,
                                phrases_and_idioms
                            ),
                            "překlady" => {
                                parse_section!(::translations::parse_translations, translations)
                            }
                            "přísloví, úsloví a pořekadla" => {
                                parse_section!(::related_terms::parse_proverbs, proverbs)
                            }
                            "slovní spojení" => {
                                parse_section!(::related_terms::parse_related_terms, compound_words)
                            }
                            "související" => {
                                parse_section!(::related_terms::parse_related_terms, related_terms)
                            }
                            "skloňování" | "skloňování (1)" | "skloňování (2)" => {
                                parse_section!(::inflection::parse_declension, inflection, pos)
                            }
                            "stupňování" => {
                                parse_section!(::inflection::parse_comparison, inflection, pos)
                            }
                            "synonyma" => parse_section!(::related_terms::parse_synonyms, synonyms),
                            "varianty" => {
                                parse_section!(::related_terms::parse_related_terms, variants)
                            }
                            "význam" => {
                                parse_section!(::definition::parse_definitions, definitions)
                            }
                            "časování" => {
                                parse_section!(::inflection::parse_conjugation, inflection, pos)
                            }
                            _ => {
                                ::add_warning(context, node, ::WarningMessage::Unrecognized);
                                0
                            }
                        }
                    }
                    _ => ::add_warning(context, node, ::WarningMessage::Unrecognized),
                }
            }
            ::Node::UnorderedList { items, .. } => {
                node_index += 1;
                ::details::parse_details(context, node, items, &mut details);
            }
            _ => {
                node_index += 1;
                ::add_warning(context, node, ::WarningMessage::Unrecognized);
            }
        }
    }
    if definitions.is_none() {
        ::add_warning(context, heading_node, ::WarningMessage::SectionEmpty);
    }
    pos_entries.push(::PosEntry {
        antonyms: antonyms.unwrap_or_default().unwrap_or_default(),
        compound_words: compound_words.unwrap_or_default().unwrap_or_default(),
        definitions: definitions.unwrap_or_default().unwrap_or_default(),
        details: details.unwrap_or_default(),
        etymology: etymology.unwrap_or_default().unwrap_or_default(),
        inflection,
        phrases_and_idioms: phrases_and_idioms.unwrap_or_default().unwrap_or_default(),
        pos,
        proverbs: proverbs.unwrap_or_default().unwrap_or_default(),
        related_terms: related_terms.unwrap_or_default().unwrap_or_default(),
        synonyms: synonyms.unwrap_or_default().unwrap_or_default(),
        translations: translations.unwrap_or_default().unwrap_or_default(),
        variants: variants.unwrap_or_default().unwrap_or_default(),
    });
    node_index
}
