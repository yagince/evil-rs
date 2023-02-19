use evil::Omit;

#[derive(Omit)]
#[omit(NewHoge, id)]
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
    assert_eq!(hoge.age, 0);
}

// #[test]
// #[should_panic]
// fn test_enum() {
//     #[derive(Omit)]
//     enum Foo {}
// }
