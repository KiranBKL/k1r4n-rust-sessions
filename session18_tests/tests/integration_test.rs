use session18_tests;

#[test]
fn it_adds_two() {
    assert_eq!(6,session18_tests::add_two(4));

    #[test] ///it wonnt be executed
    fn check() {

    }
}

//when unit test failed it wont be come here