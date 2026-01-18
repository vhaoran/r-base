
#[test]
fn macro_1() {
    // anyhow!();
    macro_rules! four {
        () => {
            1 + 3
        };
    }

    let a = four!();
    println!(" value is {:?}", a);
}

#[test]
fn macro_2() {
    // $e 元变量
    macro_rules! five {
        ($e:expr) => {
            5 * $e
        };
    }
    macro_rules! five_none {
        ($e:expr) => {
            $e.is_none()
        };
    }

    println!(" {:?}", five!(7));
    println!("some(7) is none?  {:?}", five_none!(Some(7)));
    let c: Option<i32> = None;
    println!("some(7) is none?  {:?}", five_none!(c));
}

#[test]
fn macro_3() {
    macro_rules! add {
        ($a:expr, $b:expr, $c:expr) => {
            $a + $b + $c
        };
    }
    let r = add!(1, 2, 3);

    println!("----------------------");
    println!(" {}", r);
    println!("-----------a-----------");
}

#[test]
fn macro_4() {
    /*
        $ 是字面上的美元符号标记
    ( ... ) 是被反复匹配的模式，由小括号包围。
    sep 是 可选 的分隔标记。它不能是括号或者重复操作符。常用例子包括 , 和 ; 。
    rep 是 必须 的重复操作符。当前可以是：
    ?：表示 最多一次重复，所以不能用于分割标记。
    *：表示 零次或多次重复。
    +：表示 一次或多次重复。
        */
    macro_rules! tt {
        ($a:ident,$b:expr) => {{
            let $a = 3;
            $a + $b
        }};
    }
    println!("-----------a-----------");
    println!(" {}", tt!(a, a / 2));
    println!("-----------b-----------");
}
#[test]
fn macro_5() {
    macro_rules! tt {
        ($b:expr) => {{
            let a = 30;
            a + $b
        }};
    }
    println!("-----------a-----------");
    println!(" {}", tt!(2));
    println!("-----------b-----------");
}

#[test]
fn macro_repeat_1() {
    macro_rules! min {
    // Base case:
    ($x:expr) => ($x);
    // `$x` followed by at least one `$y,`
    ($x:expr, $($y:expr),+) => (
        // Call `find_min!` on the tail `$y`
        std::cmp::min($x, min!($($y),+))
    )
    }
    println!(" {}", min!(1));
    println!(" {}", min!(11, 3));
    println!(" {}", min!(15, 7, 4));
}

#[test]
fn macro_repeat_2() {
    macro_rules! no_none {
        ($x:expr)=>(!($x.is_none()));
        ($x:expr,$($y:expr),+) => {
            if $x.is_none() {
                false
            } else {
                no_none!($($y),+)
            }
        };
    }

    println!(" {}", no_none!(Some(1)));
    let c: Option<bool> = None;
    println!(" {}", no_none!(Some(11), c));
}

#[test]
fn t_is_none() {
    // use ::no_none;
    // use mongodb::bson::doc;

    fn f() -> anyhow::Result<()> {
        let c: Option<i32> = None;
        super::v_no_none!("全部要有值", Some(3), Some(4), c, None as Option<u8>)?;
        Ok(())
    }

    println!("t_is_none: {:?}", f());
}

#[test]
fn t_num() {
    fn f() -> anyhow::Result<String> {
        super::v_num!(">3", ">=", 3, 4, 5, 1, 2)?;
        Ok("OK".to_string())
    }
    fn f1() -> anyhow::Result<String> {
        super::v_num!(">3", ">=", 3, 4, 5, 7, 8)?;
        Ok("OK".to_string())
    }
    println!(">3: {:?}", f());
    println!(">2: {:?}", f1());
}

#[test]
fn t_v_num() {
    fn f() -> anyhow::Result<String> {
        super::v_opt_num!(">3", ">", 3, Some(4), Some(7))?;
        Ok("OK".to_string())
    }
    fn f1() -> anyhow::Result<String> {
        crate::v_opt_num!(">2", ">=", 2, None as Option<i32>, Some(5))?;
        Ok("OK".to_string())
    }
    println!(">3: {:?}", f());
    println!(">2: {:?}", f1());
}

#[test]
fn t_v_len() {
    fn f() -> anyhow::Result<String> {
        super::v_len!("v_len >=3", ">", 3, "aaaa", "bbbb", "ssdsaf".to_string())?;
        Ok("OK".to_string())
    }
    fn f1() -> anyhow::Result<String> {
        super::v_len!("v_len >=3", ">", 3, "aa", "bbbb", "ssdsaf".to_string())?;
        Ok("OK".to_string())
    }

    println!("v_len >=3: {:?}", f());
    println!("v_len >=3 f1: {:?}", f1());
}

#[test]
fn t_v_n_s() {
    //---------------------
    // println!("v_len >=3: {:?}", super::v_n!("value",> 3, 3,4, 5, 7));
}

// #[test]
// fn t_v_n() {
//     println!("------>-----------");
//     println!("v_len >=3: {:?}", super::v_n!(> 3,"value", 2,4, 5, 7));
//
//     println!("------>=-----a-----------");
//     println!(
//         "v_len >=3 f1: {:?}",
//         super::v_n!(> 3,"value must be >=3", 4, 5, 7)
//     );
//     println!("v_n >=3 f1: {:?}", super::v_n!("value",> 3, 4, 5, 7));
//
//     println!("------>= <=-----a-----------");
//     println!(
//         "v_len >=3 f1: {:?}",
//         super::v_n!(@> 3,@< 10, "value must be (>= 3,<= 10) ", 4, 5, 7)
//     );
//     println!(
//         "v_len >=3 f1: {:?}",
//         super::v_n!("value must be (>= 3,<= 10) ",@> 3,@< 10,  4, 5, 7)
//     );
//
//     println!(
//         "v_len >=3 f1: {:?}",
//         super::v_n!(@> 3,@< 10, "value must be (>= 3,<= 10) ", 4, 2, 18, 5, 7)
//     );
//
//     println!("-----------[]-----------");
//     println!(
//         " >= < 4, 3,  5, 7  {:?}",
//         super::v_n!(@>= 3, @< 10, "value must be (>= 3,<= 10) ", 4, 3,  5, 7)
//     );
//     println!(
//         " > <= 4, 3, 10, 5, 7 {:?}",
//         super::v_n!(@> 3, @<= 10 , "value must be (>= 3,<= 10) ", 4, 3, 10, 5, 7)
//     );
//
//     println!(
//         " >= <= 3, 2, 18, 5, 7  {:?}",
//         super::v_n!(@>= 3,@<= 10, "value must be (>= 3,<= 10) ", 3, 2, 18, 5, 7)
//     );
//
//     println!(
//         " >= <= 3, 2, 18, 5, 7  {:?}",
//         super::v_n!(@>= 3,@< 10, "value must be (>= 3,<= 10) ", 3, 2, 18, 5, 7)
//     );
// }

#[test]
fn t_v_in() {
    println!(
        "3 in (4,5,6)---result: {:?}",
        super::v_in!("value must  in (4,3,5,6) ", 3, vec![4, 5, 6])
    );
    println!(
        "3 in (4,3,5,6)---result: {:?}",
        super::v_in!("value must  in (4,3,5,6)", 3, vec![4, 3, 5, 6])
    );
    println!(
        "slice 3 in (4,3,5,6)---result: {:?}",
        super::v_in!("value must  in (4,3,5,6)", 3, [4, 3, 5, 6])
    );
}

#[test]
fn t_opt_in_1() {
    println!(
        "3 in (4,5,6)---result: {:?}",
        super::v_opt_in!("value must  in (4,3,5,6) ", 3, Some(vec![4, 5, 6]))
    );
    println!(
        "3 in (4,3,5,6)---result: {:?}",
        super::v_opt_in!("opt value must  in (4,3,5,6)", 3, Some(vec![4, 3, 5, 6]))
    );
    println!(
        "slice 3 in (4,3,5,6)---result: {:?}",
        super::v_opt_in!("value must  in (4,3,5,6)", 3, Some([4, 3, 5, 6]))
    );
}

#[test]
fn v_n_serial_1() {
    //---------------------
    println!(
        "-----------ok---------{:?}--",
        v_n_gt!(3, "must gt 3", 3, 4, 5, 6, 7)
    );
    println!(
        "-----------ok---------{:?}--",
        v_n_gte!(3, "must gte 3", 3, 4, 5, 6, 7)
    );

    println!(
        "-----------ok---------{:?}--",
        v_n_lt!(3, "must lt 3", 3, 4, 5, 6, 7)
    );
    println!(
        "-----------ok---------{:?}--",
        v_n_lte!(3, "must lte 3", 3, 4, 5, 6, 7)
    );

    println!("-----------end-----------");
}

#[test]
fn v_len_serial_1() {
    //---------------------
    println!(
        "-----------ok---------{:?}--",
        v_len_gt!(3, "must gt 3", "abcde", "defghis")
    );
    println!(
        "-----------ok---------{:?}--",
        v_len_gte!(3, "must gte 3", "abcde", "defghis")
    );

    println!(
        "-----------ok---------{:?}--",
        v_len_lt!(3, "must lt 3", "abcde", "defghis")
    );
    println!(
        "-----------ok---------{:?}--",
        v_len_lte!(3, "must lte 3", "abcde", "defghis")
    );

    println!("-----------end-----------");
}

#[test]
fn v_ope_len_serial_1() {
    //---------------------
    println!(
        "-----------ok---------{:?}--",
        v_opt_len_gt!(3, "opt len must gt 3", Some("abcde"), Some("defghis"))
    );
    println!(
        "-----------ok---------{:?}--",
        v_opt_len_gte!(3, "opt len must gte 3", Some("abcde"), Some("defghis"))
    );

    println!(
        "-----------ok---------{:?}--",
        v_opt_len_lt!(3, "opt len must lt 3", Some("abcde"), Some("defghis"))
    );
    println!(
        "-----------ok---------{:?}--",
        v_opt_len_lte!(3, "opt len must lte 3", Some("abcde"), Some("defghis"))
    );

    println!("-----------end-----------");
}

#[test]
fn v_opt_n_serial_1() {
    //---------------------
    println!(
        "-----------ok---------{:?}--",
        v_opt_n_gt!(
            3,
            "option number must gt 3",
            Some(3),
            Some(4),
            Some(5),
            Some(6)
        )
    );
    println!(
        "-----------ok---------{:?}--",
        v_opt_n_gte!(3, "must gte 3", Some(3), Some(4), Some(5), Some(6))
    );

    println!(
        "-----------ok---------{:?}--",
        v_opt_n_lt!(3, "must lt 3", Some(3), Some(4), Some(5), Some(6))
    );
    println!(
        "-----------ok---------{:?}--",
        v_opt_n_lte!(3, "must lte 3", Some(3), Some(4), Some(5), Some(6))
    );

    println!("-----------end-----------");
}

#[test]
fn opt_true_1() {
    //---------------------
    println!(
        "-----------{:?}-----------",
        v_opt_true!("opt must is true,", Some(false), Some(false))
    );
    println!(
        "-----------{:?}-----------",
        v_false!(" must is false,", false, false)
    );
    println!(
        "-----------{:?}-----------",
        v_opt_false!("opt must is false,", Some(false), Some(true))
    );
    //
}

#[test]
fn range_1() {
    //---------------------
    println!("{:?}", v_n_range!(>= 1, <= 2, "no msg", 5, 7, 10));
    println!("{:?}", v_n_range!(>= 1, < 2, "no msg", 1, 1, 1));
    println!("{:?}", v_n_range!(> 1, <= 2, "no msg", 5, 7, 10));
    println!("{:?}", v_n_range!(> 1, < 2, "no msg", 5, 7, 10));
}

#[test]
fn eq_1() {
    //---------------------
    println!("{:?}", v_n_eq!(1, "1-no msg", 1, 1, 1, 1));
    println!("{:?}", v_n_eq!(1, "2-no msg", 5, 7, 10));

    println!("{:?}", v_n_not_eq!(1, "3-not eq 1", 1, 1, 1));
    println!("{:?}", v_n_not_eq!(1, "4-not eq 1", 3, 4, 5));

    println!(
        "{:?}",
        v_opt_n_eq!(1, "1-no msg", Some(1), Some(1), Some(1), Some(1))
    );
    println!(
        "{:?}",
        v_opt_n_eq!(1, "2-no msg", Some(5), Some(10), Some(8))
    );

    println!(
        "{:?}",
        v_opt_n_not_eq!(1, "3-not eq 1", Some(1), Some(1), Some(1), Some(1))
    );
    println!(
        "{:?}",
        v_opt_n_not_eq!(1, "4-not eq 1", Some(5), Some(10), Some(8))
    );

    println!("-----------ok-----------");
}
