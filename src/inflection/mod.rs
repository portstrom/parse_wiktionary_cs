// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

//! Types specifying varyious patterns of inflection.

mod basic;
mod comparison;
mod conjugation;
mod noun;
mod numeral;

macro_rules! inflection {
    { $documentation:tt, $name:ident $( $field:ident )+ } => {
        #[derive(Debug, Default, Deserialize, Serialize)]
        #[doc=$documentation]
        pub struct $name<'a> {
            $(
                #[allow(missing_docs)]
                #[serde(default, skip_serializing_if = "Vec::is_empty")]
                pub $field: Vec<::Cow<'a, str>>
            ),*
        }
    }
}

inflection! {
    "Basic declension of adjectives.",
    AdjectiveDeclensionBasic paccf paccf_jm paccm paccm_jm paccma paccma_jm paccn paccn_jm pdatf pdatm pdatma pdatn pgenf pgenm pgenma pgenn pinsf pinsm pinsma pinsn plocf plocm plocma plocn pnomf pnomf_jm pnomm pnomm_jm pnomma pnomma_jm pnomn pnomn_jm saccf saccf_jm saccm saccm_jm saccma saccma_jm saccn saccn_jm sdatf sdatm sdatma sdatn sgenf sgenm sgenma sgenn sinsf sinsm sinsma sinsn slocf slocm slocma slocn snomf snomf_jm snomm snomm_jm snomma snomma_jm snomn snomn_jm
}

inflection! {
    "Forms for each degree of comparison.",
    Comparison comparative positive superlative
}

inflection! {
    "Basic declension of nouns.",
    NounDeclensionBasic dacc ddat dgen dins dloc dnom dvoc pacc pdat pgen pins ploc pnom pvoc sacc sdat sgen sins sloc snom svoc
}

inflection! {
    "Declension of numerals not distinguishing singular and plural forms.",
    NumeralDeclensionBasic acc dat gen ins loc nom voc
}

inflection! {
    "Declension of numerals distinguishing singular and plural forms.",
    NumeralDeclensionSgPl pacc pdat pgen pins ploc pnom pvoc sacc sdat sgen sins sloc snom svoc
}

inflection! {
    "Declension of pronouns that inflect like adjectives.",
    PronounDeclensionAdjective paccf paccm paccma paccn pdatf pdatm pdatma pdatn pgenf pgenm pgenma pgenn pinsf pinsm pinsma pinsn plocf plocm plocma plocn pnomf pnomm pnomma pnomn pvocf pvocm pvocma pvocn saccf saccm saccma saccn sdatf sdatm sdatma sdatn sgenf sgenm sgenma sgenn sinsf sinsm sinsma sinsn slocf slocm slocma slocn snomf snomm snomma snomn svocf svocm svocma svocn
}

inflection! {
    "Basic declension of pronouns.",
    PronounDeclensionBasic acc dat gen ins loc nom voc
}

inflection! {
    "Conjugation of verbs.",
    Conjugation mtraf mtram mtrap pactf pactm pimp1 pimp2 ppasf ppasm ppre1 ppre2 ppre3 ptraf ptram ptrap sactf sactm sactn simp2 spasf spasm spasn spre1 spre2 spre3
}

macro_rules! parse_inflection { {
    name: $name:tt
    $( , basic_template: $basic_pos:tt $basic_template_name:tt $variant:tt )*
    $( , special_template: $special_pos:tt $special_template_name:tt $function_name:expr )*
} => {
    pub(super) fn $name<'a>(
        context: &mut ::Context<'a>,
        heading_node: &::Node,
        nodes: &[::Node<'a>],
        output: &mut Vec<::InflectionEntry<'a>>,
        pos: ::Pos
    ) -> usize {
        let mut node_index = 0;
        let mut details = None;
        let mut inflection = None;
        while let Some(node) = nodes.get(node_index) {
            match node {
                ::Node::Heading { .. } => break,
                ::Node::Template { name, parameters, .. } => if let Some(name) = ::parse_text(name) {
                    match &name as _ {
                        $( $basic_template_name if pos == ::Pos::$basic_pos && context.language.unwrap() == ::Language::Cs => {
                            node_index += 1;
                            inflection = Some(if inflection.is_some() {
                                ::add_warning(context, node, ::WarningMessage::Duplicate);
                                None
                            } else {
                                ::inflection::basic::parse_inflection_basic(context, parameters, ::Inflection::$variant)
                            });
                            continue;
                        } )*
                        $( $special_template_name if pos == ::Pos::$special_pos && context.language.unwrap() == ::Language::Cs => {
                            node_index += 1;
                            inflection = Some(if inflection.is_some() {
                                ::add_warning(context, node, ::WarningMessage::Duplicate);
                                None
                            } else {
                                $function_name(context, node, parameters)
                            });
                            continue;
                        } )*
                        _ => {}
                    }
                }
                ::Node::UnorderedList { items, .. } => {
                    node_index += 1;
                    ::details::parse_details(context, node, items, &mut details);
                    continue;
                }
                _ => {}
            }
            node_index += 1;
            ::unrecognized_unless_ignored(context, node);
        }
        if details.is_none() && inflection.is_none() {
            ::add_warning(context, heading_node, ::WarningMessage::SectionEmpty);
        } else {
            output.push(::InflectionEntry {
                details: details.unwrap_or_default(),
                inflection: inflection.unwrap_or_default()
            });
        }
        node_index
    }
} }

parse_inflection! {
    name: parse_comparison,
    special_template: Adjective "Stupňování (cs)" ::inflection::comparison::parse_comparison_template,
    special_template: Adverb "Stupňování (cs)" ::inflection::comparison::parse_comparison_template
}

parse_inflection! {
    name: parse_conjugation,
    special_template: Verb "Sloveso (cs)" ::inflection::conjugation::parse_conjugation_template
}

parse_inflection! {
    name: parse_declension,
    basic_template: Adjective "Adjektivum (cs)" AdjectiveDeclensionBasic,
    basic_template: Numeral "Číslovka adj (cs)" NumeralDeclensionAdjective,
    basic_template: Pronoun "Zájmeno (cs)" PronounDeclensionBasic,
    basic_template: Pronoun "Zájmeno adj (cs)" PronounDeclensionAdjective,
    special_template: Noun "Substantivum (cs)" ::inflection::noun::parse_noun_declension_basic,
    special_template: Numeral "Číslovka (cs)" ::inflection::numeral::parse_numeral_declension_basic
}
