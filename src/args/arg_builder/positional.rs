// Std
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;
use std::result::Result as StdResult;

// Third Party
use vec_map::VecMap;

// Internal
use args::{AnyArg, Arg, DispOrder, Valued, Base, ValuedArg, BaseArg, GetValued, GetBase};
use args::settings::{ArgFlags, ArgSettings};

#[derive(Default)]
#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct Positional<'n, 'e> where 'n: 'e {
    b: Base<'n, 'e>,
    v: Valued<'n, 'e>,
    index: usize,
}

impl<'n, 'e> Positional<'n, 'e> {
    pub fn new(name: &'n str, idx: u64) -> Self {
        Positional {
            b: Base::new(name),
            index: idx,
            ..Default::default()
        }
    }

    pub fn from_arg(a: &Arg<'n, 'e>, idx: u64) -> Self {
        debug_assert!(a.short.is_none() || a.long.is_none(),
                      format!("Argument \"{}\" has conflicting requirements, both index() and \
                      short(), or long(), were supplied",
                              a.name));

        let mut pb = Positional {
            b: Base::from(a),
            v: Valued::from(a),
            ..Default::default()
        };
        if a.max_vals.is_some() || a.min_vals.is_some() ||
           (a.num_vals.is_some() && a.num_vals.unwrap() > 1) {
            pb.set(ArgSettings::Multiple);
        }
        pb
    }

    #[inline]
    pub fn multiple_str(&self) -> &str {
        if self.is_set(ArgSettings::Multiple) && self.val_names().is_none() {
            "..."
        } else {
            ""
        }
    }

    pub fn set(&mut self, s: ArgSettings) {
        self.b.set(s);
    }

    pub fn name_no_brackets(&self) -> Cow<str> {
        if let Some(ref names) = self.val_names() {
            Cow::Owned(names.values()
                .map(|n| format!("<{}>", n))
                .collect::<Vec<_>>()
                .join(" "))
        } else {
            Cow::Borrowed(self.name())
        }
    }
}

impl<'n, 'e> Display for Positional<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // TODO: see if we can use name_no_brackets()
        if let Some(ref names) = self.val_names() {
            try!(write!(f,
                        "{}",
                        names.values()
                            .map(|n| format!("<{}>", n))
                            .collect::<Vec<_>>()
                            .join(" ")));
        } else {
            try!(write!(f, "<{}>", self.name()));
        }
        try!(write!(f, "{}", self.multiple_str()));

        Ok(())
    }
}

impl<'n, 'e> GetBase<'n, 'e> for Positional<'n, 'e> {
    fn base(&self) -> &Base<'n, 'e> {
        &self.b
    }
}

impl<'n, 'e> GetValued<'n, 'e> for Positional<'n, 'e> {
    fn valued(&self) -> &Valued<'n, 'e> {
        &self.v
    }
}

#[cfg(test)]
mod test {
    use args::settings::ArgSettings;
    use super::Positional;
    use vec_map::VecMap;

    #[test]
    fn display_mult() {
        let mut p = Positional::new("pos", 1);
        p.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", p), "<pos>...");
    }

    #[test]
    fn display_required() {
        let mut p2 = Positional::new("pos", 1);
        p2.set(ArgSettings::Required);

        assert_eq!(&*format!("{}", p2), "<pos>");
    }

    #[test]
    fn display_val_names() {
        let mut p2 = Positional::new("pos", 1);
        let mut vm = VecMap::new();
        vm.insert(0, "file1");
        vm.insert(1, "file2");
        p2.v.val_names = Some(vm);

        assert_eq!(&*format!("{}", p2), "<file1> <file2>");
    }

    #[test]
    fn display_val_names_req() {
        let mut p2 = Positional::new("pos", 1);
        p2.set(ArgSettings::Required);
        let mut vm = VecMap::new();
        vm.insert(0, "file1");
        vm.insert(1, "file2");
        p2.v.val_names = Some(vm);

        assert_eq!(&*format!("{}", p2), "<file1> <file2>");
    }
}
