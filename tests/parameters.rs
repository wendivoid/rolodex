use rolodex::*;

#[test]
fn simple() {
    assert_eq!(
        ("\n", Property::builder().value(Value::Begin).build()),
        Parse::parse("BEGIN:VCARD\n").unwrap()
    );
    assert_eq!(
        ("\n", Property::builder().value(Value::End).build()),
        Parse::parse("END:VCARD\n").unwrap()
    );
    assert_eq!(
        (
            "\n",
            Property::builder()
                .params(Parameters(vec![]))
                .value(Value::Version("4.0".into()))
                .build()
        ),
        Parse::parse("VERSION:4.0\n").unwrap()
    );

    assert_eq!(
        (
            ":VALUE\n",
            vec![
                Parameter::builder()
                    .name("MEDIATYPE")
                    .value("image/gif")
                    .build(),
                Parameter::builder().name("OTHER").value("value").build(),
            ]
        ),
        property::parse_parameters(";MEDIATYPE=image/gif;OTHER=value:VALUE\n").unwrap()
    );
}
