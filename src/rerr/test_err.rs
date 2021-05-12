// use crate::RErr;

#[test]
fn err_1() {
    //---------------------
    let e = crate::RErr::new("abc");
    println!("----test_err_1.rs---a--{:#?}---", e);
}

#[test]
fn err_2() {
    //---------------------
    fn f() -> Result<(), Box<dyn std::error::Error>> {
        let _a = f1()?;
        Ok(())
    }

    fn f1() -> Result<(), crate::RErr> {
        Err(crate::RErr::new("abc"))
    }

    let a = f();
    println!("----test_err_1.rs-----{:?}---", a);
}

#[test]
fn err_3() {
    let a = std::fs::read_to_string("/a.txt".to_string());
    if let Err(e) = a {
        println!("----err---{}--", crate::RErr::tran(e));
    }
    println!("---------");
}
