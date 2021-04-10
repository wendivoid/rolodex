use rolodex::*;
use rolodex::parse::parse_typed_value;

const DATA: &str = "https://testurl\n";
const RAW_DATA: &str = "data:image/gif;base64,R0lGODlhEAAQAMQAAORHHOVSKudfOulrSOp3WOyDZu6QdvCchPGolfO0o/XBs/fNwfjZ0frl3/zy7////wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACH5BAkAABAALAAAAAAQABAAAAVVICSOZGlCQAosJ6mu7fiyZeKqNKToQGDsM8hBADgUXoGAiqhSvp5QAnQKGIgUhwFUYLCVDFCrKUE1lBavAViFIDlTImbKC5Gm2hB0SlBCBMQiB0UjIQA7\n";

fn url() -> types::Image<'static> {
    types::Image::Url(types::Url::builder()
        .schema("https")
        .domain("testurl")
        .build())
}

fn raw_url() -> types::Image<'static> {
    types::Image::Data {
        ty: "image/gif".into(),
        encoding: "base64".into(),
        data: "R0lGODlhEAAQAMQAAORHHOVSKudfOulrSOp3WOyDZu6QdvCchPGolfO0o/XBs/fNwfjZ0frl3/zy7////wAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACH5BAkAABAALAAAAAAQABAAAAVVICSOZGlCQAosJ6mu7fiyZeKqNKToQGDsM8hBADgUXoGAiqhSvp5QAnQKGIgUhwFUYLCVDFCrKUE1lBavAViFIDlTImbKC5Gm2hB0SlBCBMQiB0UjIQA7".into()
    }
}

#[test]
fn simple() {
    assert_eq!(Ok(("\n", url())), Parse::parse(DATA));
    assert_eq!(Ok(("\n", TypeOrRaw::Type(raw_url()))), parse_typed_value(RAW_DATA));
    assert_eq!(Ok(("\n", raw_url())), Parse::parse(RAW_DATA));

}
