use assert_matches::assert_matches;
use evil::Omit;

#[derive(Omit, Debug)]
#[omit(NewHoge, id, derive(Debug, Clone))]
#[omit(OldHoge, age)]
struct Hoge {
    pub id: u64,
    pub age: u64,
}

#[test]
fn test_hoge() {
    let hoge = Hoge { id: 1000, age: 0 };
    assert_eq!(hoge.id, 1000);
    assert_eq!(hoge.age, 0);
}

#[test]
fn test_new_hoge() {
    let hoge = NewHoge { age: 0 };
    dbg!(&hoge);
    assert_eq!(hoge.clone().age, 0);
}

#[test]
fn test_validator() {
    use validator::Validate;

    #[derive(Omit, Debug, Validate)]
    #[omit(NewData, id, derive(Debug, Validate))]
    struct Data {
        #[validate(range(min = 1))]
        pub id: u64,
        #[validate(range(min = 18, max = 20))]
        pub age: u32,
    }

    let data = Data { id: 1, age: 1 };
    assert_matches!(data.validate(), Err(_));

    let data = NewData { age: 1 };
    assert_matches!(data.validate(), Err(_));
}
// #[test]
// #[should_panic]
// fn test_enum() {
//     #[derive(Omit)]
//     enum Foo {}
// }
