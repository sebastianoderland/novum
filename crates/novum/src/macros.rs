macro_rules! if_feature {
    ($feature:literal, { $($items:item)* }) => {
        $(
            #[cfg(feature = $feature)]
            $items
        )*
    };
}

pub(crate) use if_feature;
