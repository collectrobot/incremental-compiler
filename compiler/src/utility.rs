// credits to https://stackoverflow.com/a/63904992 for this macro
#[macro_export]
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[macro_export]
macro_rules! idstr {
    ($s:expr) => {
        crate::types::IdString::new($s.to_owned())
    }
}

#[macro_export]
macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };

    {} => {
        std::collections::HashMap::new()
    }
);

#[macro_export]
macro_rules! set(
    { $($key:expr),+ } => {
        {
            let mut m = std::collections::HashSet::new();
            $(
                m.insert($key);
            )+
            m
        }
     };

    {} => {
        std::collections::HashSet::new()
    }
);