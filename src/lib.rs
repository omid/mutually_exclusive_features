#![no_std]

/**
This macro allows you to specify that certain features are mutually exclusive.
It generates any combination of the selected features, by the combination formula:
`n!/(2!*(n - 2)!)`, where n is the number of features.
For example, if you have 5 features, you will get 5!/(3!*2!) which is 10 combinations
or for 10 features, you will get 10!/(8!*2!) which is 45 combinations

To use it, simply call it with the features you want to be mutually exclusive:
```
use mutually_exclusive_features::none_or_one_of;
none_or_one_of!("feature1", "feature2", "feature3");
```

Which will generate the following code:
```ignore
#[cfg(all(feature="feature1", feature="feature2"))]
compile_error!("The `feature1` and `feature2` features are mutually exclusive and cannot be enabled at the same time!");

#[cfg(all(feature="feature1", feature="feature3"))]
compile_error!("The `feature1` and `feature3` features are mutually exclusive and cannot be enabled at the same time!");

#[cfg(all(feature="feature2", feature="feature3"))]
compile_error!("The `feature2` and `feature3` features are mutually exclusive and cannot be enabled at the same time!");
```
**/
#[macro_export]
macro_rules! none_or_one_of {
    // root call
    ($($F:literal),+ $(,)?) => {
        $crate::none_or_one_of!(@recurs: [params: $($F,)*]);
    };

    // exactly two
    (
        @recurs:
        [params: $F1:literal, $F2:literal,]
    ) => {
        #[cfg(all(feature=$F1, feature=$F2))]
        compile_error!(concat!("The `", $F1, "` and `", $F2, "` features are mutually exclusive and cannot be enabled at the same time!"));
    };

    // more than two
    (
        @recurs:
        [params: $F1:literal, $($FS:literal,)+]
    ) => {
        $($crate::none_or_one_of!(@recurs: [params: $F1, $FS,]);)*
        $crate::none_or_one_of!(@recurs: [params: $($FS,)*]);
    };

    // ignore cases
    () => {};
    (@recurs: [params: $F1:literal,]) => {};
}

/**
This is exactly like `none_or_one_of` except that it requires exactly one of the features to be selected.

To use it, simply call it with the features you want to be mutually exclusive:
```
use mutually_exclusive_features::exactly_one_of;
exactly_one_of!("feature1", "feature2", "feature3");
```

Which will generate the following code:
```ignore
#[cfg(all(feature="feature1", feature="feature2"))]
compile_error!("The `feature1` and `feature2` features are mutually exclusive and cannot be enabled at the same time!");

#[cfg(all(feature="feature1", feature="feature3"))]
compile_error!("The `feature1` and `feature3` features are mutually exclusive and cannot be enabled at the same time!");

#[cfg(all(feature="feature2", feature="feature3"))]
compile_error!("The `feature2` and `feature3` features are mutually exclusive and cannot be enabled at the same time!");

#[cfg(not(any(feature="feature1", feature="feature2", feature="feature3")))]
compile_error!("You must enable exactly one of `feature1`, `feature2`, `feature3` features!");
```
**/
#[macro_export]
macro_rules! exactly_one_of {
    ($($F:literal),+ $(,)?) => {
        $crate::none_or_one_of!(@recurs: [params: $($F,)*]);

        #[cfg(not(any($(feature=$F),*)))]
        compile_error!(concat!("You must enable exactly one of ", $crate::exactly_one_of!(@comma_sep: $($F,)*), " features!"));
    };

    (@comma_sep: $last:literal $(,)?) => { concat!("`", $last, "`") };
    (@comma_sep: $head:literal, $($rest:literal),+ $(,)?) => {
        concat!("`", $head, "`, ", $crate::exactly_one_of!(@comma_sep: $($rest),+))
    };
}
