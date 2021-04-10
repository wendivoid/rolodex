use rolodex::types::*;
use rolodex::*;

const DATA: &'static str = "BEGIN:VCARD
VERSION:3.0
N:Gump;Forrest;;Mr.;
FN:Forrest Gump
ORG:Bubba Gump Shrimp Co.
TITLE:Shrimp Man
PHOTO;VALUE=URI;TYPE=GIF:http://www.example.com/dir_photos/my_photo.gif
TEL;TYPE=WORK,VOICE:(111) 555-1212
TEL;TYPE=HOME,VOICE:(404) 555-1212
ADR;TYPE=WORK,PREF:;;100 Waters Edge;Baytown;LA;30314;United States of America
LABEL;TYPE=WORK,PREF:100 Waters Edge\\nBaytown\\, LA 30314\\nUnited States of America
ADR;TYPE=HOME:;;42 Plantation St.;Baytown;LA;30314;United States of America
LABEL;TYPE=HOME:42 Plantation St.\\nBaytown\\, LA 30314\\nUnited States of America
EMAIL:forrestgump@example.com
REV:2008-04-24T19:52:43Z
END:VCARD";

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
        Property::builder().value(Value::Begin).build(),
        Property::builder()
            .value(Value::Version("3.0".into()))
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
        Property {
            params: Parameters(vec![
                Parameter::builder().name("VALUE").value("URI").build(),
                Parameter::builder().name("TYPE").value("GIF").build(),
            ]),
            value: Value::Photo(TypeOrRaw::Type(Image::Url(
                Url::builder()
                    .schema("http")
                    .domain("www.example.com")
                    .path("/dir_photos/my_photo.gif")
                    .build(),
            ))),
        },
        Property {
            params: Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("WORK,VOICE")
                .build()]),
            value: Value::Tel("(111) 555-1212".into()),
        },
        Property {
            params: Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("HOME,VOICE")
                .build()]),
            value: Value::Tel("(404) 555-1212".into()),
        },
        Property {
            params: Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("WORK,PREF")
                .build()]),
            value: Value::Adr(address1()),
        },
        Property {
            params: Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("WORK,PREF")
                .build()]),
            value: Value::Other(
                "LABEL".into(),
                "100 Waters Edge\\nBaytown\\, LA 30314\\nUnited States of America".into(),
            ),
        },
        Property {
            params: Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("HOME")
                .build()]),
            value: Value::Adr(address2()),
        },
        Property {
            params: Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("HOME")
                .build()]),
            value: Value::Other(
                "LABEL".into(),
                "42 Plantation St.\\nBaytown\\, LA 30314\\nUnited States of America".into(),
            ),
        },
        Property::builder()
            .value(Value::Email(TypeOrRaw::Type(
                Email::builder()
                    .user("forrestgump")
                    .domain("example.com")
                    .build(),
            )))
            .build(),
        Property::builder()
            .value(Value::Rev("2008-04-24T19:52:43Z".into()))
            .build(),
        Property::builder().value(Value::End).build(),
    ])
}

#[test]
fn vcard_round_trip() {
    match VCard::parse(DATA) {
        Err(nom::Err::Failure(err)) => panic!("{}", err.display(DATA)),
        Err(nom::Err::Error(err)) => panic!("{}", err.display(DATA)),
        Err(err) => panic!("{}", err),
        Ok(item) => assert_eq!(item, expected()),
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
