use nom::error::VerboseError;
use nom::{alt, do_parse, named, tag, tag_no_case, opt};

use super::{parse_parameters, Property};
use crate::parse::{parse_multiple_value, parse_name, parse_typed_value, parse_value};
use crate::{Parameters, Value};

named!(pub parse_property<&str, Property, VerboseError<&str>>, alt!(
//    parse_begin |
//    parse_end |
    parse_logo |
    parse_source |
    parse_kind |
    parse_xml |
    parse_fn |
    parse_n |
    parse_nickname |
    parse_photo |
    parse_birthday |
    parse_anniversary |
    parse_gender |
    parse_address |
    parse_tel |
    parse_email |
    parse_impp |
    parse_lang |
    parse_timezone |
    parse_geo |
    parse_title |
    parse_role |
    parse_organization |
    parse_member |
    parse_related |
    parse_categories |
    parse_note |
    parse_prodid |
    parse_rev |
    parse_sound |
    parse_uid |
    parse_clientpidmap |
    parse_url |
    parse_version |
    parse_key |
    parse_fburl |
    parse_caladrurl |
    parse_calurl |
    parse_other
));

named!(pub parse_begin<&str, Property, VerboseError<&str>>, do_parse!(
    tag_no_case!("BEGIN") >>
    tag!(":") >>
    tag_no_case!("VCARD") >>
    opt!(tag!("\r")) >>
    opt!(tag!("\n")) >>
    (Property { params: Parameters(vec![]), value: Value::Begin })
));

named!(pub parse_end<&str, Property, VerboseError<&str>>, do_parse!(
    tag_no_case!("END") >>
    tag!(":") >>
    tag_no_case!("VCARD") >>
    opt!(tag!("\r")) >>
    opt!(tag!("\n")) >>
    (Property { params: Parameters(vec![]), value: Value::End })
));

macro_rules! impl_simple_prop_parser {
    ($label:ident, $txt:expr, $variant:ident) => {
        named!($label<&str, Property, VerboseError<&str>>, do_parse!(
            tag_no_case!($txt) >>
            params: parse_parameters >>
            tag!(":") >>
            value: parse_value >>
            opt!(tag!("\r")) >>
            (Property { params: Parameters(params), value: Value::$variant(value) })
        ));
    };
    ($label:ident, $txt:expr, $variant:ident, $value_func:ident) => {
        named!($label<&str, Property, VerboseError<&str>>, do_parse!(
            tag_no_case!($txt) >>
            params: parse_parameters >>
            tag!(":") >>
            value: $value_func >>
            opt!(tag!("\r")) >>
            (Property { params: Parameters(params), value: Value::$variant(value) })
        ));
    };
}

impl_simple_prop_parser!(parse_source, "SOURCE", Source, parse_typed_value);
impl_simple_prop_parser!(parse_kind, "KIND", Kind, parse_typed_value);
impl_simple_prop_parser!(parse_xml, "XML", Xml);
impl_simple_prop_parser!(parse_fn, "FN", Fn);
impl_simple_prop_parser!(parse_n, "N", N, parse_typed_value);
impl_simple_prop_parser!(parse_nickname, "NICKNAME", Nickname, parse_multiple_value);
impl_simple_prop_parser!(parse_photo, "PHOTO", Photo, parse_typed_value);
impl_simple_prop_parser!(parse_geo, "GEO", Geo, parse_typed_value);
impl_simple_prop_parser!(parse_birthday, "BDAY", Birthday, parse_typed_value);
impl_simple_prop_parser!(
    parse_anniversary,
    "ANNIVERSARY",
    Anniversary,
    parse_typed_value
);
impl_simple_prop_parser!(parse_gender, "GENDER", Gender, parse_typed_value);
impl_simple_prop_parser!(parse_address, "ADR", Adr, parse_typed_value);
impl_simple_prop_parser!(parse_tel, "TEL", Tel);
impl_simple_prop_parser!(parse_email, "EMAIL", Email, parse_typed_value);
impl_simple_prop_parser!(parse_impp, "IMPP", Impp);
impl_simple_prop_parser!(parse_lang, "LANG", Lang);
impl_simple_prop_parser!(parse_timezone, "TZ", Timezone);
impl_simple_prop_parser!(parse_title, "TITLE", Title);
impl_simple_prop_parser!(parse_role, "ROLE", Role);
impl_simple_prop_parser!(parse_logo, "LOGO", Logo, parse_typed_value);
impl_simple_prop_parser!(parse_organization, "ORG", Organization);
impl_simple_prop_parser!(parse_member, "MEMBER", Member);
impl_simple_prop_parser!(parse_related, "RELATED", Related);
impl_simple_prop_parser!(
    parse_categories,
    "CATEGORIES",
    Categories,
    parse_multiple_value
);
impl_simple_prop_parser!(parse_note, "NOTE", Note);
impl_simple_prop_parser!(parse_prodid, "PRODID", ProdID);
impl_simple_prop_parser!(parse_rev, "REV", Rev);
impl_simple_prop_parser!(parse_sound, "SOUND", Sound);
impl_simple_prop_parser!(parse_uid, "UID", Uid);
impl_simple_prop_parser!(parse_clientpidmap, "CLIENTPIDMAP", ClientPIDMap);
impl_simple_prop_parser!(parse_url, "URL", Url, parse_typed_value);
impl_simple_prop_parser!(parse_version, "VERSION", Version);
impl_simple_prop_parser!(parse_key, "KEY", Key);
impl_simple_prop_parser!(parse_fburl, "FBURL", FbUrl);
impl_simple_prop_parser!(parse_calurl, "CALURL", CalURL);
impl_simple_prop_parser!(parse_caladrurl, "CALADRURL", CalAdrURL);

named!(parse_other<&str, Property, VerboseError<&str>>, do_parse!(
    name: parse_name >>
    params: parse_parameters >>
    tag!(":") >>
    value: parse_value >>
    ( Property { params: Parameters(params), value: Value::Other(name, value) })
));
