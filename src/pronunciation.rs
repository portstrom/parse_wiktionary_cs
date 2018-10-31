// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

#[derive(Default)]
pub struct Pronunciation<'a> {
    pub audio: Option<::Audio<'a>>,
    pub homophones: Vec<Vec<::Flowing<'a>>>,
    pub ipa: Vec<::Cow<'a, str>>,
}

pub fn parse_pronunciation<'a>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    output: &mut Option<Option<Pronunciation<'a>>>,
) -> usize {
    ::parse_section(context, heading_node, output, |context, output| {
        let mut list_output = None;
        let mut node_index = 0;
        while let Some(node) = nodes.get(node_index) {
            match node {
                ::Node::Heading { .. } => break,
                ::Node::UnorderedList { items, .. } => {
                    list_output = Some(if list_output.is_some() {
                        ::add_warning(context, node, ::WarningMessage::Duplicate);
                        None
                    } else {
                        parse_list(context, node, items)
                    })
                }
                _ => ::unrecognized_unless_ignored(context, node),
            }
            node_index += 1;
        }
        if list_output.is_none() {
            ::add_warning(context, heading_node, ::WarningMessage::SectionEmpty);
        }
        let mut homophones = None;
        while let Some(node) = nodes.get(node_index) {
            match node {
                ::Node::Heading {
                    level,
                    nodes: heading_nodes,
                    ..
                } => {
                    if *level < 4 {
                        break;
                    }
                    node_index += 1;
                    if *level == 4 && ::text_equals(heading_nodes, "homofony") {
                        node_index += ::related_terms::parse_related_terms(
                            context,
                            node,
                            &nodes[node_index..],
                            &mut homophones,
                        );
                    }
                }
                _ => {
                    node_index += 1;
                    ::unrecognized_unless_ignored(context, node)
                }
            }
        }
        let mut list_output = list_output.unwrap_or_default().unwrap_or_default();
        list_output.homophones = homophones.unwrap_or_default().unwrap_or_default();
        *output = Some(Some(list_output));
        node_index
    })
}

fn parse_list<'a>(
    context: &mut ::Context<'a>,
    list_node: &::Node,
    items: &[::ListItem<'a>],
) -> Option<Pronunciation<'a>> {
    match items {
        [item1] => match item1.nodes.as_slice() {
            [template_node] => if let ::Node::Template {
                name, parameters, ..
            } = template_node
            {
                if let Some(name) = ::parse_text(name) {
                    match &name as _ {
                        "Audio" => {
                            return Some(Pronunciation {
                                audio: Some(parse_template_audio(
                                    context,
                                    template_node,
                                    parameters,
                                )?),
                                homophones: vec![],
                                ipa: vec![],
                            })
                        }
                        "IPA" => {
                            return Some(Pronunciation {
                                audio: None,
                                homophones: vec![],
                                ipa: parse_template_ipa(context, template_node, parameters)?,
                            })
                        }
                        _ => {}
                    }
                }
            },
            [template_node1, ::Node::Text { value: ", ", .. }, template_node2] => {
                if let ::Node::Template {
                    name: name1,
                    parameters: parameters1,
                    ..
                } = template_node1
                {
                    if let ::Node::Template {
                        name: name2,
                        parameters: parameters2,
                        ..
                    } = template_node2
                    {
                        if let Some(name1) = ::parse_text(name1) {
                            match &name1 as _ {
                                "Audio" => if ::text_equals(name2, "IPA") {
                                    return Some(Pronunciation {
                                        audio: Some(parse_template_audio(
                                            context,
                                            template_node1,
                                            parameters1,
                                        )?),
                                        homophones: vec![],
                                        ipa: parse_template_ipa(
                                            context,
                                            template_node2,
                                            parameters2,
                                        )?,
                                    });
                                },
                                "IPA" => if ::text_equals(name2, "Audio") {
                                    return Some(Pronunciation {
                                        ipa: parse_template_ipa(
                                            context,
                                            template_node1,
                                            parameters1,
                                        )?,
                                        audio: Some(parse_template_audio(
                                            context,
                                            template_node2,
                                            parameters2,
                                        )?),
                                        homophones: vec![],
                                    });
                                },
                                _ => {}
                            }
                        }
                    }
                }
            }
            _ => {}
        },
        [item1, item2] => if let [template1] = item1.nodes.as_slice() {
            if let ::Node::Template {
                name: name1,
                parameters: parameters1,
                ..
            } = template1
            {
                if ::text_equals(name1, "IPA") {
                    if let [template2] = item2.nodes.as_slice() {
                        if let ::Node::Template {
                            name: name2,
                            parameters: parameters2,
                            ..
                        } = template2
                        {
                            if ::text_equals(name2, "Audio") {
                                return Some(Pronunciation {
                                    ipa: parse_template_ipa(context, template1, parameters1)?,
                                    audio: Some(parse_template_audio(
                                        context,
                                        template2,
                                        parameters2,
                                    )?),
                                    homophones: vec![],
                                });
                            }
                        }
                    }
                }
            }
        },
        _ => {}
    }
    ::add_warning(context, list_node, ::WarningMessage::ValueUnrecognized);
    None
}

fn parse_template_audio<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter<'a>],
) -> Option<::Audio<'a>> {
    if let [file_name_parameter @ ::Parameter { name: None, .. }, label_parameter @ ::Parameter { name: None, .. }] =
        parameters
    {
        Some(::Audio {
            file_name: parse_parameter_text_not_empty(context, file_name_parameter)?,
            label: parse_parameter_text_not_empty(context, label_parameter)?,
        })
    } else {
        ::add_warning(context, template_node, ::WarningMessage::ValueUnrecognized);
        None
    }
}

fn parse_template_ipa<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter<'a>],
) -> Option<Vec<::Cow<'a, str>>> {
    match parameters {
        [parameter @ ::Parameter { name: None, .. }] => {
            Some(vec![parse_parameter_text_not_empty(context, parameter)?])
        }
        [parameter1 @ ::Parameter { name: None, .. }, parameter2 @ ::Parameter { name: None, .. }] =>
        {
            let value1 = parse_parameter_text_not_empty(context, parameter1)?;
            let value2 = parse_parameter_text_not_empty(context, parameter2)?;
            if value1 == value2 {
                ::add_warning(context, parameter2, ::WarningMessage::Duplicate);
                None
            } else {
                Some(vec![value1, value2])
            }
        }
        _ => {
            ::add_warning(context, template_node, ::WarningMessage::ValueUnrecognized);
            None
        }
    }
}

fn parse_parameter_text_not_empty<'a>(
    context: &mut ::Context,
    parameter: &::Parameter<'a>,
) -> Option<::Cow<'a, str>> {
    let result = ::parse_text_not_empty(&parameter.value);
    if result.is_none() {
        ::add_warning(context, parameter, ::WarningMessage::ValueUnrecognized);
    }
    result
}
