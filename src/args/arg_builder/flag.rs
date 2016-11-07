// Std
use std::convert::From;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;
use std::result::Result as StdResult;

// Third Party
use vec_map::VecMap;

// Internal
use args::{AnyArg, Arg, DispOrder, Switched, Base, SwitchedArg, BaseArg, GetSwitched, GetBase};
use args::settings::{ArgFlags, ArgSettings};

#[derive(Debug, Default, Clone)]
#[doc(hidden)]
pub struct Flag<'n, 'e> where 'n: 'e {
    pub b: Base<'n, 'e>,
    pub s: Switched<'e>,
}

impl<'n, 'e> Flag<'n, 'e> {
    pub fn new(name: &'n str) -> Self {
        Flag { b: Base::new(name), ..Default::default() }
    }

    pub fn set(&mut self, s: ArgSettings) {
        self.b.set(s);
    }
}

impl<'a, 'b, 'z> From<&'z Arg<'a, 'b>> for Flag<'a, 'b> {
    fn from(a: &'z Arg<'a, 'b>) -> Self {
        assert!(!a.b.is_set(ArgSettings::Required),
                format!("The argument '{}' cannot be required because it's a flag, perhaps you \
                forgot takes_value(true)?",
                        a.name));

        Flag {
            b: Base::from(a),
            s: Switched::from(a),
        }
    }
}

impl<'n, 'e> Display for Flag<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        if let Some(l) = self.long() {
            try!(write!(f, "--{}", l));
        } else {
            try!(write!(f, "-{}", self.short().unwrap()));
        }

        Ok(())
    }
}

impl<'n, 'e> GetBase<'n, 'e> for Flag<'n, 'e> {
    fn base(&self) -> &Base<'n, 'e> {
        &self.b
    }
}

impl<'n, 'e> GetSwitched<'e> for Flag<'n, 'e> {
    fn switched(&self) -> &Switched<'e> {
        &self.s
    }
}

#[cfg(test)]
mod test {
    use args::settings::ArgSettings;
    use super::Flag;

    #[test]
    fn flag_display_long() {
        let mut f = Flag::new("flg");
        f.set(ArgSettings::Multiple);
        f.s.long = Some("flag");

        assert_eq!(&*format!("{}", f), "--flag");
    }

    #[test]
    fn flag_display_short() {
        let mut f2 = Flag::new("flg");
        f2.s.short = Some('f');

        assert_eq!(&*format!("{}", f2), "-f");
    }

    #[test]
    fn flag_display_single_alias() {
        let mut f = Flag::new("flg");
        f.s.long = Some("flag");
        f.s.aliases = Some(vec![("als", true)]);

        assert_eq!(&*format!("{}", f), "--flag");
    }

    #[test]
    fn flag_display_multiple_aliases() {
        let mut f = Flag::new("flg");
        f.s.short = Some('f');
        f.s.aliases = Some(vec![
                         ("alias_not_visible", false),
                         ("f2", true),
                         ("f3", true),
                         ("f4", true)
                    ]);
        assert_eq!(&*format!("{}", f), "-f");
    }
}
