// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_proverbs<'a>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    output: &mut Option<Option<Vec<::Cow<'a, str>>>>,
) -> usize {
    ::parse_unordered_list_section_basic(
        context,
        heading_node,
        nodes,
        output,
        |context, _list_node, list_items| {
            Some(
                list_items
                    .iter()
                    .filter_map(|item| {
                        if let [::Node::Link { target, text, .. }] = item.nodes.as_slice() {
                            if ::text_equals(text, target) {
                                return Some(::Cow::Borrowed(*target));
                            }
                        }
                        ::add_warning(context, item, ::WarningMessage::ValueUnrecognized);
                        None
                    })
                    .collect(),
            )
        },
    )
}

pub fn parse_synonyms<'a>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    output: &mut Option<Option<Vec<Vec<::Flowing<'a>>>>>,
) -> usize {
    ::parse_ordered_list_section(
        context,
        heading_node,
        nodes,
        output,
        |context, list_item| match list_item.nodes.as_slice() {
            [::Node::Text { value, .. }] if *value == "—" || *value == "–" => vec![],
            _ => parse_term_list_item(context, list_item),
        },
    )
}

pub fn parse_related_terms<'a>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    output: &mut Option<Option<Vec<Vec<::Flowing<'a>>>>>,
) -> usize {
    ::parse_unordered_list_section_basic(
        context,
        heading_node,
        nodes,
        output,
        |context, _list_node, list_items| {
            Some(
                list_items
                    .iter()
                    .map(|item| parse_term_list_item(context, item))
                    .collect(),
            )
        },
    )
}

fn parse_term_list_item<'a>(
    context: &mut ::Context<'a>,
    list_item: &::ListItem<'a>,
) -> Vec<::Flowing<'a>> {
    list_item
        .nodes
        .iter()
        .map(|node| {
            match node {
                ::Node::Link { target, text, .. } => {
                    return ::parse_link(context, node, target, text)
                }
                ::Node::Template {
                    name, parameters, ..
                } => if let Some(name) = ::parse_text(name) {
                    match &name as _ {
                        "Příznak2" => {
                            let labels =
                                ::templates::parse_template_labels(context, node, parameters);
                            if !labels.is_empty() {
                                return ::Flowing::Labels { labels };
                            }
                        }
                        "Upřesnění" => if let Some(result) =
                            ::templates::parse_template_qualifier(parameters)
                        {
                            return result;
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
