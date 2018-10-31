// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

macro_rules! parse_numeral_declension_basic {
    { $( $variant:ident $( $field:ident ) + ),+ } => {
        pub fn parse_numeral_declension_basic<'a>(
            context: &mut ::Context<'a>,
            template_node: &::Node,
            parameters: &[::Parameter<'a>]
        ) -> Option<::Inflection<'a>> {
            let mut inflection = None;
            for parameter in parameters {
                if let Some(name) = ::parse_parameter_name(parameter) {
                    match name {
                        $( $( stringify!($field) => {
                            if inflection.is_none() {
                                inflection = Some(::Inflection::$variant(Default::default()));
                            }
                            if let Some(::Inflection::$variant(inflection)) = &mut inflection {
                                ::inflection_field::parse_inflection_field(context, parameter, &mut inflection.$field);
                                continue;
                            }
                        } )+ )+,
                        _ => {}
                    }
                }
                ::add_warning(context, parameter, ::WarningMessage::Unrecognized);
                return None;
            }
            if inflection.is_none() {
                ::add_warning(context, template_node, ::WarningMessage::Empty);
            }
            inflection
        }
    }
}

parse_numeral_declension_basic! {
    NumeralDeclensionBasic acc dat gen ins loc nom voc,
    NumeralDeclensionSgPl pacc pdat pgen pins ploc pnom pvoc sacc sdat sgen sins sloc snom svoc
}
