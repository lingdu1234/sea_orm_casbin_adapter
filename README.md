# sea orm casbin adapter

## Introduction
> sea-orm-casbin-adapter is a casbin adapter for sea-orm.  
> just like sqx-adapter, it can be used in sea-orm.


# ...
because of the sqx casbin adapter is not work correctly,  
so  write a adapter with sea-orm. it is only tested on Mysql,Postgresql. not Sqlite3,may be work.

## usage<使用方法>
1.  add dependencies
```
[dependencies]
sea_orm_casbin_adapter = "*"
tokio = "*"
```
2.  add casbin model policy file
3.  add code to your code
   ```rust
   use sea_orm_casbin_adapter::{casbin::prelude::*, SeaOrmAdapter};
   
#[tokio::main]
async fn main() -> Result<()> {
    let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
        .await
        .unwrap();

    // mysql://root:youpassword@127.0.0.1:13306/wk
    // postgres://postgres:youpassword@127.0.0.1:25432/wk
    let db = SeaOrmAdapter::new("postgres://postgres:youpassword@127.0.0.1:25432/wk")
        .await
        .expect("open db error");

    let mut e = Enforcer::new(m, db).await?;
    e.enable_log(true);
    e.add_named_policy(
        "g",
        vec![
            "data9_admin".to_string(),
            "data9".to_string(),
            "write".to_string(),
        ],
    )
    .await
    .unwrap();

    e.add_policy(
        vec![
            "dataxx_admin".to_string(),
            "dataxx".to_string(),
            "write".to_string(),
        ],
    )
    .await
    .unwrap();

    e.add_policies(vec![
        vec![
            "data1_admin".to_string(),
            "data4".to_string(),
            "write".to_string(),
        ],
        vec![
            "data3_admin".to_string(),
            "data5".to_string(),
            "write".to_string(),
        ],
    ])
    .await
    .unwrap();

    // e.remove_policies(vec![
    //     vec![
    //         "data6_admin".to_string(),
    //         "data4".to_string(),
    //         "write".to_string(),
    //     ],
    //     vec![
    //         "data6_admin".to_string(),
    //         "data5".to_string(),
    //         "write".to_string(),
    //     ],
    // ])
    // .await
    // .unwrap();
    println!("C===============");
    e.enforce(("data2_admin", "data2", "write"))?;
    println!("D===============");

    println!("e===============");

    Ok(())
}

   ```

   支持直接传入数据库连接
   ```rust
   let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
        .await
        .unwrap();
    let pool: DatabaseConnection = Database::connect("protocol://username:password@host/database").await?;
    let adpt = SeaOrmAdapter::new_with_pool(pool).await.unwrap();
    let mut e = Enforcer::new(m, adpt).await?;
    e.enforce(("data2_admin", "data2", "write"))?;

   ```

