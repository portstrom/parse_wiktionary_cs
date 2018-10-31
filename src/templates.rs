// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_template_labels<'a>(
    context: &mut ::Context,
    template_node: &::Node,
    parameters: &[::Parameter<'a>],
) -> Vec<::Cow<'a, str>> {
    let mut labels = vec![];
    for parameter in parameters {
        if parameter.name.is_some() {
            ::add_warning(context, parameter, ::WarningMessage::Unrecognized);
            continue;
        }
        match ::parse_text_not_empty(&parameter.value) {
            None => ::add_warning(context, parameter, ::WarningMessage::ValueUnrecognized),
            Some(text) => if labels.iter().any(|label| label == &text) {
                ::add_warning(context, parameter, ::WarningMessage::Duplicate);
            } else {
                labels.push(text);
            },
        }
    }
    if labels.is_empty() {
        ::add_warning(context, template_node, ::WarningMessage::Empty);
    }
    labels
}

pub fn parse_template_qualifier<'a>(parameters: &[::Parameter<'a>]) -> Option<::Flowing<'a>> {
    if let [::Parameter {
        name: None, value, ..
    }] = parameters
    {
        if let Some(value) = ::parse_text_not_empty(value) {
            return Some(::Flowing::Qualifier { value });
        }
    }
    None
}
