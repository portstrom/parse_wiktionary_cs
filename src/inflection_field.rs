// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

macro_rules! unrecognized {
    ($context:expr, $node:expr) => {{
        ::add_warning($context, $node, ::WarningMessage::Unrecognized);
        return;
    }};
}

pub fn parse_inflection_field<'a>(
    context: &mut ::Context<'a>,
    parameter: &::Parameter<'a>,
    output: &mut Vec<::Cow<'a, str>>,
) {
    if !output.is_empty() {
        ::add_warning(context, parameter, ::WarningMessage::Duplicate);
    }
    let mut expecting_separator = false;
    let mut terms = vec![];
    for node in &parameter.value {
        match node {
            ::Node::Link { target, text, .. } => {
                if expecting_separator || !::text_equals(text, target) {
                    unrecognized!(context, parameter);
                }
                expecting_separator = true;
                terms.push(::Cow::Borrowed(*target));
            }
            ::Node::Text { value, .. } => {
                let mut scan_position = 0;
                let mut start_position = 0;
                loop {
                    match value.as_bytes().get(scan_position).cloned() {
                        Some(b',') | Some(b'/') => {
                            let term = &value[start_position..scan_position].trim();
                            match (expecting_separator, term.is_empty()) {
                                (true, true) => expecting_separator = false,
                                (false, false) => terms.push(::Cow::Borrowed(term)),
                                _ => unrecognized!(context, parameter),
                            }
                            scan_position += 1;
                            start_position = scan_position;
                        }
                        Some(_) => scan_position += 1,
                        None => break,
                    }
                }
                if expecting_separator {
                    unrecognized!(context, parameter);
                }
                let term = &value[start_position..scan_position].trim();
                if !term.is_empty() {
                    expecting_separator = true;
                    terms.push(::Cow::Borrowed(term));
                }
            }
            _ => {}
        }
    }
    if terms.is_empty() {
        ::add_warning(context, parameter, ::WarningMessage::Empty);
    }
    *output = terms;
}
