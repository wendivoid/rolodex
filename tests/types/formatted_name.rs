use rolodex::{types::FormattedName, Parse};

const DATA: &str = "Stevenson;John;Philip,Paul;Dr.;Jr.,M.D.,A.C.P.\n";

fn name() -> FormattedName<'static> {
    FormattedName {
        surname: vec!["Stevenson".into()],
        given: vec!["John".into()],
        additional: vec!["Philip".into(), "Paul".into()],
        suffix: vec!["Jr.".into(), "M.D.".into(), "A.C.P.".into()],
        prefix: vec!["Dr.".into()],
    }
}

#[test]
fn simple() {
    assert_eq!(Ok(("\n", name())), Parse::parse(DATA));
}
