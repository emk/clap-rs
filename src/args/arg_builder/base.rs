use args::{ArgSettings, Arg, ArgFlags};

pub trait GetBase<'n, 'e> {
    fn base(&self) -> &Base<'n, 'e>;
}

pub trait BaseArg<'n, 'e> {
    fn name(&self) -> &'n str;
    fn help(&self) -> Option<&'e str>;
    fn blacklist(&self) -> Option<&[&str]>;
    fn required_unless(&self) -> Option<&[&str]>;
    fn overrides(&self) -> Option<&[&str]>;
    fn groups(&self) -> Option<&[&str]>;
    fn requires(&self) -> Option<&[&str]>;
    fn is_set(&self, ArgSettings) -> bool;
}

impl<'n, 'e, T> BaseArg<'n, 'e> for T where T: GetBase<'n, 'e> {
    fn name(&self) -> &'n str {
        self.base().name
    }
    fn overrides(&self) -> Option<&[&str]> {
        self.base().overrides.as_ref().map(|o| &o[..])
    }
    fn requires(&self) -> Option<&[&str]> {
        self.base().requires.as_ref().map(|o| &o[..])
    }
    fn blacklist(&self) -> Option<&[&str]> {
        self.base().blacklist.as_ref().map(|o| &o[..])
    }
    fn required_unless(&self) -> Option<&[&str]> {
        self.base().r_unless.as_ref().map(|o| &o[..])
    }
    fn is_set(&self, s: ArgSettings) -> bool {
        self.base().settings.is_set(s)
    }
    fn help(&self) -> Option<&'e str> {
        self.base().help
    }
    fn groups(&self) -> Option<&[&str]> {
        self.base().groups.as_ref().map(|o| &o[..])
    }
}


#[derive(Debug)]
pub struct Base<'a, 'b> where 'a: 'b {
    pub name: &'a str,
    pub help: Option<&'b str>,
    pub blacklist: Option<Vec<&'a str>>,
    pub settings: ArgFlags,
    pub r_unless: Option<Vec<&'a str>>,
    pub overrides: Option<Vec<&'a str>>,
    pub groups: Option<Vec<&'a str>>,
    pub requires: Option<Vec<&'a str>>,
}

impl<'n, 'e> Default for Base<'n, 'e> {
    fn default() -> Self {
        Base {
            name: "",
            help: None,
            blacklist: None,
            settings: ArgFlags::new(),
            r_unless: None,
            overrides: None,
            requires: None,
            groups: None,
        }
    }
}

impl<'n, 'e> Base<'n, 'e> {
    pub fn new(name: &'n str) -> Self {
        Base { name: name, ..Default::default() }
    }

    pub fn set(&mut self, s: ArgSettings) {
        self.settings.set(s);
    }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Base<'n, 'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self {
        Base {
            name: a.name, 
            help: a.help,
            blacklist: a.blacklist,
            settings: a.settings,
            r_unless: a.r_unless,
            overrides: a.overrides,
            requires: a.requires,
            groups: a.groups,
        }
    }
}

impl<'n, 'e> Clone for Base<'n, 'e> {
    fn clone(&self) -> Self {
        Base {
            name: self.name,
            help: self.help,
            blacklist: self.blacklist.clone(),
            overrides: self.overrides.clone(),
            requires: self.requires.clone(),
            settings: self.settings,
            groups: self.groups.clone(),
            r_unless: self.r_unless.clone(),
        }
    }
}
