pub mod jenkins;
mod svn;
mod subpack;

pub use jenkins::{create_default_config_file, JenkinsInput, JenkinsInfo, get_jenkins_config, JenkinsToml, dump_jenkins_toml, get_jenkins_toml};
use std::path::PathBuf;
use svn::get_svn_repo_info;
use subpack::get_sub_packages;
use std::fs;

pub fn post_with(repopath: &PathBuf, jenkins_toml: JenkinsToml) {
    let url;
    let repo_path = fs::canonicalize(repopath).unwrap();
    let readme = jenkins_toml.readme();
    let hyphen = jenkins_toml.hyphen();
    let svn_repo_info = get_svn_repo_info(&repo_path);
    let mut req_args: Vec<(&str, &str)> = vec![("hyphen", &hyphen), ("readme", &readme)];
    let proid = jenkins_toml.proid();
    let first_name = jenkins_toml.first_name();
    let packages = get_sub_packages(&repo_path);
    for i in 0..packages.len() {
        req_args.push(("upgrade_content", packages[i].as_str()));
    }
    req_args.push(("svn_path", svn_repo_info.repourl.as_str()));
    req_args.push(("svn_revision", svn_repo_info.revision.as_str()));
    req_args.push(("proid", &proid));
    req_args.push(("first_name", &first_name));
    println!("{:?}", req_args);
    match get_jenkins_config() {
        Some(jk_config) => {
            url = jk_config.get_url();
            ureq::post(&url).send_form(&req_args).unwrap();
        }
        _ => (),
    }
}
