// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

//! Parse dictionary pages from the Czech language edition of Wiktionary into structured data.
//!
//! For general information about Parse Wiktionary, see the readme file.
//!
//! # Examples
//!
//! This example prints all definitions found in an article, together with the language and part of speech of the entry.
//!
//! ```
//! # extern crate parse_wiki_text;
//! # extern crate parse_wiktionary_cs;
//! #
//! let wiki_text = concat!(
//!     "==čeština==\n",
//!     "===sloveso===\n",
//!     "====význam====\n",
//!     "#Protestovat proti absurdnímu příkazu nebo povinnosti prostřednictvím ",
//!     "jestě absurdněji puntičkářského a nadšeného chování"
//! );
//! let configuration = parse_wiktionary_cs::create_configuration();
//! let parsed_wiki_text = configuration.parse(wiki_text);
//! let parsed_article = parse_wiktionary_cs::parse(wiki_text, &parsed_wiki_text.nodes);
//! # let mut found = false;
//! for language_entry in parsed_article.language_entries {
//!     for pos_entry in language_entry.pos_entries {
//!         for definition in pos_entry.definitions {
//!             println!(
//!                 "The word 'švejkovat' of language {language:?} and part of speech {pos:?} has the definition: {definition}",
//!                 language = language_entry.language,
//!                 pos = pos_entry.pos,
//!                 definition = &definition.definition.iter().map(|node| match node {
//!                     parse_wiktionary_cs::Flowing::Text { value } => value,
//!                     _ => ""
//!                 }).collect::<String>()
//!             );
//! #           found = true;
//!         }
//!     }
//! }
//! # assert!(found);
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate parse_wiki_text;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod configuration;
mod definition;
mod details;
mod etymology;
mod external_links;
mod hyphenation;
pub mod inflection;
mod inflection_field;
mod language;
mod languages;
mod pos;
mod pronunciation;
mod related_terms;
mod section;
mod templates;
mod translations;
mod util;

pub use configuration::create_configuration;
pub use languages::Language;
use parse_wiki_text::{ListItem, Node, Parameter};
use section::*;
use std::{borrow::Cow, collections::HashMap};
use util::*;

/// Audio sample.
#[derive(Debug, Deserialize, Serialize)]
pub struct Audio<'a> {
    /// The file name referred to.
    pub file_name: Cow<'a, str>,

    /// The label to display for the audio sample.
    pub label: Cow<'a, str>,
}

/// A single definition from a list of definitions of an entry.
///
/// Parsed from a single list item in the section `význam`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Definition<'a> {
    /// A series of elements to display as the definition.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definition: Vec<Flowing<'a>>,

    /// List of example sentences belonging to the definition, from the template [`Příklad`](https://cs.wiktionary.org/wiki/%C5%A0ablona:P%C5%99%C3%ADklad).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub examples: Vec<Cow<'a, str>>,

    /// List of labels, from the template [`Příznaky`](https://cs.wiktionary.org/wiki/%C5%A0ablona:P%C5%99%C3%ADznaky).
    ///
    /// Duplicate labels are not allowed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub labels: Vec<Cow<'a, str>>,

    /// A text to display as a phrase, if any, from the template [`Vazba`](https://cs.wiktionary.org/wiki/%C5%A0ablona:Vazba).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phrase: Option<Cow<'a, str>>,
}

/// External link.
#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalLink<'a> {
    /// The kind of page the link refers to.
    ///
    /// Parsed from the name of the parameter that has as its value the title of the page the link refers to. Different kinds are allowed for each wiki. For example links to Wikipedia allow the kinds `kategorie` (category), `portál` (portal), `rozcestník` (disambiguation page) and `článek` (article).
    pub kind: Cow<'a, str>,

    /// The wiki the link refers to.
    pub type_: ExternalLinkType,

    /// The title of the page the link refers to.
    pub value: Cow<'a, str>,
}

/// Identifier for a site as a target of an external link.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalLinkType {
    /// Wikimedia Commons.
    Commons,

    /// Wikibooks.
    Wikibooks,

    /// Wikinews.
    Wikinews,

    /// Wikipedia.
    Wikipedia,

    /// Wikiquote.
    Wikiquote,

    /// Wikisource.
    Wikisource,

    /// Wikispecies.
    Wikispecies,

    /// Wikiversity.
    Wikiversity,

    /// Wikivoyage.
    Wikivoyage,
}

/// An element in a sequence that allows different kinds of elements.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Flowing<'a> {
    /// Toggle italic.
    ///
    /// Parsed from the wiki text `''`.
    Italic,

    /// List of labels, from the template [`Příznak2`](https://cs.wiktionary.org/wiki/%C5%A0ablona:P%C5%99%C3%ADznak2).
    ///
    /// Duplicate labels are not allowed.
    Labels {
        /// The labels.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        labels: Vec<Cow<'a, str>>,
    },

    /// Link.
    ///
    /// Parsed from wiki text starting with `[[`.
    Link {
        /// The target the link refers to.
        target: Cow<'a, str>,

        /// The text to display for the link.
        text: Cow<'a, str>,
    },

    /// Indication that something is a plural, from the template [`množ`](https://cs.wiktionary.org/wiki/%C5%A0ablona:mno%C5%BE).
    Plural,

    /// Qualifier, from the template [`Upřesnění`](https://cs.wiktionary.org/wiki/%C5%A0ablona:Up%C5%99esn%C4%9Bn%C3%AD).
    Qualifier {
        /// The text to display.
        value: Cow<'a, str>,
    },

    /// Chunk of plain text.
    Text {
        /// The text to display.
        value: Cow<'a, str>,
    },

    /// Link to a foreign word in translations, from the template [`P`](https://cs.wiktionary.org/wiki/%C5%A0ablona:P).
    Translation {
        /// The gender of the term the link refers to, if specified, otherwise empty.
        #[serde(skip_serializing_if = "Option::is_none")]
        gender: Option<Cow<'a, str>>,

        /// The term the link refers to.
        term: Cow<'a, str>,
    },

    /// Element that could not be recognized.
    Unknown {
        /// The wiki text of the element.
        value: Cow<'a, str>,
    },
}

/// Pattern of inflection.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Inflection<'a> {
    /// From the template [`Adjektivum (cs)`](https://cs.wiktionary.org/wiki/%C5%A0ablona:Adjektivum_(cs)).
    AdjectiveDeclensionBasic(inflection::AdjectiveDeclensionBasic<'a>),

    /// From the template [`Stupňování (cs)`](https://cs.wiktionary.org/wiki/%C5%A0ablona:Stup%C5%88ov%C3%A1n%C3%AD_(cs)).
    Comparison(inflection::Comparison<'a>),

    /// From the template [`Sloveso (cs)`](https://cs.wiktionary.org/wiki/%C5%A0ablona:Sloveso_(cs)).
    Conjugation(inflection::Conjugation<'a>),

    /// From the template [`Substantivum (cs)`](https://cs.wiktionary.org/wiki/%C5%A0ablona:Substantivum_(cs)).
    Indeclinable,

    /// From the template [`Substantivum (cs)`](https://cs.wiktionary.org/wiki/%C5%A0ablona:Substantivum_(cs)).
    NounDeclensionBasic(inflection::NounDeclensionBasic<'a>),

    /// From the template [`Číslovka adj (cs)`](https://cs.wiktionary.org/wiki/%C5%A0ablona:%C4%8C%C3%ADslovka_adj_(cs)).
    NumeralDeclensionAdjective(inflection::AdjectiveDeclensionBasic<'a>),

    /// From the template [`Číslovka (cs)`](https://cs.wiktionary.org/wiki/%C5%A0ablona:%C4%8C%C3%ADslovka_(cs)).
    NumeralDeclensionBasic(inflection::NumeralDeclensionBasic<'a>),

    /// From the template [`Číslovka (cs)`](https://cs.wiktionary.org/wiki/%C5%A0ablona:%C4%8C%C3%ADslovka_(cs)).
    NumeralDeclensionSgPl(inflection::NumeralDeclensionSgPl<'a>),

    /// From the template [`Zájmeno adj (cs)`](https://cs.wiktionary.org/wiki/%C5%A0ablona:Z%C3%A1jmeno_adj_(cs)).
    PronounDeclensionAdjective(inflection::PronounDeclensionAdjective<'a>),

    /// From the template [`Zájmeno (cs)`](https://cs.wiktionary.org/wiki/%C5%A0ablona:Z%C3%A1jmeno_(cs)).
    PronounDeclensionBasic(inflection::PronounDeclensionBasic<'a>),
}

/// Information about one pattern of inflection for an entry.
#[derive(Debug, Deserialize, Serialize)]
pub struct InflectionEntry<'a> {
    /// Various details about the entry.
    ///
    /// Parsed from the unordered list in the section.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Vec<Flowing<'a>>>,

    /// The inflection itself.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inflection: Option<Inflection<'a>>,
}

/// Dictionary entry for a single language.
#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageEntry<'a> {
    /// List of audio samples for the entry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audio: Vec<Audio<'a>>,

    /// Etymology of the entry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub etymology: Vec<Flowing<'a>>,

    /// Homophones of the entry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub homophones: Vec<Vec<Flowing<'a>>>,

    /// Hyphenation of the entry, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyphenation: Option<Cow<'a, str>>,

    /// List of pronunciations of the entry written in IPA.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ipa: Vec<Cow<'a, str>>,

    /// The language of the entry.
    pub language: Language,

    /// Entries for parts of speech for this language.
    ///
    /// Parsed from the sections with the part of speech as their heading.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pos_entries: Vec<PosEntry<'a>>,

    /// Variants for the entry.
    ///
    /// Parsed from the section `varianty`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<Vec<Flowing<'a>>>,
}

/// Output of parsing a page.
#[derive(Debug, Deserialize, Serialize)]
pub struct Output<'a> {
    /// External links.
    ///
    /// Parsed from the section “externí odkazy”.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub external_links: Vec<ExternalLink<'a>>,

    /// The dictionary entries by language.
    ///
    /// Parsed from the sections with the name of the language as their heading.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub language_entries: Vec<LanguageEntry<'a>>,

    /// Warnings from the parser telling that something is not well-formed.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub warnings: Vec<Warning>,
}

/// Part of speech.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Pos {
    /// Abbreviation.
    ///
    /// Parsed from the section `zkratka` and similar numbered sections.
    Abbreviation,

    /// Adjective.
    ///
    /// Parsed from the section `přídavné jméno` and similar numbered sections.
    Adjective,

    /// Adverb.
    ///
    /// Parsed from the section `příslovce` and similar numbered sections.
    Adverb,

    /// Compound word.
    ///
    /// Parsed from the section `slovní spojení` and similar numbered sections.
    CompoundWord,

    /// Conjunction.
    ///
    /// Parsed from the section `spojka` and similar numbered sections.
    Conjunction,

    /// Idiom.
    ///
    /// Parsed from the section `idiom` and similar numbered sections.
    Idiom,

    /// Interjection.
    ///
    /// Parsed from the section `citoslovce` and similar numbered sections.
    Interjection,

    /// Noun.
    ///
    /// Parsed from the section `podstatné jméno` and similar numbered sections.
    Noun,

    /// Numeral.
    ///
    /// Parsed from the section `číslovka` and similar numbered sections.
    Numeral,

    /// Particle.
    ///
    /// Parsed from the section `částice` and similar numbered sections.
    Particle,

    /// Prefix.
    ///
    /// Parsed from the section `předpona` and similar numbered sections.
    Prefix,

    /// Preposition.
    ///
    /// Parsed from the section `předložka` and similar numbered sections.
    Preposition,

    /// Pronoun.
    ///
    /// Parsed from the section `zájmeno` and similar numbered sections.
    Pronoun,

    /// Proverb.
    ///
    /// Parsed from the sections `přísloví` and `rčení` and similar numbered sections.
    Proverb,

    /// Suffix.
    ///
    /// Parsed from the section `přípona` and similar numbered sections.
    Suffix,

    /// Verb.
    ///
    /// Parsed from the section `sloveso` and similar numbered sections.
    Verb,
}

/// The entry for a part of speech within the entry for a language.
#[derive(Debug, Deserialize, Serialize)]
pub struct PosEntry<'a> {
    /// Antonyms for the entry.
    ///
    /// Parsed from the section `antonyma`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub antonyms: Vec<Vec<Flowing<'a>>>,

    /// Compound words for the entry.
    ///
    /// Parsed from the section `slovní spojení`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub compound_words: Vec<Vec<Flowing<'a>>>,

    /// Definitions of the entry.
    ///
    /// Parsed from the section `význam`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub definitions: Vec<Definition<'a>>,

    /// Various details about the entry.
    ///
    /// Parsed from the unordered list between the POS heading and the next heading
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Vec<Flowing<'a>>>,

    /// Etymology of the entry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub etymology: Vec<Flowing<'a>>,

    /// Inflection of the entry, from the sections `časování`, `skloňování`, `skloňování (1)`, `skloňování (2)` and `stupňování`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub inflection: Vec<InflectionEntry<'a>>,

    /// Phrases and idioms for the entry.
    ///
    /// Parsed from the section `fráze a idiomy`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phrases_and_idioms: Vec<Vec<Flowing<'a>>>,

    /// Part of speech of the entry.
    ///
    /// Some parts of speech hold additional information.
    pub pos: Pos,

    /// Proverbs for the entry.
    ///
    /// Parsed from the section `přísloví, úsloví a pořekadla`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub proverbs: Vec<Cow<'a, str>>,

    /// Related terms for the entry.
    ///
    /// Parsed from the section `související`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_terms: Vec<Vec<Flowing<'a>>>,

    /// Synonyms for the entry.
    ///
    /// Parsed from the section `synonyma`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub synonyms: Vec<Vec<Flowing<'a>>>,

    /// Translations for each definition of the entry.
    ///
    /// Parsed from the section 'překlady'.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub translations: Vec<Translations<'a>>,

    /// Variants for the entry.
    ///
    /// Parsed from the section `varianty`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<Vec<Flowing<'a>>>,
}

/// The translations for a single definition.
///
/// Parsed from the template [Překlady](https://cs.wiktionary.org/wiki/%C5%A0ablona:P%C5%99eklady).
#[derive(Debug, Deserialize, Serialize)]
pub struct Translations<'a> {
    /// The gloss of the definition the translations relate to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gloss: Option<Cow<'a, str>>,

    /// The translations by language.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub translations: HashMap<Language, Vec<Flowing<'a>>>,
}

/// Warning from the parser telling that something is not well-formed.
///
/// When a warning occurs, it's not guaranteed that the text near the warning is parsed correctly. Usually the data that could not be unambiguously parsed due to the warning is excluded from the output, to make sure the output doesn't contain incorrectly parsed data.
#[derive(Debug, Deserialize, Serialize)]
pub struct Warning {
    /// The byte position in the wiki text where the warning ends.
    pub end: usize,

    /// The language of the language section in which the warning occurred, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,

    /// An identifier for the kind of warning.
    pub message: WarningMessage,

    /// The byte position in the wiki text where the warning starts.
    pub start: usize,
}

/// Identifier for a kind of warning from the parser.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WarningMessage {
    /// The element is a duplicate of something that comes before it.
    ///
    /// This may be a heading that contains the same text as a previous heading in the same section, or a parameter that has the same name as a previous parameter to the same template.
    Duplicate,

    /// The element is missing some required content.
    Empty,

    /// The section following the heading is missing some required content.
    SectionEmpty,

    /// The element is recognized but not represented in the output.
    ///
    /// The element conveys meaningful information, but this information has not been parsed and is not represented in the output. This applies for example to the templates `Doplnit` and `Viz` and the extension tag `ref`. In contrast to other warnings, this warning does not indicate there is anything wrong with the wiki text. It just indicates that the wiki text contains additional information that is not represented in the output. The element is recognized as valid in the position it occurs, but its content is not parsed, and nothing can be said about whether the content is valid.
    Supplementary,

    /// The element is not recognized.
    ///
    /// This may be because of the type of the element itself or because of anything inside it.
    Unrecognized,

    /// The value of the element conflicts with information occurring before it.
    ///
    /// This can mean for example that a specified language within a section doesn't match the language specified for the section as a whole.
    ValueConflicting,

    /// The element is recognized, but its value is not.
    ///
    /// On lists it means that a list with this kind is valid in this position, but something about the list items contained in the list is not recognized.
    ///
    /// On templates it means that a template with this name is valid in this position, but something about the parameters of the template is not recognized.
    ///
    /// On template parameters it means that a parameter with this name (or lack of name) is valid in this position, but something about the value of the parameter is not recognized.
    ValueUnrecognized,
}

/// Parses an article from the Czech language version of Wiktionary into structured data.
///
/// `wiki_text` is the wiki text of the article. `nodes` is the sequence of nodes obtained by parsing the wiki text with the crate [Parse Wiki Text](https://github.com/portstrom/parse_wiki_text).
#[must_use]
pub fn parse<'a>(wiki_text: &'a str, nodes: &[Node<'a>]) -> Output<'a> {
    let mut context = Context {
        language: None,
        warnings: vec![],
        wiki_text,
    };
    let mut external_links = None;
    let mut language_entries = vec![];
    let mut node_index = 0;
    let node_limit = nodes.len()
        - nodes
            .iter()
            .rev()
            .take_while(|node| match node {
                Node::Category { .. } | Node::ParagraphBreak { .. } => true,
                ::Node::Text { value, .. } => value.trim_left().is_empty(),
                _ => false,
            })
            .count();
    let nodes = &nodes[0..node_limit];
    while let Some(node) = nodes.get(node_index) {
        node_index += 1;
        match node {
            Node::Category { .. } => {}
            Node::Heading {
                level,
                nodes: title,
                ..
            } => if *level < 3 {
                if *level < 2 {
                    add_warning(&mut context, node, WarningMessage::Unrecognized);
                    break;
                }
                if let Some(title) = parse_text(title) {
                    if let Some(language) = Language::from_name(&title) {
                        node_index += language::parse_language(
                            &mut context,
                            node,
                            &nodes[node_index..],
                            &mut language_entries,
                            language,
                        );
                        continue;
                    }
                    match &title as _ {
                        "externí odkazy" => {
                            node_index += external_links::parse_external_links(
                                &mut context,
                                node,
                                &nodes[node_index..],
                                &mut external_links,
                            );
                            continue;
                        }
                        "poznámky" => while let Some(node) = nodes.get(node_index) {
                            match node {
                                Node::Heading { .. } => break,
                                Node::UnorderedList { .. } => {
                                    add_warning(&mut context, node, WarningMessage::Supplementary)
                                }
                                Node::Tag { name, nodes, .. } => if name != "references" {
                                    add_warning(&mut context, node, WarningMessage::Unrecognized);
                                } else if !nodes.is_empty() {
                                    add_warning(
                                        &mut context,
                                        node,
                                        WarningMessage::ValueUnrecognized,
                                    );
                                },
                                _ => unrecognized_unless_ignored(&mut context, node),
                            }
                            node_index += 1;
                        },
                        _ => {}
                    }
                }
            },
            Node::Template { name, .. } => add_warning(
                &mut context,
                node,
                if text_equals(name, "Viz") {
                    WarningMessage::Supplementary
                } else {
                    WarningMessage::Unrecognized
                },
            ),
            ::Node::Text { value, .. } => if !value.trim_left().is_empty() {
                add_warning(&mut context, node, WarningMessage::Unrecognized);
            },
            _ => add_warning(&mut context, node, WarningMessage::Unrecognized),
        }
    }
    Output {
        external_links: external_links.unwrap_or_default().unwrap_or_default(),
        language_entries,
        warnings: context.warnings,
    }
}
