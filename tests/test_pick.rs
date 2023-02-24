use evil::Pick;

#[derive(Pick, Debug)]
#[pick(NewHoge, id, derive(Debug, Clone))]
struct Hoge {
    pub id: u64,
    pub age: u64,
}

#[test]
fn test_pick_hoge() {
    let hoge = Hoge { id: 1000, age: 0 };
    assert_eq!(hoge.id, 1000);
    assert_eq!(hoge.age, 0);
}

#[test]
fn test_pick_new_hoge() {
    let hoge = NewHoge { id: 1000 };
    assert_eq!(hoge.id, 1000);
}
