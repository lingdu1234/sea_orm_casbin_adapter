#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use sea_orm_casbin_adapter::{casbin::prelude::*, SeaOrmAdapter};
    ///  step 0

    const DB_LINK: &str = "mysql://root:lingdu515639@127.0.0.1:13306/wk";
    // const DB_LINK,true: &str = "postgres://postgres:lingdu515639@127.0.0.1:25432/wk";

    /// step 1
    ///
    #[tokio::test]
    async fn add_policy() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        e.add_policy(vec!["am".to_string(), "ad".to_string(), "rw".to_string()])
            .await
            .unwrap();
        e.add_policy(vec!["a".to_string(), "b".to_string(), "c".to_string()])
            .await
            .unwrap();
        Ok(())
    }
    /// step 2
    ///
    #[tokio::test]
    async fn add_policies() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        e.add_policies(vec![
            vec!["bm".to_string(), "bd".to_string(), "rw".to_string()],
            vec!["bm".to_string(), "be".to_string(), "r".to_string()],
        ])
        .await
        .unwrap();
        Ok(())
    }

    /// step 3
    ///
    #[tokio::test]
    async fn add_named_policy() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        e.add_named_policy("g", vec!["cm".to_string(), "cd".to_string()])
            .await
            .unwrap();
        Ok(())
    }

    /// step 4
    ///
    #[tokio::test]
    async fn add_named_policies() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        e.add_named_policies(
            "g",
            vec![
                vec!["dm".to_string(), "dd".to_string()],
                vec!["dm".to_string(), "ee".to_string()],
            ],
        )
        .await
        .unwrap();

        Ok(())
    }
    /// step 5
    /// remove_policy
    #[tokio::test]
    async fn remove_policy() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        e.remove_policy(vec!["a".to_string(), "b".to_string(), "c".to_string()])
            .await
            .unwrap();
        Ok(())
    }

    /// step 6
    /// remove_policies
    #[tokio::test]
    async fn remove_policies() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        e.remove_policies(vec![
            vec!["bm".to_string(), "bd".to_string(), "rw".to_string()],
            vec!["bm".to_string(), "be".to_string(), "r".to_string()],
        ])
        .await
        .unwrap();
        Ok(())
    }

    /// step 7
    /// remove_named_policy
    #[tokio::test]
    async fn remove_named_policy() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        e.remove_named_policy("g", vec!["cm".to_string(), "cd".to_string()])
            .await
            .unwrap();
        Ok(())
    }

    /// step 8
    /// remove_named_policies
    #[tokio::test]
    async fn remove_named_policies() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        e.remove_named_policies(
            "g",
            vec![
                vec!["dm".to_string(), "dd".to_string()],
                vec!["dm".to_string(), "ee".to_string()],
            ],
        )
        .await
        .unwrap();
        Ok(())
    }

    /// step 9
    /// remove_filtered_policy
    #[tokio::test]
    async fn remove_filtered_policy() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        e.remove_filtered_policy(0, vec!["bm".to_string()])
            .await
            .unwrap();
        Ok(())
    }

    /// step 10
    /// remove_filtered_named_policy
    #[tokio::test]
    async fn remove_filtered_named_policy() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        e.remove_filtered_named_policy("g", 0, vec!["dm".to_string()])
            .await
            .unwrap();
        e.remove_filtered_named_policy("p", 1, vec!["b".to_string()])
            .await
            .unwrap();
        Ok(())
    }

    /// step 10
    /// remove_filtered_named_policy
    #[tokio::test]
    async fn test_enforce() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        e.enforce(("am", "ad", "rw")).unwrap();
        e.enforce(("am", "ad", "cd")).unwrap();
        e.load_policy().await.unwrap();
        e.load_filtered_policy(Filter {
            p: vec!["bm", "bd"],
            g: vec!["dm", "dd"],
        })
        .await
        .unwrap();
        Ok(())
    }

    /// step 10
    /// remove_filtered_named_policy
    #[tokio::test]
    async fn get_filtered_named_policy() -> Result<()> {
        let m = DefaultModel::from_file("config/casbin_conf/rbac_model.conf")
            .await
            .unwrap();
        let adpt = SeaOrmAdapter::new(DB_LINK, true)
            .await
            .expect("open db error");

        let mut e = Enforcer::new(m, adpt).await?;
        e.enable_log(true);
        let aaa = e.get_filtered_grouping_policy(0, vec!["dm".to_string()]);
        println!("----------------------->{:?}", aaa);
        Ok(())
    }
}
