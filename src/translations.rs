// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

use std::collections::hash_map::Entry;

pub fn parse_translations<'a>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    output: &mut Option<Option<Vec<::Translations<'a>>>>,
) -> usize {
    ::parse_ordered_list_section(
        context,
        heading_node,
        nodes,
        output,
        |context, list_item| {
            let mut translations = ::HashMap::new();
            if let [::Node::Template {
                name, parameters, ..
            }] = list_item.nodes.as_slice()
            {
                if ::text_equals(name, "Překlady") {
                    let mut gloss = None;
                    for parameter in parameters {
                        if let Some(name) = ::parse_parameter_name(parameter) {
                            if let Some(language) = ::Language::from_language_code(&name) {
                                if language != context.language.unwrap() {
                                    match translations.entry(language) {
                                        Entry::Occupied(mut entry) => {
                                            ::add_warning(
                                                context,
                                                parameter,
                                                ::WarningMessage::Duplicate,
                                            );
                                            entry.insert(parse_translation(
                                                context,
                                                name,
                                                &parameter.value,
                                            ));
                                        }
                                        Entry::Vacant(entry) => {
                                            entry.insert(parse_translation(
                                                context,
                                                name,
                                                &parameter.value,
                                            ));
                                        }
                                    }
                                }
                                continue;
                            }
                            if name == "význam" {
                                if gloss.is_some() {
                                    ::add_warning(context, parameter, ::WarningMessage::Duplicate);
                                }
                                let value = ::parse_text_not_empty(&parameter.value);
                                if value.is_none() {
                                    ::add_warning(
                                        context,
                                        parameter,
                                        ::WarningMessage::ValueUnrecognized,
                                    );
                                }
                                gloss = Some(value);
                                continue;
                            }
                        }
                        ::add_warning(context, parameter, ::WarningMessage::Unrecognized);
                    }
                    return ::Translations {
                        gloss: gloss.unwrap_or_default(),
                        translations,
                    };
                }
            }
            ::add_warning(context, list_item, ::WarningMessage::ValueUnrecognized);
            ::Translations {
                gloss: None,
                translations,
            }
        },
    )
}

fn parse_translation<'a>(
    context: &mut ::Context<'a>,
    language: &str,
    nodes: &[::Node<'a>],
) -> Vec<::Flowing<'a>> {
    nodes
        .iter()
        .map(|node| {
            match node {
                ::Node::Template {
                    name, parameters, ..
                } => if let Some(name) = ::parse_text(name) {
                    match &name as _ {
                        "P" => if let Some(node) = parse_template_translation(language, parameters)
                        {
                            return node;
                        },
                        "Příznak2" => {
                            let labels =
                                ::templates::parse_template_labels(context, node, parameters);
                            if !labels.is_empty() {
                                return ::Flowing::Labels { labels };
                            }
                        }
                        "Upřesnění" => {
                            if let Some(node) = ::templates::parse_template_qualifier(parameters) {
                                return node;
                            }
                        }
                        "množ" => if parameters.is_empty() {
                            return ::Flowing::Plural;
                        },
                        _ => {}
                    }
                },
                ::Node::Text { value, .. } => {
                    return ::Flowing::Text {
                        value: ::Cow::Borrowed(value),
                    }
                }
                _ => {}
            }
            ::create_unknown(context, node)
        })
        .collect()
}

fn parse_template_translation<'a>(
    outer_language: &str,
    parameters: &[::Parameter<'a>],
) -> Option<::Flowing<'a>> {
    match parameters {
        [::Parameter {
            name: None,
            value: language,
            ..
        }, ::Parameter {
            name: None,
            value: term,
            ..
        }] => if ::text_equals(language, outer_language) {
            if let Some(term) = ::parse_text_not_empty(term) {
                return Some(::Flowing::Translation { gender: None, term });
            }
        },
        [::Parameter {
            name: None,
            value: language,
            ..
        }, ::Parameter {
            name: None,
            value: term,
            ..
        }, ::Parameter {
            name: None,
            value: gender,
            ..
        }] => if ::text_equals(language, outer_language) {
            if let Some(term) = ::parse_text_not_empty(term) {
                if let Some(gender) = ::parse_text_not_empty(gender) {
                    return Some(::Flowing::Translation {
                        gender: Some(gender),
                        term,
                    });
                }
            }
        },
        _ => {}
    }
    None
}
