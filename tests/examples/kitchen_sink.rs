use rolodex::types::*;
use rolodex::*;

const DATA: &'static str = "BEGIN:VCARD
VERSION:3.0
FN;CHARSET=UTF-8:John D Doe
N;CHARSET=UTF-8:Doe;John;D;MR;JR
GENDER:M
LOGO;TYPE=png:https://testurl
PHOTO;TYPE=png:https://testurl
UID;CHARSET=UTF-8:69531f4a-c34d-4a1e-8922-bd38a9476a53
EMAIL;CHARSET=UTF-8;type=HOME,INTERNET:john.doe@testmail
EMAIL;CHARSET=UTF-8;type=WORK,INTERNET:john.doe@workmail
TEL;TYPE=CELL:12345678900
TEL;TYPE=PAGER:312-555-1515
TEL;TYPE=HOME,VOICE:312-555-1313
TEL;TYPE=WORK,VOICE:312-555-1212
TEL;TYPE=HOME,FAX:312-555-1616
TEL;TYPE=WORK,FAX:312-555-1717
END:VCARD\r\n";

fn name() -> TypeOrRaw<FormattedName<'static>> {
    TypeOrRaw::Type(FormattedName {
        surname: vec!["Doe".into()],
        given: vec!["John".into()],
        additional: vec!["D".into()],
        prefix: vec!["MR".into()],
        suffix: vec!["JR".into()],
    })
}

fn expected() -> VCard<'static> {
    VCard(vec![
        Property::builder()
            .value(Value::Version("3.0".into()))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder()
                .name("CHARSET")
                .value("UTF-8")
                .build()]))
            .value(Value::Fn("John D Doe".into()))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder()
                .name("CHARSET")
                .value("UTF-8")
                .build()]))
            .value(Value::N(name()))
            .build(),
        Property::builder()
            .value(Value::Gender(TypeOrRaw::Type(
                Gender::builder().sex(Sex::Male).build(),
            )))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder()
            .name("TYPE")
            .value("png")
            .build()]))
            .value(Value::Logo(TypeOrRaw::Type(Image::Url(
                Url::builder().schema("https").domain("testurl").build(),
            ))))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder()
            .name("TYPE")
            .value("png")
            .build()]))
            .value(Value::Photo(TypeOrRaw::Type(Image::Url(
                Url::builder().schema("https").domain("testurl").build(),
            ))))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder()
            .name("CHARSET")
            .value("UTF-8")
            .build()]))
            .value(Value::Uid("69531f4a-c34d-4a1e-8922-bd38a9476a53".into()))
            .build(),
        Property::builder()
            .params(Parameters(vec![
                Parameter::builder().name("CHARSET").value("UTF-8").build(),
                Parameter::builder()
                .name("type")
                .value("HOME,INTERNET")
                .build(),
            ]))
            .value(Value::Email(TypeOrRaw::Type(
                Email::builder().user("john.doe").domain("testmail").build(),
            )))
            .build(),
        Property::builder()
            .params(Parameters(vec![
                Parameter::builder().name("CHARSET").value("UTF-8").build(),
                Parameter::builder()
                    .name("type")
                    .value("WORK,INTERNET")
                    .build(),
            ]))
            .value(Value::Email(TypeOrRaw::Type(
                Email::builder().user("john.doe").domain("workmail").build(),
            )))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("CELL")
                .build()]))
            .value(Value::Tel("12345678900".into()))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("PAGER")
                .build()]))
            .value(Value::Tel("312-555-1515".into()))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("HOME,VOICE")
                .build()
            ]))
            .value(Value::Tel("312-555-1313".into()))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("WORK,VOICE")
                .build()]))
            .value(Value::Tel("312-555-1212".into()))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("HOME,FAX")
                .build()]))
            .value(Value::Tel("312-555-1616".into()))
            .build(),
        Property::builder()
            .params(Parameters(vec![Parameter::builder()
                .name("TYPE")
                .value("WORK,FAX")
                .build()]))
            .value(Value::Tel("312-555-1717".into()))
            .build(),
    ])
}

#[test]
fn vcard_round_trip() {
    match VCard::parse(DATA) {
        Err(nom::Err::Failure(err)) => panic!("\n{}", err.display(DATA)),
        Err(nom::Err::Error(err)) => panic!("\n{}", err.display(DATA)),
        Err(err) => panic!("\n{}", err),
        //Ok(item) => assert_eq!(expected(), item),
        Ok(item) => {
            let expected = expected();
            for (x, i) in item.0.into_iter().enumerate() {
                assert_eq!(expected.0[x], i);
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
