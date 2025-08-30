use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::sync::Mutex;

use super::module_init::*;

//
type IType = ModuleConfig;

fn instance() -> &'static Arc<Mutex<IType>> {
    static INSTANCE: OnceCell<Arc<Mutex<IType>>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        let m = IType::default();
        Arc::new(Mutex::new(m))
    })
}

pub async fn get_base_cfg() -> IType {
    let a = self::instance().clone();
    let m = a.lock().await;
    m.clone()
}

pub(crate) async fn set(cfg: ModuleConfig) {
    let a = self::instance().clone();
    let mut m = a.lock().await;
    *m = cfg;
}

mod t{
    #[test]
    fn sss() {
        //---------------------
        println!("-----------hello world-----------", );
    }

}