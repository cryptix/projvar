// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::environment::Environment;
use crate::tools;
use crate::var::Key;
use std::error::Error;
use std::fmt;

use super::var;
pub struct VarSource;

type BoxResult<T> = Result<T, Box<dyn Error>>;

// TODO Move this elsewhere
fn is_branch(environment: &mut Environment, refr: &str) -> BoxResult<Option<String>> {
    let mut branch = None;
    if let Some(repo) = environment.repo() {
        let checked_out_branch = repo.branch()?;
        if let Some(checked_out_branch) = checked_out_branch {
            if refr.ends_with(&format!("/{}", &checked_out_branch)) {
                branch = Some(refr);
            }
        }
    }
    Ok(branch.map(std::borrow::ToOwned::to_owned))
}

// TODO Move this elsewhere
fn is_tag(environment: &mut Environment, refr: &str) -> BoxResult<Option<String>> {
    let mut tag = None;
    if let Some(repo) = environment.repo() {
        let checked_out_branch = repo.tag()?;
        if let Some(checked_out_branch) = checked_out_branch {
            if refr.ends_with(&format!("/{}", &checked_out_branch)) {
                tag = Some(refr);
            }
        }
    }
    Ok(tag.map(std::borrow::ToOwned::to_owned))
}

impl super::VarSource for VarSource {
    fn is_usable(&self, _environment: &mut Environment) -> bool {
        true
    }

    fn retrieve(&self, environment: &mut Environment, key: Key) -> BoxResult<Option<String>> {
        Ok(match key {
            Key::Name => super::proj_name_from_slug(environment.vars.get("GITHUB_REPOSITORY"))?, // usually: GITHUB_REPOSITORY="user/project"
            Key::RepoWebUrl => {
                match (
                    environment.vars.get("GITHUB_SERVER_URL"),
                    environment.vars.get("GITHUB_REPOSITORY"),
                ) {
                    (Some(server), Some(repo)) => {
                        // "${GITHUB_SERVER_URL}/${GITHUB_REPOSITORY}/"
                        // usually:
                        // GITHUB_SERVER_URL="https://github.com/"
                        // GITHUB_REPOSITORY="user/project"
                        Some(format!("{}/{}/", server, repo))
                    }
                    (_, _) => None,
                }
            }
            Key::RepoFrozenWebUrl => super::try_construct_frozen(self, environment)?,
            Key::Ci => var(environment, "CI"),
            Key::BuildBranch => {
                let refr = var(environment, "GITHUB_REF");
                if let Some(refr) = refr {
                    is_branch(environment, &refr)?
                } else {
                    None
                }
            }
            Key::BuildTag => {
                let refr = var(environment, "GITHUB_REF");
                if let Some(refr) = refr {
                    is_tag(environment, &refr)?
                } else {
                    None
                }
            }
            Key::RepoCloneUrl => {
                let repo_web_url = self.retrieve(environment, Key::RepoWebUrl)?;
                if let Some(repo_web_url) = repo_web_url {
                    // usually:
                    // https://github.com/hoijui/nim-ci.git
                    // https://gitlab.com/hoijui/tebe.git
                    Some(tools::git::web_to_clone_url(&repo_web_url, false)?)
                } else {
                    None
                }
            }
            Key::BuildHostingUrl => {
                let repo_web_url = self.retrieve(environment, Key::RepoWebUrl)?;
                if let Some(repo_web_url) = repo_web_url {
                    Some(tools::git::web_to_build_hosting_url(&repo_web_url)?) // TODO This currently comes without final '/', is that OK?
                } else {
                    None
                }
            }
            Key::BuildOs => var(environment, "RUNNER_OS"), // TODO Not sure if this makes sense ... have to check in practise!
            Key::BuildIdent => var(environment, "GITHUB_SHA"),
            Key::Version
            | Key::BuildDate
            | Key::VersionDate
            | Key::BuildOsFamily
            | Key::BuildArch
            | Key::License
            | Key::BuildNumber => None,
        })
    }
}

impl fmt::Display for VarSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::any::type_name::<VarSource>())
    }
}
