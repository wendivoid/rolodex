use rolodex::types::*;
use rolodex::*;

const DATA: &'static str = "BEGIN:VCARD
VERSION:2.1
N:Gump;Forrest;;Mr.
FN:Forrest Gump
ORG:Bubba Gump Shrimp Co.
TITLE:Shrimp Man
PHOTO;GIF:http://www.example.com/dir_photos/my_photo.gif
TEL;WORK;VOICE:(111) 555-1212
TEL;HOME;VOICE:(404) 555-1212
ADR;WORK;PREF:;;100 Waters Edge;Baytown;LA;30314;United States of America
LABEL;WORK;PREF;ENCODING=QUOTED-PRINTABLE;CHARSET=UTF-8:100 Waters Edge=0D=0A=\\n Baytown\\, LA 30314=0D=0AUnited States of America
ADR;HOME:;;42 Plantation St.;Baytown;LA;30314;United States of America
LABEL;HOME;ENCODING=QUOTED-PRINTABLE;CHARSET=UTF-8:42 Plantation St.=0D=0A=\\n Baytown, LA 30314=0D=0AUnited States of America
EMAIL:forrestgump@example.com
REV:20080424T195243Z
END:VCARD\r\n";

fn name() -> TypeOrRaw<FormattedName<'static>> {
    TypeOrRaw::Type(FormattedName {
        surname: vec!["Gump".into()],
        given: vec!["Forrest".into()],
        additional: vec![],
        prefix: vec!["Mr.".into()],
        suffix: vec![],
    })
}

fn address1() -> TypeOrRaw<Address<'static>> {
    TypeOrRaw::Type(Address {
        po_box: None,
        extended: None,
        street: Some("100 Waters Edge".into()),
        region: Some("LA".into()),
        locality: Some("Baytown".into()),
        code: Some("30314".into()),
        country: Some("United States of America".into()),
    })
}

fn address2() -> TypeOrRaw<Address<'static>> {
    TypeOrRaw::Type(Address {
        po_box: None,
        extended: None,
        street: Some("42 Plantation St.".into()),
        region: Some("LA".into()),
        locality: Some("Baytown".into()),
        code: Some("30314".into()),
        country: Some("United States of America".into()),
    })
}

fn expected() -> VCard<'static> {
    VCard(vec![
        Property::builder()
            .value(Value::Version("2.1".into()))
            .build(),
        Property::builder().value(Value::N(name())).build(),
        Property::builder()
            .value(Value::Fn("Forrest Gump".into()))
            .build(),
        Property::builder()
            .value(Value::Organization("Bubba Gump Shrimp Co.".into()))
            .build(),
        Property::builder()
            .value(Value::Title("Shrimp Man".into()))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder().name("GIF").build()]))
            .value(Value::Photo(TypeOrRaw::Type(Image::Url(
                Url::builder()
                    .schema("http")
                    .domain("www.example.com")
                    .path("/dir_photos/my_photo.gif")
                    .build(),
            ))))
            .build(),
        Property::builder()
            .params(Parameters(vec![
                Parameter::builder().name("WORK").build(),
                Parameter::builder().name("VOICE").build(),
            ]))
            .value(Value::Tel("(111) 555-1212".into()))
            .build(),
        Property::builder()
            .params(Parameters(vec![
                Parameter::builder().name("HOME").build(),
                Parameter::builder().name("VOICE").build(),
            ]))
            .value(Value::Tel("(404) 555-1212".into()))
            .build(),
        Property::builder()
            .params(Parameters(vec![
                Parameter::builder().name("WORK").build(),
                Parameter::builder().name("PREF").build(),
            ]))
            .value(Value::Adr(address1()))
            .build(),
        Property::builder()
            .params(Parameters(vec![
                Parameter::builder().name("WORK").build(),
                Parameter::builder().name("PREF").build(),
                Parameter::builder()
                    .name("ENCODING")
                    .value("QUOTED-PRINTABLE")
                    .build(),
                Parameter::builder().name("CHARSET").value("UTF-8").build(),
            ]))
            .value(Value::Other(
                "LABEL".into(),
                "100 Waters Edge=0D=0A=\\n Baytown\\, LA 30314=0D=0AUnited States of America"
                    .into(),
            ))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder().name("HOME").build()]))
            .value(Value::Adr(address2()))
            .build(),
        Property::builder()
            .params(Parameters(vec![
                Parameter::builder().name("HOME").build(),
                Parameter::builder()
                    .name("ENCODING")
                    .value("QUOTED-PRINTABLE")
                    .build(),
                Parameter::builder().name("CHARSET").value("UTF-8").build(),
            ]))
            .value(Value::Other(
                "LABEL".into(),
                "42 Plantation St.=0D=0A=\\n Baytown, LA 30314=0D=0AUnited States of America"
                    .into(),
            ))
            .build(),
        Property::builder()
            .value(Value::Email(TypeOrRaw::Type(
                Email::builder()
                    .user("forrestgump")
                    .domain("example.com")
                    .build(),
            )))
            .build(),
        Property::builder()
            .value(Value::Rev("20080424T195243Z".into()))
            .build(),
    ])
}

#[test]
fn vcard_round_trip() {
    match VCard::parse(DATA) {
        Err(nom::Err::Failure(err)) => panic!("{}", err.display(DATA)),
        Err(nom::Err::Error(err)) => panic!("{}", err.display(DATA)),
        Err(err) => panic!("{}", err),
        Ok(item) => {
            let expected = expected();
            let mut expected_iter = expected.iter();
            for item in item.0.iter() {
                assert_eq!(item, expected_iter.next().unwrap());
            }
        }
    }
}

#[cfg(feature = "serde")]
#[test]
fn json_round_trip() {
    let data = expected();
    let json_data = serde_json::to_string(&data).unwrap();
    let result: VCard = serde_json::from_str(&json_data).unwrap();
    assert_eq!(result, data);
}
