// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

macro_rules! parse_conjugation_template {
    { fields { $( $field_name:ident )+ } groups { $( $flag_name:ident $flag_value:tt $( $group_field_name:ident )* ),+ } } => {
        pub fn parse_conjugation_template<'a>(
            context: &mut ::Context<'a>,
            _template_node: &::Node,
            parameters: &[::Parameter<'a>]
        ) -> Option<::Inflection<'a>> {
            $( let mut $flag_name = None; )+
            let mut conjugation = super::Conjugation::default();
            for parameter in parameters {
                if let Some(name) = ::parse_parameter_name(parameter) {
                    match name {
                        $( stringify!($field_name) => {
                            ::inflection_field::parse_inflection_field(context, parameter, &mut conjugation.$field_name);
                            continue;
                        } )+,
                        $( stringify!($flag_name) => {
                            if $flag_name.is_none() {
                                if ::text_equals(&parameter.value, $flag_value) {
                                    $flag_name = Some(true);
                                    continue;
                                }
                            }
                        }
                           $( stringify!($group_field_name) => {
                               if $flag_name != Some(true) {
                                   $flag_name = Some(false);
                                   ::inflection_field::parse_inflection_field(context, parameter, &mut conjugation.$group_field_name);
                                   continue;
                               }
                           } )* )+,
                        _ => {}
                    }
                }
                ::add_warning(context, parameter, ::WarningMessage::Unrecognized);
                return None;
            }
            Some(::Inflection::Conjugation(conjugation))
        }
    }
}

parse_conjugation_template! {
    fields { pactf pactm pimp1 pimp2 ppre1 ppre2 ppre3 sactf sactm sactn simp2 spre1 spre2 spre3 }
    groups {
        dok "ano",
        mtra "skrýt" mtraf mtram mtrap,
        pas "skrýt" ppasf ppasm spasf spasm spasn,
        ptra "skrýt" ptraf ptram ptrap
    }
}
