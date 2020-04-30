pub mod opt;

#[cfg(test)]
mod tests {
    use super::opt::Opt;
    #[test]
    fn it_works() {
        let opt = Opt::with(vec![
            "-a".to_string(),
            "hello".to_string(),
            "-k".to_string(),
            "-l".to_string(),
            "-100".to_string(),
            "-p".to_string(),
            "100".to_string(),
        ]);
        match opt.get_opt(Opt::one("-a")) {
            Some(v) => {
                assert_eq!(v.one(), "hello"); 
            },
            None => {
                panic!("-a should have hello.");
            }
        };
        match opt.get_opt(Opt::one("-k")) {
            Some(v) => {
                assert_eq!(v.one(), ""); 
            },
            None => {
                panic!("-k should have empty string");
            }
        };
        match opt.get_opt(Opt::one("-l")) {
            Some(v) => {
                assert_eq!(v.one(), "-100"); 
            },
            None => {
                panic!("-l should have -100");
            }
        };
        match opt.get_opt(Opt::one("-p")) {
            Some(v) => {
                assert_eq!(v.one(), "100"); 
            },
            None => {
                panic!("-p should have 100");
            }
        };
        assert_eq!(opt.get_opt_short_with_default("-k", "ano").one(),"ano");
    }
}
