// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_definitions<'a>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    output: &mut Option<Option<Vec<::Definition<'a>>>>,
) -> usize {
    ::parse_ordered_list_section(
        context,
        heading_node,
        nodes,
        output,
        |context, list_item| {
            let mut definition = vec![];
            let mut examples = vec![];
            let mut iterator = list_item.nodes.iter();
            let mut labels = None;
            let mut phrase = None;
            while let Some(node) = iterator.next() {
                match node {
                    ::Node::Italic { .. } => definition.push(::Flowing::Italic),
                    ::Node::Link { target, text, .. } => {
                        definition.push(::parse_link(context, node, target, text))
                    }
                    ::Node::Template {
                        name, parameters, ..
                    } => {
                        if let Some(name) = ::parse_text(name) {
                            match &name as _ {
                                "Příznaky" => if definition.is_empty() {
                                    if labels.is_some() {
                                        labels = Some(None);
                                        ::add_warning(context, node, ::WarningMessage::Duplicate);
                                        continue;
                                    }
                                    if let [parameter @ ::Parameter { name: None, .. }] =
                                        &parameters[..1]
                                    {
                                        if check_language(context, &parameter.value) {
                                            labels =
                                                Some(Some(::templates::parse_template_labels(
                                                    context,
                                                    node,
                                                    &parameters[1..],
                                                )));
                                        } else {
                                            labels = Some(None);
                                            definition.push(::create_unknown2(
                                                context,
                                                node,
                                                parameter,
                                                ::WarningMessage::ValueConflicting,
                                            ));
                                        }
                                        continue;
                                    }
                                    labels = Some(None)
                                },
                                "Upřesnění" => if let Some(result) =
                                    ::templates::parse_template_qualifier(parameters)
                                {
                                    definition.push(result);
                                    continue;
                                },
                                "Vazba" => if definition.is_empty() {
                                    phrase = Some(if phrase.is_some() {
                                        ::add_warning(context, node, ::WarningMessage::Duplicate);
                                        None
                                    } else {
                                        match parameters.as_slice() {
                                            [language_parameter @ ::Parameter {
                                                name: None, ..
                                            }, phrase_parameter @ ::Parameter {
                                                name: None, ..
                                            }] => if check_language(
                                                context,
                                                &language_parameter.value,
                                            ) {
                                                match ::parse_text_not_empty(
                                                    &phrase_parameter.value,
                                                ) {
                                                    None => {
                                                        definition.push(::create_unknown2(
                                                            context,
                                                            node,
                                                            phrase_parameter,
                                                            ::WarningMessage::ValueUnrecognized,
                                                        ));
                                                        None
                                                    }
                                                    value @ Some(_) => value,
                                                }
                                            } else {
                                                definition.push(::create_unknown2(
                                                    context,
                                                    node,
                                                    language_parameter,
                                                    ::WarningMessage::ValueConflicting,
                                                ));
                                                None
                                            },
                                            _ => {
                                                definition.push(::create_unknown2(
                                                    context,
                                                    node,
                                                    node,
                                                    ::WarningMessage::ValueUnrecognized,
                                                ));
                                                None
                                            }
                                        }
                                    });
                                    continue;
                                },
                                _ => {}
                            }
                        }
                        definition.push(::create_unknown(context, node));
                    }
                    ::Node::Text { value, .. } => if definition.is_empty() {
                        let value = value.trim_left();
                        if !value.is_empty() {
                            definition.push(::Flowing::Text {
                                value: ::Cow::Borrowed(value),
                            });
                        }
                    } else {
                        definition.push(::Flowing::Text {
                            value: ::Cow::Borrowed(value),
                        });
                    },
                    ::Node::UnorderedList { items, .. } => {
                        examples = items
                            .iter()
                            .filter_map(|item| {
                                let output_item = match item.nodes.as_slice() {
                                    [::Node::Template {
                                        name, parameters, ..
                                    }] => parse_example(context, name, parameters),
                                    [::Node::Template {
                                        name: template_name,
                                        parameters,
                                        ..
                                    }, tag_node]
                                        if match tag_node {
                                            ::Node::Tag { name: tag_name, .. } => tag_name == "ref",
                                            _ => false,
                                        } =>
                                    {
                                        ::add_warning(
                                            context,
                                            tag_node,
                                            ::WarningMessage::Supplementary,
                                        );
                                        parse_example(context, template_name, parameters)
                                    }
                                    _ => None,
                                };
                                if output_item.is_none() {
                                    ::add_warning(
                                        context,
                                        item,
                                        ::WarningMessage::ValueUnrecognized,
                                    );
                                }
                                output_item
                            })
                            .collect();
                        while let Some(node) = iterator.next() {
                            ::add_warning(context, node, ::WarningMessage::Unrecognized);
                        }
                    }
                    _ => definition.push(::create_unknown(context, node)),
                }
            }
            if definition.is_empty() {
                ::add_warning(context, list_item, ::WarningMessage::Empty);
            }
            ::Definition {
                phrase: phrase.unwrap_or_default(),
                definition,
                labels: labels.unwrap_or_default().unwrap_or_default(),
                examples,
            }
        },
    )
}

fn parse_example<'a>(
    context: &mut ::Context,
    name: &[::Node<'a>],
    parameters: &[::Parameter<'a>],
) -> Option<::Cow<'a, str>> {
    if ::text_equals(name, "Příklad") {
        if let [::Parameter {
            name: None,
            value: language,
            ..
        }, ::Parameter {
            name: None,
            value: example,
            ..
        }] = parameters
        {
            if check_language(context, language) {
                return ::parse_text(example);
            }
        }
    }
    None
}

fn check_language(context: &::Context, nodes: &[::Node]) -> bool {
    ::text_equals(nodes, context.language.unwrap().language_code())
}
