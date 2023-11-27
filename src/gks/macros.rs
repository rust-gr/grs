macro_rules! impl_each {
    (($($types:ident),+) $body:tt) => {
        $(
            impl $types $body
        )+
    };
}

pub(crate) use impl_each;
