// SPDX-FileCopyrightText: Copyright 2024 Dmitry Marakasov <amdmi3@amdmi3.ru>
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::mem;

use crate::VersionFlags;
use crate::component::Component;
use crate::parse::{SomeComponents, get_next_version_component};

pub(crate) struct VersionComponentIterator<'a> {
    rest_of_version: &'a str,
    carried_component: Option<Component<'a>>,
    flags: VersionFlags,
}

impl VersionComponentIterator<'_> {
    pub(crate) fn new(version: &str, flags: VersionFlags) -> VersionComponentIterator<'_> {
        VersionComponentIterator {
            rest_of_version: version,
            carried_component: None,
            flags,
        }
    }

    pub(crate) fn next(&mut self) -> Component<'_> {
        if let Some(component) = mem::take(&mut self.carried_component) {
            return component;
        }

        let (components, rest_of_version) =
            get_next_version_component(self.rest_of_version, self.flags);

        self.rest_of_version = rest_of_version;

        match components {
            SomeComponents::One(component) => component,
            SomeComponents::Two(component1, component2) => {
                self.carried_component = Some(component2);
                component1
            }
        }
    }

    pub(crate) fn is_exhausted(&self) -> bool {
        self.rest_of_version.is_empty() && self.carried_component.is_none()
    }
}
