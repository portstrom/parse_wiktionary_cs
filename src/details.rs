// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_details<'a>(
    context: &mut ::Context<'a>,
    list_node: &::Node,
    list_items: &[::ListItem<'a>],
    output: &mut Option<Vec<Vec<::Flowing<'a>>>>,
) {
    *output = Some(if output.is_some() {
        ::add_warning(context, list_node, ::WarningMessage::Duplicate);
        vec![]
    } else {
        list_items
            .iter()
            .filter_map(|item| {
                if item.nodes.is_empty() {
                    ::add_warning(context, item, ::WarningMessage::Empty);
                    None
                } else {
                    Some(
                        item.nodes
                            .iter()
                            .map(|node| match node {
                                ::Node::Italic { .. } => ::Flowing::Italic,
                                ::Node::Link { target, text, .. } => {
                                    ::parse_link(context, node, target, text)
                                }
                                ::Node::Text { value, .. } => ::Flowing::Text {
                                    value: ::Cow::Borrowed(value),
                                },
                                _ => ::create_unknown(context, node),
                            })
                            .collect(),
                    )
                }
            })
            .collect()
    });
}
