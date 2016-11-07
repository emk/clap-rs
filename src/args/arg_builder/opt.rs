// Std
use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;
use std::result::Result as StdResult;

// Third Party
use vec_map::VecMap;

// Internal
use args::{AnyArg, Arg, DispOrder, Valued, Switched, Base, ValuedArg, BaseArg, SwitchedArg, GetValued, GetSwitched, GetBase};
use args::settings::{ArgFlags, ArgSettings};

#[derive(Default, Clone)]
#[allow(missing_debug_implementations)]
#[doc(hidden)]
pub struct Opt<'n, 'e> where 'n: 'e {
    pub b: Base<'n, 'e>,
    pub s: Switched<'e>,
    pub v: Valued<'n, 'e>,
}

impl<'n, 'e> Opt<'n, 'e> {
    pub fn new(name: &'n str) -> Self {
        Opt { b: Base::new(name), ..Default::default() }
    }

    pub fn from_arg(a: &Arg<'n, 'e>) -> Self {
        // TODO: check this function...

        // No need to check for .index() as that is handled above
        let mut ob = Opt::from(a);
        
        if let Some(ref vec) = ob.val_names() {
            if vec.len() > 1 {
                ob.v.num_vals = Some(vec.len() as u64);
            }
        }
        if let Some(ref vec) = ob.val_names() {
            if vec.len() > 1 {
                ob.v.num_vals = Some(vec.len() as u64);
            }
        }
        if let Some(ref p) = a.validator {
            ob.v.validator = Some(p.clone());
        }
        ob
    }

    pub fn set(&mut self, s: ArgSettings) {
        self.b.set(s);
    }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Opt<'n, 'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self {
        Opt {
            b: Base::from(a),
            s: Switched::from(a),
            v: Valued::from(a),
        }
    }
}

impl<'n, 'e> Display for Opt<'n, 'e> {
    fn fmt(&self, f: &mut Formatter) -> Result {
        debugln!("fn=fmt");
        // Write the name such --long or -l
        if let Some(l) = self.long() {
            try!(write!(f, "--{} ", l));
        } else {
            try!(write!(f, "-{} ", self.short().unwrap()));
        }

        // Write the values such as <name1> <name2>
        if let Some(ref vec) = self.val_names() {
            let mut it = vec.iter().peekable();
            while let Some((_, val)) = it.next() {
                try!(write!(f, "<{}>", val));
                if it.peek().is_some() {
                    try!(write!(f, " "));
                }
            }
            let num = vec.len();
            if self.is_set(ArgSettings::Multiple) && num == 1 {
                try!(write!(f, "..."));
            }
        } else if let Some(num) = self.num_vals() {
            let mut it = (0..num).peekable();
            while let Some(_) = it.next() {
                try!(write!(f, "<{}>", self.name()));
                if it.peek().is_some() {
                    try!(write!(f, " "));
                }
            }
        } else {
            try!(write!(f,
                        "<{}>{}",
                        self.name(),
                        if self.is_set(ArgSettings::Multiple) {
                            "..."
                        } else {
                            ""
                        }));
        }

        Ok(())
    }
}

impl<'n, 'e> GetBase<'n, 'e> for Opt<'n, 'e> {
    fn base(&self) -> &Base<'n, 'e> {
        &self.b
    }
}

impl<'n, 'e> GetSwitched<'e> for Opt<'n, 'e> {
    fn switched(&self) -> &Switched<'e> {
        &self.s
    }
}

impl<'n, 'e> GetValued<'n, 'e> for Opt<'n, 'e> {
    fn valued(&self) -> &Valued<'n, 'e> {
        &self.v
    }
}
#[cfg(test)]
mod test {
    use args::settings::ArgSettings;
    use super::OptBuilder;
    use vec_map::VecMap;

    #[test]
    fn optbuilder_display1() {
        let mut o = OptBuilder::new("opt");
        o.long = Some("option");
        o.settings.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", o), "--option <opt>...");
    }

    #[test]
    fn optbuilder_display2() {
        let mut v_names = VecMap::new();
        v_names.insert(0, "file");
        v_names.insert(1, "name");

        let mut o2 = OptBuilder::new("opt");
        o2.short = Some('o');
        o2.val_names = Some(v_names);

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }

    #[test]
    fn optbuilder_display3() {
        let mut v_names = VecMap::new();
        v_names.insert(0, "file");
        v_names.insert(1, "name");

        let mut o2 = OptBuilder::new("opt");
        o2.short = Some('o');
        o2.val_names = Some(v_names);
        o2.settings.set(ArgSettings::Multiple);

        assert_eq!(&*format!("{}", o2), "-o <file> <name>");
    }

    #[test]
    fn optbuilder_display_single_alias() {
        let mut o = OptBuilder::new("opt");
        o.long = Some("option");
        o.aliases = Some(vec![("als", true)]);

        assert_eq!(&*format!("{}", o), "--option <opt>");
    }

    #[test]
    fn optbuilder_display_multiple_aliases() {
        let mut o = OptBuilder::new("opt");
        o.long = Some("option");
        o.aliases = Some(vec![
                         ("als_not_visible", false),
                         ("als2", true),
                         ("als3", true),
                         ("als4", true)
                    ]);
        assert_eq!(&*format!("{}", o), "--option <opt>");
    }
}
