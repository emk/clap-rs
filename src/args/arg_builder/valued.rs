use std::rc::Rc;
use std::result::Result as StdResult;

use vec_map::VecMap;

use Arg;

pub trait GetValued<'a, 'b> {
    fn valued(&self) -> &Valued<'a, 'b>;
}

pub trait ValuedArg<'n, 'e> {
    fn possible_vals(&self) -> Option<&[&'e str]>;
    fn val_names(&self) -> Option<&VecMap<&'e str>>;
    fn num_vals(&self) -> Option<u64>;
    fn max_vals(&self) -> Option<u64>;
    fn min_vals(&self) -> Option<u64>;
    fn validator(&self) -> Option<&Rc<Fn(String) -> Result<(), String>>>;
    fn val_delim(&self) -> Option<char>;
    fn default_val(&self) -> Option<&'n str>;
}

impl<'n, 'e, T> ValuedArg<'n, 'e> for T where T: GetValued<'n, 'e> {
    fn val_names(&self) -> Option<&VecMap<&'e str>> {
        self.valued().val_names.as_ref()
    }
    fn max_vals(&self) -> Option<u64> {
        self.valued().max_vals
    }
    fn num_vals(&self) -> Option<u64> {
        self.valued().num_vals
    }
    fn possible_vals(&self) -> Option<&[&'e str]> {
        self.valued().possible_vals.as_ref().map(|o| &o[..])
    }
    fn validator(&self) -> Option<&Rc<Fn(String) -> StdResult<(), String>>> {
        self.valued().validator.as_ref()
    }
    fn min_vals(&self) -> Option<u64> {
        self.valued().min_vals
    }
    fn val_delim(&self) -> Option<char> {
        self.valued().val_delim
    }
    fn default_val(&self) -> Option<&'n str> {
        self.valued().default_val
    }
}

pub struct Valued<'a, 'b> {
    pub possible_vals: Option<Vec<&'b str>>,
    pub val_names: Option<VecMap<&'b str>>,
    pub num_vals: Option<u64>,
    pub max_vals: Option<u64>,
    pub min_vals: Option<u64>,
    pub validator: Option<Rc<Fn(String) -> Result<(), String>>>,
    pub val_delim: Option<char>,
    pub default_val: Option<&'a str>,
}

impl<'n, 'e> Default for Valued<'n, 'e> {
    fn default() -> Self {
        Valued {
            possible_vals: None,
            num_vals: None,
            min_vals: None,
            max_vals: None,
            val_names: None,
            validator: None,
            val_delim: Some(','),
            default_val: None,
        }
    }
}

impl<'n, 'e, 'z> From<&'z Arg<'n, 'e>> for Valued<'n, 'e> {
    fn from(a: &'z Arg<'n, 'e>) -> Self {
        Valued {
            possible_vals: a.possible_vals,
            num_vals: a.num_vals,
            min_vals: a.min_vals,
            max_vals: a.max_vals,
            val_names: a.val_names,
            validator: a.validator.clone(),
            val_delim: a.val_delim,
            default_val: a.default_val,
        }
    }
}

impl<'n, 'e> Clone for Valued<'n, 'e> {
    fn clone(&self) -> Self {
        Valued {
            possible_vals: self.possible_vals,
            num_vals: self.num_vals,
            min_vals: self.min_vals,
            max_vals: self.max_vals,
            val_names: self.val_names,
            validator: self.validator.clone(),
            val_delim: self.val_delim,
            default_val: self.default_val,
        }
    }
}
