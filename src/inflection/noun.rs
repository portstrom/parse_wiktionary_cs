// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub fn parse_noun_declension_basic<'a>(
    context: &mut ::Context<'a>,
    _template_node: &::Node,
    parameters: &[::Parameter<'a>],
) -> Option<::Inflection<'a>> {
    let mut inflection = super::NounDeclensionBasic::default();
    let mut indeclinable = None;
    for parameter in parameters {
        if let Some(name) = ::parse_parameter_name(parameter) {
            let terms = match &name as _ {
                "dacc" => &mut inflection.dacc,
                "ddat" => &mut inflection.ddat,
                "dgen" => &mut inflection.dgen,
                "dins" => &mut inflection.dins,
                "dloc" => &mut inflection.dloc,
                "dnom" => &mut inflection.dnom,
                "dvoc" => &mut inflection.dvoc,
                "pacc" => &mut inflection.pacc,
                "pdat" => &mut inflection.pdat,
                "pgen" => &mut inflection.pgen,
                "pins" => &mut inflection.pins,
                "ploc" => &mut inflection.ploc,
                "pnom" => &mut inflection.pnom,
                "pvoc" => &mut inflection.pvoc,
                "sacc" => &mut inflection.sacc,
                "sdat" => &mut inflection.sdat,
                "sgen" => &mut inflection.sgen,
                "sins" => &mut inflection.sins,
                "sloc" => &mut inflection.sloc,
                "snom" => &mut inflection.snom,
                "svoc" => &mut inflection.svoc,
                _ => {
                    ::add_warning(context, parameter, ::WarningMessage::Unrecognized);
                    return None;
                }
            };
            if indeclinable != Some(true) {
                indeclinable = Some(false);
                ::inflection_field::parse_inflection_field(context, parameter, terms);
                continue;
            }
        } else if parameter.name.is_none()
            && indeclinable.is_none()
            && ::text_equals(&parameter.value, "nesklonné")
        {
            indeclinable = Some(true);
            continue;
        }
        ::add_warning(context, parameter, ::WarningMessage::Unrecognized);
        return None;
    }
    Some(if indeclinable == Some(true) {
        ::Inflection::Indeclinable
    } else {
        ::Inflection::NounDeclensionBasic(inflection)
    })
}
