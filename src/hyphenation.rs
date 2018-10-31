// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_hyphenation<'a>(
    context: &mut ::Context<'a>,
    node: &::Node<'a>,
    nodes: &[::Node<'a>],
    output: &mut Option<Option<::Cow<'a, str>>>,
) -> usize {
    ::parse_unordered_list_section_basic(
        context,
        node,
        nodes,
        output,
        |context, list_node, list_items| {
            if let [item] = list_items {
                if let text @ Some(_) = ::parse_text(&item.nodes) {
                    return text;
                }
            }
            ::add_warning(context, list_node, ::WarningMessage::ValueUnrecognized);
            None
        },
    )
}
