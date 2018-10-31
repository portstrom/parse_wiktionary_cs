// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_etymology<'a>(
    context: &mut ::Context<'a>,
    heading_node: &::Node<'a>,
    nodes: &[::Node<'a>],
    output: &mut Option<Option<Vec<::Flowing<'a>>>>,
) -> usize {
    ::parse_section(context, heading_node, output, |context, output| {
        let mut node_index = 0;
        let mut output_nodes = vec![];
        while let Some(node) = nodes.get(node_index) {
            output_nodes.push(match node {
                ::Node::Heading { .. } => break,
                ::Node::Italic { .. } => ::Flowing::Italic,
                ::Node::Link { target, text, .. } => ::parse_link(context, node, target, text),
                ::Node::Text { value, .. } => ::Flowing::Text {
                    value: ::Cow::Borrowed(value),
                },
                _ => ::create_unknown(context, node),
            });
            node_index += 1;
        }
        if output_nodes.is_empty() {
            ::add_warning(context, heading_node, ::WarningMessage::SectionEmpty);
        }
        *output = Some(Some(output_nodes));
        node_index
    })
}
