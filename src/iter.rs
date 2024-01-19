use std::mem;
use crate::Flags;
use crate::component::Component;
use crate::parse::{SomeComponents, get_next_version_component};

pub enum IteratedComponent {

}

pub struct VersionComponentIterator<'a> {
    rest_of_version: &'a str,
    needs_trailing_component: bool,
    carried_component: Option<Component<'a>>,
    flags: Flags,
}

impl VersionComponentIterator<'_> {
    pub fn new<'a>(version: &'a str, flags: Flags) -> VersionComponentIterator<'a> {
        return VersionComponentIterator{
            rest_of_version: version,
            needs_trailing_component: flags.contains(Flags::LowerBound | Flags::UpperBound),
            carried_component: None,
            flags
        };
    }

    pub fn next(&mut self) -> Component {
        if let Some(component) = mem::take(&mut self.carried_component) {
            return component;
        }

        let (components, rest_of_version) = get_next_version_component(self.rest_of_version, self.flags);

        self.rest_of_version = rest_of_version;

        match components {
            SomeComponents::One(component) => {
                return component;
            },
            SomeComponents::Two(component1, component2) => {
                self.carried_component = Some(component2);
                return component1;
            }
        }
    }

    pub fn is_exhausted(&self) -> bool {
        return self.rest_of_version.is_empty();
    }
}

