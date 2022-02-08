use super::models::{ActiveModel, Column, Entity, Model, NewCasbinRule};
use super::Error;
use casbin::{error::AdapterError, Error as CasbinError, Filter, Result};
use sea_orm::sea_query::{self, ColumnDef, Condition};
use sea_orm::ActiveValue::NotSet;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, QueryFilter,
    Set, TransactionTrait,
};
pub async fn new(conn: &DatabaseConnection, is_init: bool) -> bool {
    match is_init {
        true => {
            let stmt = sea_query::Table::create()
                .table(Entity)
                .if_not_exists()
                .col(
                    ColumnDef::new(Column::Id)
                        .integer()
                        .not_null()
                        .primary_key()
                        .auto_increment(),
                )
                .col(ColumnDef::new(Column::Ptype).string())
                .col(ColumnDef::new(Column::V0).string())
                .col(ColumnDef::new(Column::V1).string())
                .col(ColumnDef::new(Column::V2).string())
                .col(ColumnDef::new(Column::V3).string())
                .col(ColumnDef::new(Column::V4).string())
                .col(ColumnDef::new(Column::V5).string())
                .to_owned();
            // 创建表格
            let builder = conn.get_database_backend();
            conn.execute(builder.build(&stmt))
                .await
                .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))
                .expect("casbin table create failed");
            true
        }
        false => true,
    }
}

///  删除策略
pub async fn remove_policy(conn: &DatabaseConnection, pt: &str, rule: Vec<String>) -> Result<bool> {
    let rule = normalize_casbin_rule(rule, 0);
    Entity::delete_many()
        .filter(Column::Ptype.eq(pt))
        .filter(Column::V0.eq((&rule[0]).clone()))
        .filter(Column::V1.eq((&rule[1]).clone()))
        .filter(Column::V2.eq((&rule[2]).clone()))
        .filter(Column::V3.eq((&rule[3]).clone()))
        .filter(Column::V4.eq((&rule[4]).clone()))
        .filter(Column::V5.eq((&rule[5]).clone()))
        .exec(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;

    Ok(true)
}

///  删除多个策略
pub async fn remove_policies(
    conn: &DatabaseConnection,
    pt: &str,
    rules: Vec<Vec<String>>,
) -> Result<bool> {
    for rule in rules {
        let rule = normalize_casbin_rule(rule, 0);
        // let d =
        Entity::delete_many()
            .filter(Column::Ptype.eq(pt))
            .filter(Column::V0.eq((&rule[0]).clone()))
            .filter(Column::V1.eq((&rule[1]).clone()))
            .filter(Column::V2.eq((&rule[2]).clone()))
            .filter(Column::V3.eq((&rule[3]).clone()))
            .filter(Column::V4.eq((&rule[4]).clone()))
            .filter(Column::V5.eq((&rule[5]).clone()))
            .exec(conn)
            .await
            .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    }

    Ok(true)
}

///  删除筛选的策略
pub async fn remove_filtered_policy(
    conn: &DatabaseConnection,
    pt: &str,
    field_index: usize,
    field_values: Vec<String>,
) -> Result<bool> {
    let field_values_x = normalize_casbin_rule(field_values, field_index);
    let mut d = Entity::delete_many().filter(Column::Ptype.eq(pt));
    if field_index == 5 {
        if !field_values_x[0].is_empty() {
            d = d.filter(Column::V5.eq(field_values_x[0].clone()));
        }
    } else if field_index == 4 {
        if !field_values_x[0].is_empty() {
            d = d.filter(Column::V4.eq(field_values_x[0].clone()));
        }
        if !field_values_x[1].is_empty() {
            d = d.filter(Column::V5.eq(field_values_x[1].clone()));
        }
    } else if field_index == 3 {
        if !field_values_x[0].is_empty() {
            d = d.filter(Column::V3.eq(field_values_x[0].clone()))
        }
        if !field_values_x[1].is_empty() {
            d = d.filter(Column::V4.eq(field_values_x[1].clone()));
        }
        if !field_values_x[2].is_empty() {
            d = d.filter(Column::V5.eq(field_values_x[2].clone()))
        }
    } else if field_index == 2 {
        if !field_values_x[0].is_empty() {
            d = d.filter(Column::V2.eq(field_values_x[0].clone()));
        }
        if !field_values_x[1].is_empty() {
            d = d.filter(Column::V3.eq(field_values_x[1].clone()));
        }
        if !field_values_x[2].is_empty() {
            d = d.filter(Column::V4.eq(field_values_x[2].clone()));
        }
        if !field_values_x[3].is_empty() {
            d = d.filter(Column::V5.eq(field_values_x[3].clone()));
        }
    } else if field_index == 1 {
        if !field_values_x[0].is_empty() {
            d = d.filter(Column::V1.eq(field_values_x[0].clone()));
        }
        if !field_values_x[1].is_empty() {
            d = d.filter(Column::V2.eq(field_values_x[1].clone()));
        }
        if !field_values_x[2].is_empty() {
            d = d.filter(Column::V3.eq(field_values_x[2].clone()));
        }
        if !field_values_x[3].is_empty() {
            d = d.filter(Column::V4.eq(field_values_x[3].clone()));
        }
        if !field_values_x[4].is_empty() {
            d = d.filter(Column::V5.eq(field_values_x[4].clone()));
        }
    } else {
        if !field_values_x[0].is_empty() {
            d = d.filter(Column::V0.eq(field_values_x[0].clone()));
        }
        if !field_values_x[1].is_empty() {
            d = d.filter(Column::V1.eq(field_values_x[1].clone()));
        }
        if !field_values_x[2].is_empty() {
            d = d.filter(Column::V2.eq(field_values_x[2].clone()));
        }
        if !field_values_x[3].is_empty() {
            d = d.filter(Column::V3.eq(field_values_x[3].clone()));
        }
        if !field_values_x[4].is_empty() {
            d = d.filter(Column::V4.eq(field_values_x[4].clone()));
        }
        if !field_values_x[5].is_empty() {
            d = d.filter(Column::V5.eq(field_values_x[5].clone()));
        }
    };
    //  执行删除
    d.exec(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;

    Ok(true)
}

//  加载策略
pub async fn load_policy(conn: &DatabaseConnection) -> Result<Vec<Model>> {
    let casbin_rule = Entity::find()
        .all(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(casbin_rule)
}

pub(crate) async fn load_filtered_policy<'a>(
    conn: &DatabaseConnection,
    filter_x: &Filter<'_>,
) -> Result<Vec<Model>> {
    let (g_filter, p_filter) = filtered_where_values(filter_x);
    let casbin_rule = Entity::find()
        .filter(
            Condition::any()
                .add(
                    Condition::all()
                        .add(Column::Ptype.like("p%"))
                        .add(Column::V0.contains(g_filter[0]))
                        .add(Column::V1.contains(g_filter[1]))
                        .add(Column::V2.contains(g_filter[2]))
                        .add(Column::V3.contains(g_filter[3]))
                        .add(Column::V4.contains(g_filter[4]))
                        .add(Column::V5.contains(g_filter[5])),
                )
                .add(
                    Condition::all()
                        .add(Column::Ptype.like("g%"))
                        .add(Column::V0.contains(p_filter[0]))
                        .add(Column::V1.contains(p_filter[1]))
                        .add(Column::V2.contains(p_filter[2]))
                        .add(Column::V3.contains(p_filter[3]))
                        .add(Column::V4.contains(p_filter[4]))
                        .add(Column::V5.contains(p_filter[5])),
                ),
        )
        .all(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(casbin_rule)
}

///  保存策略
pub async fn save_policy(conn: &DatabaseConnection, rules: Vec<NewCasbinRule<'_>>) -> Result<()> {
    let txn = conn
        .begin()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    for rule in rules {
        let v1 = rule.v1.unwrap_or("");
        let v2 = rule.v2.unwrap_or("");
        let v3 = rule.v3.unwrap_or("");
        let v4 = rule.v4.unwrap_or("");
        let v5 = rule.v5.unwrap_or("");
        let s = Entity::find()
            .filter(Column::Ptype.eq(rule.ptype))
            .filter(Column::V0.eq(rule.v0))
            .filter(Column::V1.eq(v1))
            .filter(Column::V2.eq(v2))
            .filter(Column::V3.eq(v3))
            .filter(Column::V4.eq(v4))
            .filter(Column::V5.eq(v5))
            .all(&txn)
            .await
            .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;

        if !s.is_empty() {
            continue;
        }

        // let uid = scru128::scru128();
        let p = ActiveModel {
            id: NotSet,
            ptype: Set(rule.ptype.to_string()),
            v0: Set(rule.v0.to_string()),
            v1: Set(rule.v1.unwrap_or("").to_string()),
            v2: Set(rule.v2.unwrap_or("").to_string()),
            v3: Set(rule.v3.unwrap_or("").to_string()),
            v4: Set(rule.v4.unwrap_or("").to_string()),
            v5: Set(rule.v5.unwrap_or("").to_string()),
        };

        p.insert(&txn)
            .await
            .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    }

    txn.commit()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(())
}

///  添加策略
pub(crate) async fn add_policy(conn: &DatabaseConnection, rule: NewCasbinRule<'_>) -> Result<bool> {
    let v1 = rule.v1.unwrap_or("");
    let v2 = rule.v2.unwrap_or("");
    let v3 = rule.v3.unwrap_or("");
    let v4 = rule.v4.unwrap_or("");
    let v5 = rule.v5.unwrap_or("");
    // 先查询
    let s = Entity::find()
        .filter(Column::Ptype.eq(rule.ptype))
        .filter(Column::V0.eq(rule.v0))
        .filter(Column::V1.eq(v1))
        .filter(Column::V2.eq(v2))
        .filter(Column::V3.eq(v3))
        .filter(Column::V4.eq(v4))
        .filter(Column::V5.eq(v5))
        .all(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    //  确认下是否存在  在添加
    if !s.is_empty() {
        return Ok(false);
    }
    // let uid = scru128::scru128();
    let p = ActiveModel {
        id: NotSet,
        ptype: Set(rule.ptype.to_string()),
        v0: Set(rule.v0.to_string()),
        v1: Set(v1.to_string()),
        v2: Set(v2.to_string()),
        v3: Set(v3.to_string()),
        v4: Set(v4.to_string()),
        v5: Set(v5.to_string()),
    };

    p.insert(conn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(true)
}

///  添加多个策略
pub(crate) async fn add_policies(
    conn: &DatabaseConnection,
    rules: Vec<NewCasbinRule<'_>>,
) -> Result<bool> {
    let txn = conn
        .begin()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    for rule in rules {
        let v1 = rule.v1.unwrap_or("");
        let v2 = rule.v2.unwrap_or("");
        let v3 = rule.v3.unwrap_or("");
        let v4 = rule.v4.unwrap_or("");
        let v5 = rule.v5.unwrap_or("");
        // 先查询
        let s = Entity::find()
            .filter(Column::Ptype.eq(rule.ptype))
            .filter(Column::V0.eq(rule.v0))
            .filter(Column::V1.eq(v1))
            .filter(Column::V2.eq(v2))
            .filter(Column::V3.eq(v3))
            .filter(Column::V4.eq(v4))
            .filter(Column::V5.eq(v5))
            .all(&txn)
            .await
            .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
        //  确认下是否存在  在添加
        if !s.is_empty() {
            continue;
        }
        // let uid = scru128::scru128();
        let p = ActiveModel {
            id: NotSet,
            ptype: Set(rule.ptype.to_string()),
            v0: Set(rule.v0.to_string()),
            v1: Set(v1.to_string()),
            v2: Set(v2.to_string()),
            v3: Set(v3.to_string()),
            v4: Set(v4.to_string()),
            v5: Set(v5.to_string()),
        };

        p.insert(&txn)
            .await
            .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    }
    txn.commit()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(true)
}
///  清除策略
pub(crate) async fn clear_policy(conn: &DatabaseConnection) -> Result<()> {
    let txn = conn
        .begin()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Entity::delete_many()
        .exec(&txn)
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    txn.commit()
        .await
        .map_err(|err| CasbinError::from(AdapterError(Box::new(Error::DbErr(err)))))?;
    Ok(())
}

///  
///
///  将策略数组格式化成全部位置都有数据
fn normalize_casbin_rule(mut rule: Vec<String>, field_index: usize) -> Vec<String> {
    rule.resize(6 - field_index, String::from(""));
    rule
}
/// 获取筛选参数
fn filtered_where_values<'a>(filter: &Filter<'a>) -> ([&'a str; 6], [&'a str; 6]) {
    let mut g_filter: [&'a str; 6] = ["%", "%", "%", "%", "%", "%"];
    let mut p_filter: [&'a str; 6] = ["%", "%", "%", "%", "%", "%"];
    for (idx, val) in filter.g.iter().enumerate() {
        if val != &"" {
            g_filter[idx] = val;
        }
    }
    for (idx, val) in filter.p.iter().enumerate() {
        if val != &"" {
            p_filter[idx] = val;
        }
    }
    (g_filter, p_filter)
}
