use maller::*;


#[test]
fn fail_call(){
    let mut maller=Maller::<&str,()>::new();
    assert!(maller.call(&"cat").is_none());

}