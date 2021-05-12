use async_trait::async_trait;
use deadpool::managed::{Object, Pool, RecycleResult};
use std::time::Duration;
// use tokio::time::sleep;
use std::net::TcpStream;
use std::sync::{Arc, Mutex as Mu};

struct Computer {}
struct Manager {}
type RedisPool = deadpool::managed::Pool<Computer, Box<dyn std::error::Error>>;

//
impl Computer {
    async fn get_answer(&self) -> i32 {
        42
    }
}

//
#[async_trait]
impl deadpool::managed::Manager<Computer, Box<dyn std::error::Error>> for Manager {
    async fn create(&self) -> Result<Computer, Box<dyn std::error::Error>> {
        println!("----computer-----created....");
        Ok(Computer {})
    }
    async fn recycle(
        &self,
        _conn: &mut Computer,
    ) -> deadpool::managed::RecycleResult<Box<dyn std::error::Error>> {
        println!("----computer-----recycled....");
        Ok(())
    }
}

#[tokio::test]
async fn dp_1() {
    // use self::red::*;
    let mgr = Manager {};
    let pool = RedisPool::new(mgr, 16);
    let conn = pool.get().await.unwrap();
    for _i in 0..10u16 {
        let answer = conn.get_answer().await;
        println!("----test_2.rs---a----{:?}-", pool.status());
        assert_eq!(answer, 42);
        println!("-----------------------");
    }
}

#[tokio::test]
async fn dp_2() {
    use std::sync::{Arc, Mutex as Mu};

    let mgr = Manager {};
    let pool = RedisPool::new(mgr, 16);
    let a = Arc::new(Mu::new(pool));

    for _i in 0..1000u32 {
        let a = a.clone();
        let pool = a.lock().unwrap();

        let conn = pool.get().await.unwrap();
        let answer = conn.get_answer().await;
        println!("----test_2.rs---a----{:?}-", pool.status());
        assert_eq!(answer, 42);
    }
}

#[tokio::test]
async fn deadpool_3() {
    use std::sync::Arc;
    use tokio::sync::Mutex as Mu;

    let mgr = Manager {};
    let pool = RedisPool::new(mgr, 3);
    let a = Arc::new(Mu::new(pool));

    for i in 0..62u16 {
        let a = a.clone();
        let id = i;
        // println!("----doing --no: {}--", id);
        tokio::spawn(async move {
            println!("-- {}   ---", id);
            let pool = a.lock().await;

            let conn = pool.get().await.unwrap();
            let answer = conn.get_answer().await;
            println!("---{}----{:?}-", id, pool.status());
            assert_eq!(answer, 42);
            if id > 70 {
                println!("----test_2.rs---abouve 70-----");
            }
            // tokio::task::yield_now().await;
        });
    }

    println!("----waiting-----");
    // let c = a.lock().await;

    tokio::task::yield_now().await;

    // std::thread::sleep(std::time::Duration::from_secs(20));
}

#[tokio::test]
async fn dp_4() {
    use std::sync::Arc;
    use tokio::sync::Mutex as Mu;

    let mgr = Manager {};
    let pool = RedisPool::new(mgr, 10);
    // let a = Arc::new(Mu::new(pool));

    for i in 0..20u16 {
        let a = pool.clone();
        let id = i;
        // println!("----doing --no: {}--", id);
        tokio::spawn(async move {
            println!("-- {}   ---", id);
            let pool = a;

            let conn = pool.get().await.unwrap();
            let answer = conn.get_answer().await;
            println!("---{}--{:?}--{:?}", id, pool.status(), answer);
            assert_eq!(answer, 42);
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            tokio::task::yield_now().await;
        });
        println!("----after await-{}----", id);
    }

    //this is wrong .....
    println!("----waiting-----");
    tokio::task::yield_now().await;
    std::thread::sleep(std::time::Duration::from_secs(2));
}

#[tokio::test]
async fn dp_5() {
    use std::sync::Arc;
    use tokio::sync::Mutex as Mu;
    let mgr = Manager {};
    let pool = RedisPool::new(mgr, 10);

    // Spawn tasks
    let _futures = (0..10)
        .map(|id| {
            println!("----id {}---", id);
            let pool = pool.clone();
            tokio::spawn(async move {
                println!("---inner-id {}---", id);
                let obj = pool.get().await.unwrap();
                let _a = obj.get_answer().await;
                tokio::time::sleep(Duration::from_millis(1)).await;
                println!("--{}  {:?}-----", id, pool.status());
            })
        })
        .collect::<Vec<_>>();

    let futures2 = (0..100)
        .map(|id| {
            println!("--###########--id {}---", id);
            let pool = pool.clone();
            tokio::spawn(async move {
                println!("--###########-inner-id {}---", id);
                let obj = pool.get().await.unwrap();
                let _a = obj.get_answer().await;
                tokio::time::sleep(Duration::from_millis(1)).await;
                println!("-###########-{}  {:?}-----", id, pool.status());
            })
        })
        .collect::<Vec<_>>();

    // Await tasks to finish
    let mut ii = 0;
    for future in futures2 {
        ii += 1;
        println!("--ii:--{}-----", ii);
        future.await.unwrap();
    }

    println!("----last line-----");
    //-----------a--------------------------
}

#[tokio::test]
async fn tk_5() {
    async fn hello() {
        println!("----hello- world--");
    }

    tokio::spawn(async move {
        hello().await;
        tokio::spawn(async move {
            println!("----inner---a-----");
        });
        for i in 0..1000u32 {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            tokio::spawn(async move {
                println!("----inner---a---{}", i);
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            });
        }
    });

    tokio::spawn(async move {
        hello().await;
    });

    println!("---------");

    tokio::spawn(async move {
        println!("----after sleep-----");
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            println!("----after sleep-----");
        }
    })
    .await;
}
