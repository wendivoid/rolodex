use rolodex::types::*;
use rolodex::*;

const DATA: &'static str = "BEGIN:VCARD
VERSION:4.0
N:Gump;Forrest;;Mr.;
FN:Forrest Gump
ORG:Bubba Gump Shrimp Co.
TITLE:Shrimp Man
PHOTO;MEDIATYPE=image/gif:http://www.example.com/dir_photos/my_photo.gif
TEL;TYPE=work,voice;VALUE=uri:tel:+1-111-555-1212
TEL;TYPE=home,voice;VALUE=uri:tel:+1-404-555-1212
ADR;TYPE=WORK;PREF=1;LABEL=\"100 Waters Edge\nBaytown\\, LA 30314\nUnited States of America\":;;100 Waters Edge;Baytown;LA;30314;United States of America
ADR;TYPE=HOME;LABEL=\"42 Plantation St.\nBaytown\\, LA 30314\nUnited States of America\":;;42 Plantation St.;Baytown;LA;30314;United States of America
EMAIL:forrestgump@example.com
REV:20080424T195243Z
x-qq:21588891
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
            .value(Value::Version("4.0".into()))
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
            .params(Parameters(vec![Parameter::builder()
                .name("MEDIATYPE")
                .value("image/gif")
                .build()]))
            .value(Value::Photo(TypeOrRaw::Type(Image::Url(
                Url::builder()
                    .schema("http")
                    .domain("www.example.com")
                    .path("/dir_photos/my_photo.gif")
                    .build(),
            ))))
            .build(),
        Property {
            params: Parameters(vec![
                Parameter::builder()
                    .name("TYPE")
                    .value("work,voice")
                    .build(),
                Parameter::builder().name("VALUE").value("uri").build(),
            ]),
            value: Value::Tel("tel:+1-111-555-1212".into()),
        },
        Property {
            params: Parameters(vec![
                Parameter::builder()
                    .name("TYPE")
                    .value("home,voice")
                    .build(),
                Parameter::builder().name("VALUE").value("uri").build(),
            ]),
            value: Value::Tel("tel:+1-404-555-1212".into()),
        },
        Property {
            params: Parameters(vec![
                Parameter::builder().name("TYPE").value("WORK").build(),
                Parameter::builder().name("PREF").value("1").build(),
                Parameter::builder()
                    .name("LABEL")
                    .value("\"100 Waters Edge\nBaytown\\, LA 30314\nUnited States of America\"")
                    .build(),
            ]),
            value: Value::Adr(address1()),
        },
        Property {
            params: Parameters(vec![
                Parameter::builder().name("TYPE").value("HOME").build(),
                Parameter::builder()
                    .name("LABEL")
                    .value("\"42 Plantation St.\nBaytown\\, LA 30314\nUnited States of America\"")
                    .build(),
            ]),
            value: Value::Adr(address2()),
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
            .value(Value::Rev("20080424T195243Z".into()))
            .build(),
        Property::builder()
            .value(Value::Other("x-qq".into(), "21588891".into()))
            .build(),
        Property::builder().value(Value::End).build(),
    ])
}

#[test]
fn vcard_round_trip() {
    match VCard::parse(DATA) {
        Err(nom::Err::Failure(err)) => panic!("\n{}", err.display(DATA)),
        Err(nom::Err::Error(err)) => panic!("\n{}", err.display(DATA)),
        Err(err) => panic!("\n{}", err),
        Ok(item) => assert_eq!(expected(), item),
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
