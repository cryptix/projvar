// SPDX-FileCopyrightText: 2021 Robin Vobruba <hoijui.quaero@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::environment::Environment;
use crate::var::Key;
use std::error::Error;

use super::var;
use super::Hierarchy;

pub struct VarSource;

type BoxResult<T> = Result<T, Box<dyn Error>>;

impl super::VarSource for VarSource {
    fn is_usable(&self, _environment: &mut Environment) -> bool {
        true
    }

    fn hierarchy(&self) -> Hierarchy {
        Hierarchy::High
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<VarSource>()
    }

    fn properties(&self) -> &Vec<String> {
        &super::NO_PROPS
    }

    fn retrieve(&self, environment: &mut Environment, key: Key) -> BoxResult<Option<String>> {
        Ok(match key {
            Key::Name => var(environment, "BITBUCKET_PROJECT_KEY"),
            Key::RepoWebUrl => {
                // BITBUCKET_REPO_FULL_NAME = The full name of the repository
                // (everything that comes after http://bitbucket.org/).
                var(environment, "BITBUCKET_REPO_FULL_NAME")
                    .map(|project_slug| format!("http://bitbucket.org/{}", project_slug))
            }
            Key::RepoVersionedWebUrl => super::try_construct_versioned(self, environment)?,
            Key::Ci => var(environment, "CI"),
            Key::BuildBranch => var(environment, "BITBUCKET_BRANCH"),
            Key::BuildTag => var(environment, "BITBUCKET_TAG"),
            Key::RepoCloneUrl => {
                // NOTE:
                // In reality, either both of these or none are set,
                // so we will never use BITBUCKET_GIT_SSH_ORIGIN, but formally,
                // it makes sense, and can be seen as a form of documentation,
                // which at some point might become handy.
                var(environment, "BITBUCKET_GIT_HTTP_ORIGIN")
                    .or_else(|| var(environment, "BITBUCKET_GIT_SSH_ORIGIN"))
            }
            Key::Version => var(environment, "BITBUCKET_COMMIT"),
            Key::BuildNumber => var(environment, "BITBUCKET_BUILD_NUMBER"),
            Key::RepoIssuesUrl => super::try_construct_issues_url(self, environment)?,
            Key::BuildHostingUrl
            | Key::BuildOs
            | Key::VersionDate
            | Key::BuildDate
            | Key::BuildOsFamily
            | Key::BuildArch
            | Key::License => None,
        })
    }
}
