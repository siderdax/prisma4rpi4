use crate::{runtime::run_with_tokio, AnyError, Tags};
use enumflags2::BitFlags;
use quaint::{prelude::Queryable, single::Quaint};
use url::Url;

/// The maximum length of identifiers on mysql is 64 bytes.
///
/// Source: https://dev.mysql.com/doc/mysql-reslimits-excerpt/5.5/en/identifier-length.html
pub fn mysql_safe_identifier(identifier: &str) -> &str {
    if identifier.len() < 64 {
        identifier
    } else {
        identifier.get(0..63).expect("mysql identifier truncation")
    }
}

pub(crate) fn get_mysql_tags(database_url: &str) -> Result<BitFlags<Tags>, String> {
    let fut = async {
        let quaint = Quaint::new(database_url).await.map_err(|err| err.to_string())?;
        let mut tags = Tags::Mysql.into();

        let metadata = quaint
            .query_raw(
                "SELECT @@lower_case_table_names lower_cases_table_names, @@GLOBAL.version version",
                &[],
            )
            .await
            .map_err(|err| err.to_string())?;

        let first_row = metadata
            .first()
            .ok_or_else(|| "Got an empty result set when fetching metadata".to_owned())?;

        match first_row.get("lower_cases_table_names").and_then(|lctn| lctn.as_i64()) {
            Some(1) => tags |= Tags::LowerCasesTableNames,
            _ => (),
        }

        match first_row.get("version").and_then(|version| version.to_string()) {
            None => Ok(tags),
            Some(version) => {
                eprintln!("Version: {:?}", version);

                if version.contains("5.6") {
                    tags |= Tags::Mysql56
                }

                if version.contains("5.7") {
                    tags |= Tags::Mysql57
                }

                if version.contains("8.") {
                    tags |= Tags::Mysql8
                }

                if version.contains("MariaDB") {
                    tags |= Tags::Mariadb
                }

                match std::env::var("IS_VITESS").as_deref() {
                    Err(_) | Ok("0") => (),
                    _ => tags |= Tags::Vitess,
                }

                eprintln!("Inferred tags: {:?}", tags);

                Ok(tags)
            }
        }
    };

    run_with_tokio(fut)
}

/// Returns a connection to the new database, as well as the corresponding
/// complete connection string.
pub async fn create_mysql_database(db_name: &str) -> Result<(Quaint, String), AnyError> {
    let mut url: Url = super::TAGS.as_ref().unwrap().database_url.parse()?;
    let mut mysql_db_url = url.clone();

    mysql_db_url.set_path("/mysql");
    url.set_path(db_name);

    debug_assert!(!db_name.is_empty());
    debug_assert!(
        db_name.len() < 64,
        "db_name should be less than 64 characters, got {:?}",
        db_name.len()
    );

    let conn = Quaint::new(&mysql_db_url.to_string()).await?;

    let drop = format!(
        r#"
        DROP DATABASE IF EXISTS `{db_name}`;
        "#,
        db_name = db_name,
    );

    let recreate = format!(
        r#"
        CREATE DATABASE `{db_name}`;
        "#,
        db_name = db_name,
    );

    // The two commands have to be run separately on mariadb.
    conn.raw_cmd(&drop).await?;
    conn.raw_cmd(&recreate).await?;
    let url_str = url.to_string();

    Ok((Quaint::new(&url_str).await?, url_str))
}
