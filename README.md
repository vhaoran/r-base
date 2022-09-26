#背景介绍：
  rust语言特点我们勿需多说，优点和缺点都十分明显。  有人曾在网上说『这是一门可以让你到宿醉的语言』事实也确实如此，在很多方面rust都做到了极致。  
  完备的包组织方式，有特色的trait,宏系统，极高的效率，泛型系统，异步框架，函数式编程，optin及result及其极有特色的？语法糖，使rust的代码更加简洁，优雅，甚至连常用的类型，也作到了极简，如:(u8,i8,i64等基础类型)。  
  其它语言中烦人的try/catch块，go中的err调用烦恼，java中对于option的追求，这在rust中是开始就具备的东西。
  rust缺点也很明显，那就是『难』，太『难』了，一个学习能力不错的开发者，可以在几天之内就开始go的开发，可以在几周之内开始java的开发，而对rust，几周只是开发的开始，对于大部分人而言，3个月做到rust真正入门，并达到中级水平，是个不小的挑战。本框架正是基于这样的原因而设计。  
  这是一个基于rust的基础框架。相较于java/go等其它的主流开发语言，rust似乎更『难』一点，而封装一个基础一点的rust框架，可以极大提高开发效率，团队中的开发人员水平各不相同，开发习惯也有差异，一个统一的基础框架更有利于统一团队的工作习惯及代码风格。    
  在一个项目开发中，最繁琐的事情就是你在每一个项目中都需要配置日志，数据库连接，缓存，mq等等项，而这些功能在项目中几乎都大同小异，根据rust中的dry(don't repeat youself)的原则，我们封装了以下内容，适用于开发经验在3年以下的初创团队。    
  本框架封装了以下内容：
  ## 基于日志的封装其于log4j
  ## mongodb客户㾍
  ## redis客户㾍
  ## elasticsearch数据库客户㾍
  ## rabbitMq消息客户㾍
  ## nats消息服务客户㾍
  ## sled 本地key-value嵌入式缓存数据库
  ## 内存缓存实现
  ## 几个数据验证的宏
# 初始化  
  主要设置一下配置文件，即可完成以上列表国连接的封装。直接使用即可。如：
  ```
  #[macro_use]
  extern crate r_base;
  #[tokio::main]
  async fn main() anyhow::Result<()> {
    //default path is "./config.yml"
    r_base::init_modules(None).await?;
    let cfg = cmn::get_bots_config(None)?;
    ...
    //e.g visit redis
    let _ = r_base::rred::set_x("my_key", "value of key", 86400 * 15).await?;
    let s = r_base::rred::get("my_key").await.unwrap_or("".to_string());
    //e.g visit sled
  }
  ```
   下面是一个配置文件的例子：
   ```
  log:
    level: debug
    # 日志路径
    # logPath: "./"
    # 文件名，不含路径
    file_name: log
    path: "./"
    size: 10
    roll_count: 3
  redis:
    host: 192.168.0.201
    port: 6379
    db: 0
    user_name: ""
    password: ""
  mongo:
    url: "mongodb://root:password@192.168.0.201:27017"
    max_idle_time: 1800
    max_pool_size: 10
    min_pool_size: 5
  es:
    url: "http://elastic:password@192.168.0.99:9200"
  #mq:
  #  url: "amqp://root:password@192.168.0.201:5672/%2f"
  #  poolMax: 100
  #  poolMin: 10
  nats:
    host: "192.168.0.201:4222"
    username: "root"
    pwd: "password"
  LevelDb:
    path: "./level.db"
   ```
  如果不需要其中的某些项，如mongodb,则注释掉mongo对应的配置项即可。
# mongodb
  主要实现了数据库连接和dao层的封装。  
  直接按模式名访问：
  ```
    let r: User = find_one(
        "test", // database name
        "user",    //table_name
        doc! {
            "_id":4,
        },
        None,
    )
    .await?;
  ```
  基于dao层的封装如下：  
  假设有一张User表，以下为dao_user.rs的内容：
  ```
  ...
  const TB: &str = "audio_lib";
  r_base::mongo_base!(MONGO_DB, TB, AudioLib);
  ```
  其中，r_base::mongo_base!宏封装了以下内容：
  从其它单元进行访问,直接访问以下内容：
```
  dao_user::find_many();查找多条记录，返回entity
  dao_user::find_one();查找1条记录，返回entity
  dao_user::update_many();修改多条记录，返回entity
  dao_user::update_one();修改1条记录，返回entity
  dao_user::delete_many();删除多条记录，返回entity
  dao_user::delete_one();  删除1条记录，返回entity
  dao_user::insert_many();  插入多条记录，返回entity
  dao_user::insert_one(); 插入1条记录，返回entity
  dao_user::page();分页查询
  dao_user::max();聚合操作，求最大值
  dao_user::min();聚合操作，求最小值
  dao_user::sum();聚合操作，求汇总值
  dao_user::avg();聚合操作，求平均值
  dao_user::count();聚合操作，求行数
  ```
  //
## redis客户端  
  由于redis的服务端特性，提供并发访问无太大意义，故对redis的访问是『单协程』的。  
  以下是常用的api:
```
  pub async fn get_bool<T>(key: T) -> bool
  pub async fn get_i64<T>(key: T, default_value: Option<i64>) -> i64
  pub async fn get_f64<T>(key: T, default_value: Option<f64>) -> f64
  pub async fn get<T>(key: T) ->anyhow::Result<String>
  pub async fn set<K, V>(key: K, v: V) anyhow::Result<()>
  pub async fn set_x<K, V>(
    key: K,
    v: V,
    expire_secs: usize,
  ) -> Result<bool, Box<dyn std::error::Error>>
  pub async fn incr<T>(key: T) -> Result<i64, Box<dyn std::error::Error>>
  pub async fn incr_by<T>(key: T, i: i64) -> Result<i64, Box<dyn std::error::Error>>
  pub async fn expire<T>(key: T, expire_secs: usize) anyhow::Result<()>
  pub async fn del<T>(key: T) anyhow::Result<()>
  pub async fn del_many(keys: Vec<String>) anyhow::Result<()> {
```
  # rnats
  # rsled
  # rmq
  # rlog
  # res

  针对每一个功能的封装都提供了丰富的例程，直接查看对应模块目录下的test.rs文件，即可以很快入手，真正达到开箱即用。


  

  