use std::collections::HashMap;
use std::env;

/// # Opt
/// main struct.
///
/// ### normal usage:
/// ```
/// use rust_command_opt::opt::Opt;
///
/// fn main() {
///     let opt = Opt::init();
///     if let Some(value) = opt.get_opt_short("-a") {
///         println!("{}", value.one());
///     }
/// }
///
/// ```
pub struct Opt {
    pub opts: HashMap<String, OptValue>,
}

impl Opt {
    /// Init with command args.
    pub fn init() -> Opt {
        let mut args: Vec<String> = env::args().collect();
        args.remove(0);
        Opt::with(args)
    }

    /// Init with you custom args.
    /// (Usually used in test)
    pub fn with(args: Vec<String>) -> Opt {
        let mut map: HashMap<String, OptValue> = HashMap::default();
        let mut last_opt = "".to_string();
        for v in args {
            if let Some(s) = v.get(0..1) {
                if s == "-" {
                    match v.parse::<f32>() {
                        Ok(_) => {
                            match map.get_mut(&last_opt) {
                                Some(find) => match find {
                                    OptValue::One(one) => {
                                        if one == "" {
                                            map.insert(last_opt.clone(), OptValue::One(v));
                                        } else {
                                            let mut vecv = vec![one.clone()];
                                            vecv.push(v.clone());
                                            map.insert(last_opt.clone(), OptValue::Many(vecv));
                                        }
                                    }
                                    OptValue::Many(many) => {
                                        many.push(v);
                                    }
                                },
                                None => {
                                    map.insert(last_opt.clone(), OptValue::One(v));
                                }
                            };
                        }
                        Err(_) => {
                            last_opt = v.clone();
                            map.insert(v, OptValue::One("".to_string()));
                        }
                    }
                } else {
                    match map.get_mut(&last_opt) {
                        Some(find) => {
                            match find {
                                OptValue::One(one) => {
                                    if one == "" {
                                        map.insert(last_opt.clone(), OptValue::One(v));
                                    } else {
                                        let mut vecv = vec![one.clone()];
                                        vecv.push(v.clone());
                                        map.insert(last_opt.clone(), OptValue::Many(vecv));
                                    }
                                }
                                OptValue::Many(many) => {
                                    many.push(v);
                                }
                            };
                        }
                        None => {
                            map.insert(last_opt.clone(), OptValue::One(v));
                        }
                    }
                }
            }
        }
        Opt { opts: map }
    }

    /// Basic get opt method.
    /// ### usage:
    /// ```
    /// use rust_command_opt::opt::Opt;
    ///
    /// let opt = Opt::init();
    /// if let Some(value) = opt.get_opt(Opt::one("-a")) {
    ///     println!("{}", value.one());
    /// }
    ///
    /// ```
    pub fn get_opt(&self, op_name: OptOption) -> Option<OptValue> {
        match op_name {
            OptOption::One(one) => match self.opts.get(&one.to_string()) {
                Some(real) => Some(real.clone()),
                None => None,
            },
            OptOption::ShortLong(short, long) => {
                if let Some(find_short) = self.opts.get(short) {
                    Some(find_short.clone())
                } else {
                    if let Some(find_long) = self.opts.get(long) {
                        Some(find_long.clone())
                    } else {
                        None
                    }
                }
            }
        }
    }

    /// Only use short option, and have a default value.
    /// ### usage
    /// ```
    /// use rust_command_opt::opt::Opt;
    ///
    /// let opt = Opt::init();
    /// let value = opt.get_opt_short_with_default("-a", "default_value").one();
    /// ```
    pub fn get_opt_short_with_default(&self, name: &str, default: &str) -> OptValue {
        if let Some(v) = self.get_opt(OptOption::One(name)) {
            if v.one() == "" {
                OptValue::One(default.to_string())
            } else {
                v
            }
        } else {
            OptValue::One(default.to_string())
        }
    }

    /// Use short and full option, and have a default value.
    /// ### usage
    /// ```
    /// use rust_command_opt::opt::Opt;
    ///
    /// let opt = Opt::init();
    /// let value = opt.get_opt_normal_with_default("-a", "--all", "default_value").one();
    /// ```
    pub fn get_opt_normal_with_default(&self, short: &str, long: &str, default: &str) -> OptValue {
        if let Some(v) = self.get_opt(Opt::normal(short, long)) {
            if v.one() == "" {
                OptValue::One(default.to_string())
            } else {
                v
            }
        } else {
            OptValue::One(default.to_string())
        }
    }

    /// Only use short option.
    /// ```
    /// use rust_command_opt::opt::Opt;
    ///
    /// let opt = Opt::init();
    /// if let Some(value) = opt.get_opt_short("-a") {
    ///     println!("{}", value.one());
    /// }
    /// ```
    pub fn get_opt_short(&self, name: &str) -> Option<OptValue> {
        self.get_opt(OptOption::One(name))
    }

    /// Only use short option.
    /// ```
    /// use rust_command_opt::opt::Opt;
    ///
    /// let opt = Opt::init();
    /// if let Some(value) = opt.get_opt_normal("-a", "--all") {
    ///     println!("{}", value.one());
    /// }
    /// ```
    pub fn get_opt_normal(&self, short: &str, long: &str) -> Option<OptValue> {
        self.get_opt(Opt::normal(short, long))
    }

    /// Make short option
    pub fn one(short: &str) -> OptOption {
        OptOption::One(short)
    }

    /// Make short and full option
    pub fn normal<'a>(short: &'a str, long: &'a str) -> OptOption<'a> {
        OptOption::ShortLong(short, long)
    }
}

/// Contains value
///
/// ### Many
/// Many value in one option.
///
/// **like:**
/// `you_program -p a b c d`
///
/// ### One
/// Only one value in one option.
///
/// **like:**
/// `you_program -p a`
#[derive(Clone)]
pub enum OptValue {
    Many(Vec<String>),
    One(String),
}

impl OptValue {
    /// Get one value in `OptValue`.
    ///
    /// if only one value in `OptValue`, you get it.
    /// or you will get the first.
    pub fn one(&self) -> String {
        match self {
            OptValue::One(one) => one.to_string(),
            OptValue::Many(many) => many[0].to_string(),
        }
    }

    /// Get many value in `OptValue`.
    ///
    /// if only one value in `OptValue`, you will also get a vector.
    /// or you will get the vector.
    pub fn many(&self) -> Vec<String> {
        match self {
            OptValue::One(one) => vec![one.to_string()],
            OptValue::Many(many) => many.clone(),
        }
    }
}

pub enum OptOption<'a> {
    One(&'a str),
    ShortLong(&'a str, &'a str),
}
