use rolodex::*;

const DATA: &str = "http://www.example.com/dir_photos/my_photo.gif\n";

fn name() -> types::Url<'static> {
    types::Url::builder()
        .schema("http")
        .domain("www.example.com")
        .path("/dir_photos/my_photo.gif")
        .build()
}

#[test]
fn simple() {
    assert_eq!(Ok(("\n", name())), Parse::parse(&format!("{}", DATA)));
    assert_eq!(Ok(("\n", name())), Parse::parse(DATA));
    assert_eq!(Ok(("\n", TypeOrRaw::Type(name()))), TypeOrRaw::<types::Url>::parse(DATA));
}
