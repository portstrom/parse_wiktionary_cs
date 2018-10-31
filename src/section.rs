// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

#[must_use]
pub fn parse_section<'a, T>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    output: &mut Option<Option<T>>,
    parse_section: impl FnOnce(&mut ::Context<'a>, &mut Option<Option<T>>) -> usize,
) -> usize {
    match output {
        None => parse_section(context, output),
        Some(_) => {
            *output = Some(None);
            ::add_warning(context, heading_node, ::WarningMessage::Duplicate);
            0
        }
    }
}

#[must_use]
pub fn parse_section_nodes<'a, T>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    output: &mut Option<Option<T>>,
    mut parse_node: impl FnMut(&mut ::Context<'a>, &::Node<'a>, &mut Option<Option<T>>) -> bool,
) -> usize {
    parse_section(context, heading_node, output, |context, output| {
        let mut node_index = 0;
        while let Some(node) = nodes.get(node_index) {
            match node {
                ::Node::Heading { .. } => break,
                _ => if !parse_node(context, node, output) {
                    ::unrecognized_unless_ignored(context, node);
                },
            }
            node_index += 1;
        }
        if output.is_none() {
            *output = Some(None);
            ::add_warning(context, heading_node, ::WarningMessage::SectionEmpty);
        }
        node_index
    })
}

#[must_use]
pub fn parse_ordered_list_section<'a, T>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    output: &mut Option<Option<Vec<T>>>,
    mut parse_list_item: impl FnMut(&mut ::Context<'a>, &::ListItem<'a>) -> T,
) -> usize {
    parse_ordered_list_section_basic(
        context,
        heading_node,
        nodes,
        output,
        |context, _list_node, list_items| {
            Some(
                list_items
                    .iter()
                    .map(|item| parse_list_item(context, item))
                    .collect(),
            )
        },
    )
}

macro_rules! parse_list_section_basic {
    ($name:ident $node_type:ident) => {
        #[must_use]
        pub fn $name<'a, T>(
            context: &mut ::Context<'a>,
            heading_node: &::Node,
            nodes: &[::Node<'a>],
            output: &mut Option<Option<T>>,
            mut parse_list: impl FnMut(&mut ::Context<'a>, &::Node, &[::ListItem<'a>]) -> Option<T>,
        ) -> usize {
            parse_section_nodes(
                context,
                heading_node,
                nodes,
                output,
                |context, node, output| {
                    if let ::Node::$node_type { items, .. } = node {
                        *output = Some(if output.is_some() {
                            ::add_warning(context, node, ::WarningMessage::Duplicate);
                            None
                        } else {
                            parse_list(context, node, items)
                        });
                        true
                    } else {
                        false
                    }
                },
            )
        }
    };
}

parse_list_section_basic! { parse_ordered_list_section_basic OrderedList }
parse_list_section_basic! { parse_unordered_list_section_basic UnorderedList }
