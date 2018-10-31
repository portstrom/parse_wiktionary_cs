// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_comparison_template<'a>(
    context: &mut ::Context<'a>,
    template_node: &::Node,
    parameters: &[::Parameter<'a>],
) -> Option<::Inflection<'a>> {
    let mut positive = (None, None);
    let mut comparative = (None, None, None);
    let mut superlative = (None, None, None);
    for parameter in parameters {
        if let Some(name) = ::parse_parameter_name(parameter) {
            let output = match &name as _ {
                "komp" => &mut comparative.0,
                "komp2" => &mut comparative.1,
                "komp3" => &mut comparative.2,
                "poz" => &mut positive.0,
                "poz2" => &mut positive.1,
                "sup" => &mut superlative.0,
                "sup2" => &mut superlative.1,
                "sup3" => &mut superlative.2,
                _ => {
                    ::add_warning(context, parameter, ::WarningMessage::Unrecognized);
                    return None;
                }
            };
            if output.is_some() {
                ::add_warning(context, parameter, ::WarningMessage::Duplicate);
                return None;
            }
            *output = ::parse_text_not_empty(&parameter.value);
            if output.is_none() {
                ::add_warning(context, parameter, ::WarningMessage::ValueUnrecognized);
                return None;
            }
        }
    }
    if positive.0.is_none() {
        ::add_warning(context, template_node, ::WarningMessage::Empty);
        return None;
    }
    Some(::Inflection::Comparison(super::Comparison {
        comparative: match comparative {
            (None, None, None) => vec![],
            (Some(item1), None, None) => vec![item1],
            (Some(item1), Some(item2), None) => vec![item1, item2],
            (Some(item1), Some(item2), Some(item3)) => vec![item1, item2, item3],
            _ => {
                ::add_warning(context, template_node, ::WarningMessage::ValueUnrecognized);
                return None;
            }
        },
        positive: match positive {
            (None, _) => {
                ::add_warning(context, template_node, ::WarningMessage::Empty);
                return None;
            }
            (Some(item1), None) => vec![item1],
            (Some(item1), Some(item2)) => vec![item1, item2],
        },
        superlative: match superlative {
            (None, None, None) => vec![],
            (Some(item1), None, None) => vec![item1],
            (Some(item1), Some(item2), None) => vec![item1, item2],
            (Some(item1), Some(item2), Some(item3)) => vec![item1, item2, item3],
            _ => {
                ::add_warning(context, template_node, ::WarningMessage::ValueUnrecognized);
                return None;
            }
        },
    }))
}
