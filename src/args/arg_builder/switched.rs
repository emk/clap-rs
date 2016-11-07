use Arg;

pub trait GetSwitched<'e> {
    fn switched(&self) -> &Switched<'e>;
}

pub trait SwitchedArg<'e> {
    fn short(&self) -> Option<char>;
    fn long(&self) -> Option<&'e str>;
    fn aliases(&self) -> Option<Vec<&'e str>>;
    fn disp_ord(&self) -> usize;
}

impl<'e, T> SwitchedArg<'e> for T where T: GetSwitched<'e> {
    fn short(&self) -> Option<char> {
        self.switched().short
    }
    fn long(&self) -> Option<&'e str> {
        self.switched().long
    }
    fn aliases(&self) -> Option<Vec<&'e str>> {
        if let Some(ref aliases) = self.switched().aliases {
            let vis_aliases: Vec<_> =
                aliases.iter()
                    .filter_map(|&(n, v)| if v { Some(n) } else { None })
                    .collect();
            if vis_aliases.is_empty() {
                None
            } else {
                Some(vis_aliases)
            }
        } else {
            None
        }
    }
    fn disp_ord(&self) -> usize {
        self.switched().disp_ord
    }
}

#[derive(Debug)]
pub struct Switched<'b> {
    pub short: Option<char>,
    pub long: Option<&'b str>,
    pub aliases: Option<Vec<(&'b str, bool)>>, // (name, visible)
    pub disp_ord: usize,
    pub unified_ord: usize,
}

impl<'e> Default for Switched<'e> {
    fn default() -> Self {
        Switched {
            short: None,
            long: None,
            aliases: None,
            disp_ord: 999,
            unified_ord: 999,
        }
    }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Switched<'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self {
        Switched {
            short: a.short,
            long: a.long,
            aliases: a.aliases.clone(),
            disp_ord: a.disp_ord,
            .. Default::default()
        }
    }
}

impl<'e> Clone for Switched<'e> {
    fn clone(&self) -> Self {
        Switched {
            short: self.short,
            long: self.long,
            aliases: self.aliases.clone(),
            disp_ord: self.disp_ord,
            unified_ord: self.unified_ord,
        }
    }
}
