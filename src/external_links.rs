// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_external_links<'a>(
    context: &mut ::Context<'a>,
    heading_node: &::Node,
    nodes: &[::Node<'a>],
    output: &mut Option<Option<Vec<::ExternalLink<'a>>>>,
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
                        let result = if let [::Node::Template {
                            name, parameters, ..
                        }] = item.nodes.as_slice()
                        {
                            ::parse_text(name).and_then(|name| match &name as _ {
                                "Commons" => parse_template(
                                    parameters,
                                    ::ExternalLinkType::Commons,
                                    &["galerie", "kategorie"],
                                ),
                                "Wikicesty" => parse_template(
                                    parameters,
                                    ::ExternalLinkType::Wikivoyage,
                                    &["kategorie", "průvodce"],
                                ),
                                "Wikicitáty" => parse_template(
                                    parameters,
                                    ::ExternalLinkType::Wikiquote,
                                    &["dílo", "kategorie", "osoba", "téma"],
                                ),
                                "Wikidruhy" => parse_template(
                                    parameters,
                                    ::ExternalLinkType::Wikispecies,
                                    &["kategorie", "taxon"],
                                ),
                                "Wikiknihy" => parse_template(
                                    parameters,
                                    ::ExternalLinkType::Wikibooks,
                                    &["kategorie", "kniha"],
                                ),
                                "Wikipedie" => parse_template(
                                    parameters,
                                    ::ExternalLinkType::Wikipedia,
                                    &["kategorie", "portál", "rozcestník", "článek"],
                                ),
                                "Wikiverzita" => parse_template(
                                    parameters,
                                    ::ExternalLinkType::Wikiversity,
                                    &["kategorie", "kurs"],
                                ),
                                "Wikizdroje" => parse_template(
                                    parameters,
                                    ::ExternalLinkType::Wikisource,
                                    &["OSN", "autor", "dílo", "kategorie", "rozcestník"],
                                ),
                                "Wikizprávy" => parse_template(
                                    parameters,
                                    ::ExternalLinkType::Wikinews,
                                    &["kategorie", "zpráva"],
                                ),
                                _ => None,
                            })
                        } else {
                            None
                        };
                        if result.is_none() {
                            ::add_warning(context, item, ::WarningMessage::ValueUnrecognized);
                        }
                        result
                    })
                    .collect(),
            )
        },
    )
}

fn parse_template<'a>(
    parameters: &[::Parameter<'a>],
    type_: ::ExternalLinkType,
    kinds: &[&str],
) -> Option<::ExternalLink<'a>> {
    if let [parameter] = parameters {
        if let Some(kind) = ::parse_parameter_name(parameter) {
            if kinds.iter().any(|item| kind == *item) {
                if let Some(value) = ::parse_text(&parameter.value) {
                    return Some(::ExternalLink {
                        kind: ::Cow::Borrowed(kind),
                        type_,
                        value,
                    });
                }
            }
        }
    }
    None
}
