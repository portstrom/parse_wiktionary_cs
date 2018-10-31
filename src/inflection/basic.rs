// Copyright 2018 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

pub trait Inflection<'a> {
    fn get_field(&mut self, name: &str) -> Option<&mut Vec<::Cow<'a, str>>>;
}

macro_rules! inflection_impl {
    ( $type:tt $( , $name:tt $field:tt )+ ) => {
        impl<'a> Inflection<'a> for super::$type<'a> {
            fn get_field(&mut self, name: &str) -> Option<&mut Vec<::Cow<'a, str>>> {
                Some(match name {
                    $( $name => &mut self.$field, )+
                    _ => return None
                })
            }
        }
    }
}

inflection_impl! {
    AdjectiveDeclensionBasic,
    "paccf" paccf,
    "paccf-jm" paccf_jm,
    "paccm" paccm,
    "paccm-jm" paccm_jm,
    "paccma" paccma,
    "paccma-jm" paccma_jm,
    "paccn" paccn,
    "paccn-jm" paccn_jm,
    "pdatf" pdatf,
    "pdatm" pdatm,
    "pdatma" pdatma,
    "pdatn" pdatn,
    "pgenf" pgenf,
    "pgenm" pgenm,
    "pgenma" pgenma,
    "pgenn" pgenn,
    "pinsf" pinsf,
    "pinsm" pinsm,
    "pinsma" pinsma,
    "pinsn" pinsn,
    "plocf" plocf,
    "plocm" plocm,
    "plocma" plocma,
    "plocn" plocn,
    "pnomf" pnomf,
    "pnomf-jm" pnomf_jm,
    "pnomm" pnomm,
    "pnomm-jm" pnomm_jm,
    "pnomma" pnomma,
    "pnomma-jm" pnomma_jm,
    "pnomn" pnomn,
    "pnomn-jm" pnomn_jm,
    "saccf" saccf,
    "saccf-jm" saccf_jm,
    "saccm" saccm,
    "saccm-jm" saccm_jm,
    "saccma" saccma,
    "saccma-jm" saccma_jm,
    "saccn" saccn,
    "saccn-jm" saccn_jm,
    "sdatf" sdatf,
    "sdatm" sdatm,
    "sdatma" sdatma,
    "sdatn" sdatn,
    "sgenf" sgenf,
    "sgenm" sgenm,
    "sgenma" sgenma,
    "sgenn" sgenn,
    "sinsf" sinsf,
    "sinsm" sinsm,
    "sinsma" sinsma,
    "sinsn" sinsn,
    "slocf" slocf,
    "slocm" slocm,
    "slocma" slocma,
    "slocn" slocn,
    "snomf" snomf,
    "snomf-jm" snomf_jm,
    "snomm" snomm,
    "snomm-jm" snomm_jm,
    "snomma" snomma,
    "snomma-jm" snomma_jm,
    "snomn" snomn,
    "snomn-jm" snomn_jm
}

inflection_impl! {
    PronounDeclensionAdjective,
    "paccf" paccf,
    "paccm" paccm,
    "paccma" paccma,
    "paccn" paccn,
    "pdatf" pdatf,
    "pdatm" pdatm,
    "pdatma" pdatma,
    "pdatn" pdatn,
    "pgenf" pgenf,
    "pgenm" pgenm,
    "pgenma" pgenma,
    "pgenn" pgenn,
    "pinsf" pinsf,
    "pinsm" pinsm,
    "pinsma" pinsma,
    "pinsn" pinsn,
    "plocf" plocf,
    "plocm" plocm,
    "plocma" plocma,
    "plocn" plocn,
    "pnomf" pnomf,
    "pnomm" pnomm,
    "pnomma" pnomma,
    "pnomn" pnomn,
    "pvocf" pvocf,
    "pvocm" pvocm,
    "pvocma" pvocma,
    "pvocn" pvocn,
    "saccf" saccf,
    "saccm" saccm,
    "saccma" saccma,
    "saccn" saccn,
    "sdatf" sdatf,
    "sdatm" sdatm,
    "sdatma" sdatma,
    "sdatn" sdatn,
    "sgenf" sgenf,
    "sgenm" sgenm,
    "sgenma" sgenma,
    "sgenn" sgenn,
    "sinsf" sinsf,
    "sinsm" sinsm,
    "sinsma" sinsma,
    "sinsn" sinsn,
    "slocf" slocf,
    "slocm" slocm,
    "slocma" slocma,
    "slocn" slocn,
    "snomf" snomf,
    "snomm" snomm,
    "snomma" snomma,
    "snomn" snomn,
    "svocf" svocf,
    "svocm" svocm,
    "svocma" svocma,
    "svocn" svocn
}

inflection_impl! {
    PronounDeclensionBasic,
    "acc" acc,
    "dat" dat,
    "gen" gen,
    "ins" ins,
    "loc" loc,
    "nom" nom,
    "voc" voc
}

pub fn parse_inflection_basic<'a, T1: Default + Inflection<'a>, T2>(
    context: &mut ::Context<'a>,
    parameters: &[::Parameter<'a>],
    variant: impl FnOnce(T1) -> T2,
) -> Option<T2> {
    let mut inflection = T1::default();
    for parameter in parameters {
        if let Some(name) = ::parse_parameter_name(parameter) {
            if let Some(terms) = inflection.get_field(&name) {
                ::inflection_field::parse_inflection_field(context, parameter, terms);
                continue;
            }
        }
        ::add_warning(context, parameter, ::WarningMessage::Unrecognized);
    }
    Some(variant(inflection))
}
