///判断option是否为none
//  no_none!("全部要有值", Some(3),Some(4),None as Option<u8>)?;
#[macro_export]
macro_rules! v_no_none {
        ($msg:expr,$x:expr)=>(
            if $x.is_none(){
                Err(anyhow!(concat!("[",$msg,"]不满足要求")))
            }else{
                Ok(())
            }
        );
        ($msg:expr,$x:expr,$($y:expr),+) => {
            if $x.is_none() {
                Err(anyhow!(concat!("[",$msg,"]不满足要求")))
            } else {
                v_no_none!($msg,$($y),+)
            }
        };
}

#[macro_export]
macro_rules! n_cmp {
    ($sign:expr, $n:expr,$x:expr) => {
        match $sign {
            ">" => $x > $n,
            ">=" => $x >= $n,
            "<" => $x < $n,
            "<=" => $x <= $n,
            "=" => $x == $n,
            "==" => $x == $n,
            "!=" => $x != $n,
            _ => $x > $n,
        }
    };
}

///
/// 数字 型的难，判断多个数字 是否 ><= n
/// sign : >/</>=/<=/= 必须是字符串
/// n: 要对比的值
///  v_num!("must >3", ">=", 3, 4, 5, 1, 2)?;
///  v_num!("must >4", "<=", 2 ,4, 5, 1, 2)?;
#[macro_export]
macro_rules! v_num {
    ($msg:expr,$sign:expr, $n:expr,$x:expr) => {{
        let ok = n_cmp!($sign,$n,$x);

        if !ok {
            Err(anyhow!(concat!("[",$msg,$sign,$n,"]不满足要求")))
        } else {
            Ok(true)
        }
    }};
    ($msg:expr ,$sign:expr, $n:expr,$x:expr,$($y:expr),+) => {{
        let ok = v_num!($msg ,$sign, $n,$x).unwrap_or(false);

        if !ok {
            Err(anyhow!(concat!("[",$msg,$sign,$n,"]不满足要求")))
        } else {
            v_num!($msg ,$sign, $n,$($y),+)
        }
    }};
}

#[macro_export]
macro_rules! v_opt_num {
    ($msg:expr ,$sign:expr, $n:expr,$x:expr) => {{
        let mut ok = false;
        if let Some(a) = $x.clone(){
             ok = n_cmp!($sign,$n,a);
        }

        if !ok{
            Err(anyhow!(concat!("[",$msg,"]不满足要求")))
        }else{
            Ok(true)
        }
    }};
    ($msg:expr ,$sign:expr, $n:expr,$x:expr,$($y:expr),+) => {{
        let ok = v_opt_num!($msg ,$sign, $n,$x).unwrap_or(false);

        if !ok {
            Err(anyhow!(concat!("[",$msg,"]不满足要求")))
        }else{
            v_opt_num!($msg ,$sign, $n,$($y),+)
        }
    }};
}

#[macro_export]
macro_rules! v_len {
    ($msg:expr ,$sign:expr, $n:expr,$x:expr) => {{
        let ok = n_cmp!($sign,$n,$x.len());

        if !ok {
            Err(anyhow!(concat!("[",$msg,"]不满足要求")))
        } else {
            Ok(true)
        }
    }};
    ($msg:expr,$sign:expr, $n:expr,$x:expr,$($y:expr),+) => {{
        let ok = v_len!($msg ,$sign, $n,$x).unwrap_or(false);

        if !ok {
            Err(anyhow!(concat!("[",$msg,"]不满足要求")))
        } else {
            v_len!($msg ,$sign, $n,$($y),+)
        }
    }};
}

#[macro_export]
macro_rules! v_opt_len {
    ($msg:expr ,$sign:expr, $n:expr,$x:expr) => {{
        let mut ok = false;
        if let Some(x) = $x.clone(){
            ok = n_cmp!($sign,$n,x.len());
        };

        if !ok {
            Err(anyhow!(concat!("[",$msg,"]不满足要求")))
        } else {
            Ok(true)
        }
    }};
    ($msg:expr,$sign:expr, $n:expr,$x:expr,$($y:expr),+) => {{
        let ok = v_opt_len!($msg ,$sign, $n,$x).unwrap_or(false);

        if !ok {
            Err(anyhow!(concat!("[",$msg,"]不满足要求")))
        } else {
            v_opt_len!($msg ,$sign, $n,$($y),+)
        }
    }};
}

//-----------v_true--------------------------
/// v_true!("",falase,true);
#[macro_export]
macro_rules! v_true {
    ($msg:expr, $v:expr) => {{
        if !$v {
            Err(anyhow!(concat!("[", $msg, "必须为真值]不满足要求")))
        }else{
            Ok(true)
        }
    }};
    ($msg:expr,$v:expr, $($y:expr),+) => {{
        if !$v {
            Err(anyhow!(concat!("[", $msg, "必须为真值]不满足要求")))
        }else{
            v_true!($msg,$($y),+)
        }
    }};
}

/*
v_opt_true!("opt must is true,", Some(false), Some(false))
*/
#[macro_export]
macro_rules! v_opt_true {
    ($msg:expr, $v:expr) => {{
        let ok = $v.clone().unwrap_or(false);
        if !ok {
            Err(anyhow!(concat!("[", $msg, "必须为真值]不满足要求")))
        }else{
            Ok(true)
        }
    }};
    ($msg:expr,$v:expr, $($y:expr),+) => {{
        let ok = $v.clone().unwrap_or(false);
        if !ok {
            Err(anyhow!(concat!("[", $msg, "必须为真值]不满足要求")))
        }else{
            v_opt_true!($msg,$($y),+)
        }
    }};
}

//-----------v_false_X--------------------------
/*
v_false!(" must is false,", false, false)
*/
#[macro_export]
macro_rules! v_false {
    ($msg:expr, $v:expr) => {{
        if $v {
            Err(anyhow!(concat!("[", $msg, "必须为假值]不满足要求")))
        }else{
            Ok(true)
        }
    }};
    ($msg:expr,$v:expr, $($y:expr),+) => {{
        if $v {
            Err(anyhow!(concat!("[", $msg, "必须为假值]不满足要求")))
        }else{
            v_true!($msg,$($y),+)
        }
    }};
}

///  v_opt_false!("opt must is false,", Some(false), Some(true))
#[macro_export]
macro_rules! v_opt_false{
    ($msg:expr, $v:expr) => {{
        let ok = $v.clone().unwrap_or(true);
        if ok {
            Err(anyhow!(concat!("[", $msg, "必须为假值]不满足要求")))
        }else{
            Ok(true)
        }
    }};
    ($msg:expr,$v:expr, $($y:expr),+) => {{
        let ok = $v.clone().unwrap_or(true);
        if ok {
            Err(anyhow!(concat!("[", $msg, "必须为假值]不满足要求")))
        }else{
            v_opt_true!($msg,$($y),+)
        }
    }};
}

/// v_in!("value must in ... ", 3, vec![4, 5, 6])
#[macro_export]
macro_rules! v_in {
    ($msg:expr, $v:expr,$vec:expr) => {{
        $vec.iter().find(|&&x| x == $v.clone()).map_or_else(
            || Err(anyhow!(concat!("[", $msg, "]不满足要求"))),
            |_x| Ok(true),
        )
    }};
}
/// v_opt_in!("value must  in (4,3,5,6) ", 3, Some(vec![4, 5, 6]))
#[macro_export]
macro_rules! v_opt_in {
    ($msg:expr, $v:expr,$vec:expr) => {{
        if $vec.is_none() {
            Err(anyhow!(concat!("[", $msg, "]不满足要求")))
        } else {
            let l = $vec.clone().unwrap();
            l.iter().find(|&&x| x == $v.clone()).map_or_else(
                || Err(anyhow!(concat!("[", $msg, "]不满足要求"))),
                |_x| Ok(true),
            )
        }
    }};
}

/// v_not_in!("",3,vec![1,2,3]);
#[macro_export]
macro_rules! v_not_in {
    ($msg:expr, $v:expr,$vec:expr) => {{
        !v_in($msg, $v, $vec)
    }};
}

//-----------n_gt_X--------------------------
/// v_n_gt!(3, "must gt 3", 3, 4, 5, )
#[macro_export]
macro_rules! v_n_gt {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        // !v_n($msg, ">", $n,$($y:expr),+)
        v_num!($msg,">",$n,$($y),+)
    }};
}

/// v_n_gte!(3, "must gte 3", 3, 4, 5, 6, 7)
#[macro_export]
macro_rules! v_n_gte {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_num!($msg,">=",$n,$($y),+)
    }};
}

/// v_n_eq!(3, "must lt 3", 3, 4, 5, 6, 7)
#[macro_export]
macro_rules! v_n_eq {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_num!($msg,"==",$n,$($y),+)
    }};
}

/// v_n_not_eq!(3, "must lt 3", 3, 4, 5, 6, 7)
#[macro_export]
macro_rules! v_n_not_eq {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_num!($msg,"!=",$n,$($y),+)
    }};
}

/// v_n_lt!(3, "must lt 3", 3, 4, 5, 6, 7)
#[macro_export]
macro_rules! v_n_lt {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_num!($msg,"<",$n,$($y),+)
    }};
}

/// v_n_lte!(3, "must lte 3", 3, 4, 5, 6, 7)
#[macro_export]
macro_rules! v_n_lte {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_num!($msg,"<=",$n,$($y),+)
    }};
}

//-----------opt_num_X--------------------------
/// v_opt_n_gt!(3,
///             "option number must gt 3",
///             Some(3),
///             Some(6)
///         )
#[macro_export]
macro_rules! v_opt_n_gt {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        // !v_n($msg, ">", $n,$($y:expr),+)
        v_opt_num!($msg,">",$n,$($y),+)
    }};
}

/// v_opt_n_eq!(3, "must gte 3", Some(3), Some(4), Some(5), Some(6))
#[macro_export]
macro_rules! v_opt_n_eq {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_opt_num!($msg,"==",$n,$($y),+)
    }};
}
/// v_opt_n_not_eq!(3, "must gte 3", Some(3), Some(4), Some(5), Some(6))
#[macro_export]
macro_rules! v_opt_n_not_eq {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_opt_num!($msg,"!=",$n,$($y),+)
    }};
}

/// v_opt_n_gte!(3, "must gte 3", Some(3), Some(4), Some(5), Some(6))
#[macro_export]
macro_rules! v_opt_n_gte {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_opt_num!($msg,">=",$n,$($y),+)
    }};
}

/// v_opt_n_lt!(3, "must lt 3", Some(3), Some(4), Some(5), Some(6))
#[macro_export]
macro_rules! v_opt_n_lt {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_opt_num!($msg,"<",$n,$($y),+)
    }};
}

/// v_opt_n_lte!(3, "must lte 3", Some(3), Some(4), Some(5), Some(6))
#[macro_export]
macro_rules! v_opt_n_lte {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_opt_num!($msg,"<=",$n,$($y),+)
    }};
}

//--v_len-----------------------------------
/// v_len_gt!(3, "must gt 3", "abcde", "defghis")
#[macro_export]
macro_rules! v_len_gt {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        // !v_n($msg, ">", $n,$($y:expr),+)
        v_len!($msg,">",$n,$($y),+)
    }};
}

/// v_len_gte!(3, "must gte 3", "abcde", "defghis")
#[macro_export]
macro_rules! v_len_gte {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_len!($msg,">=",$n,$($y),+)
    }};
}

/// v_len_lt!(3, "must lt 3", "abcde", "defghis")
#[macro_export]
macro_rules! v_len_lt {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_len!($msg,"<",$n,$($y),+)
    }};
}

/// v_len_lte!(3, "must lte 3", "abcde", "defghis")
#[macro_export]
macro_rules! v_len_lte {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_len!($msg,"<=",$n,$($y),+)
    }};
}
//-----------opt_len_X--------------------------
/// r_base::v_opt_len_gt!(0, "token", &r.token)
#[macro_export]
macro_rules! v_opt_len_gt {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        // !v_n($msg, ">", $n,$($y:expr),+)
        v_opt_len!($msg,">",$n,$($y),+)
    }};
}

/// v_opt_len_gte!(3, "opt len must gte 3", Some("abcde"), Some("defghis"))
#[macro_export]
macro_rules! v_opt_len_gte {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_opt_len!($msg,">=",$n,$($y),+)
    }};
}

/// v_opt_len_lt!(3, "opt len must lt 3", Some("abcde"), Some("defghis"))
#[macro_export]
macro_rules! v_opt_len_lt {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_opt_len!($msg,"<",$n,$($y),+)
    }};
}

/// v_opt_len_lte!(3, "opt len must lte 3", Some("abcde"), Some("defghis"))
#[macro_export]
macro_rules! v_opt_len_lte {
    ($n:expr,$msg:expr, $($y:expr),+) => {{
        v_opt_len!($msg,"<=",$n,$($y),+)
    }};
}

/// v_n_range!(>= 1, <= 2, "no msg", 5, 7, 10)
#[macro_export]
macro_rules! v_n_range {
    ( >= $start:expr,<= $end:expr,$msg:expr, $($y:expr),+) => {{
        let r = v_num!($msg,">=",$start,$($y),+);
        if r.is_err(){
            r
        }else{
            v_num!($msg,"<=",$end,$($y),+)
        }
    }};
    (>= $start:expr,< $end:expr,$msg:expr, $($y:expr),+) => {{
        let r = v_num!($msg,">=",$start,$($y),+);
        if r.is_err(){
            r
        }else{
            v_num!($msg,"<",$end,$($y),+)
        }
    }};
    (> $start:expr,<= $end:expr,$msg:expr, $($y:expr),+) => {{
        let r = v_num!($msg,">",$start,$($y),+);
        if r.is_err(){
            r
        }else{
            v_num!($msg,"<=",$end,$($y),+)
        }
    }};
    (> $start:expr,< $end:expr,$msg:expr, $($y:expr),+) => {{
        let r = v_num!($msg,">",$start,$($y),+);
        if r.is_err(){
            r
        }else{
            v_num!($msg,"<",$end,$($y),+)
        }
    }};
}
