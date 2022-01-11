extern crate sqlite;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub struct RepoInfo {
    pub repourl: String,
    pub revision: String,
}

impl fmt::Display for RepoInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Repo Url: {}\nRepo Reversion: {}", self.repourl, self.revision)
    }
}

fn get_sqlite_ret(connection: &sqlite::Connection, query: &str) -> Result<String, sqlite::Error> {
    connection
        .prepare(query)
        .and_then(|statement| Ok(statement.into_cursor()))
        .and_then(|mut cursor| {
            let mut root = String::new();
            while let Some(row) = cursor.next().unwrap() {
                root = row[0]
                    .as_string()
                    .and_then(|row| Some(row.to_string()))
                    .unwrap();
            }
            Ok(root)
        })
}

fn get_repo_root(connection: &sqlite::Connection) -> Result<String, sqlite::Error> {
    let query = "SELECT `root` FROM REPOSITORY LIMIT 1";
    get_sqlite_ret(connection, query)
}

fn get_revision(connection: &sqlite::Connection) -> Result<String, sqlite::Error> {
    let query = "SELECT CAST(MAX(`revision`) AS VARCHAR) FROM NODES_CURRENT";
    get_sqlite_ret(connection, query)
}

fn get_repo_path(connection: &sqlite::Connection) -> Result<String, sqlite::Error> {
    let query = "SELECT `repos_path` FROM NODES_CURRENT WHERE LENGTH(`repos_path`) = (SELECT MIN(LENGTH(`repos_path`)) FROM `NODES_CURRENT`)";
    get_sqlite_ret(&connection, query)
}

pub fn get_svn_repo_info(repo_path: &PathBuf) -> RepoInfo {
    let mut repo_info = RepoInfo {
        repourl: "".to_owned(),
        revision: "".to_owned(),
    };
    if let Ok(Ok(result)) = sqlite::open(repo_path.join(".svn/wc.db"))
        .map_err(|err| {
            panic!("{}", err.to_string());
        })
        .and_then(|con| {
            Ok(get_repo_root(&con).and_then(|root| {
                get_repo_path(&con)
                    .and_then(|path| Ok(root + path.as_str()))
                    .and_then(|repourl| {
                        get_revision(&con).and_then(|revison| {
                            Ok(RepoInfo {
                                repourl: repourl.to_owned(),
                                revision: revison.to_owned(),
                            })
                        })
                    })
            }))
        })
    {
        repo_info = result;
    };
    repo_info
}
