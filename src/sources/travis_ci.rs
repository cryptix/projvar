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

    #[remain::check]
    fn retrieve(&self, environment: &mut Environment, key: Key) -> BoxResult<Option<String>> {
        Ok(
            #[remain::sorted]
            match key {
                Key::BuildArch
                | Key::BuildHostingUrl
                | Key::BuildDate
                | Key::BuildOsFamily
                | Key::Ci
                | Key::Licenses
                | Key::License
                | Key::RepoIssuesUrl
                | Key::RepoCloneUrl
                | Key::RepoCommitPrefixUrl
                | Key::RepoRawVersionedPrefixUrl
                | Key::RepoVersionedDirPrefixUrl
                | Key::RepoVersionedFilePrefixUrl
                | Key::RepoWebUrl
                | Key::VersionDate => None,
                Key::BuildBranch => var(environment, "TRAVIS_BRANCH"),
                Key::BuildNumber => var(environment, "TRAVIS_BUILD_NUMBER"),
                Key::BuildOs => var(environment, "TRAVIS_OS_NAME"),
                Key::BuildTag => var(environment, "TRAVIS_TAG"),
                Key::Name => super::proj_name_from_slug(environment.vars.get("TRAVIS_REPO_SLUG"))?, // usually: TRAVIS_REPO_SLUG="user/project"
                Key::NameMachineReadable => {
                    super::try_construct_machine_readable_name_from_name(self, environment)?
                }
                Key::Version => var(environment, "TRAVIS_COMMIT"),
            },
        )
    }
}
