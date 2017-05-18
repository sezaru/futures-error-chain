extern crate futures;

macro_rules! future_chain_processed {
    // Default values for `types`.
    (
        types {}
        $( $rest: tt )*
    ) => {
        future_chain_processed! {
            types {
                Error, ErrorKind, FutureExt, FutureChain;
            }
            $( $rest )*
        }
    };
    // With `futures::BoxFuture` wrapper.
    (
        types {
            $error_name:ident, $error_kind_name:ident,
            $future_ext_name:ident, $future_name:ident;
        }
        $( $rest: tt )*
    ) => {
        future_chain_processed! {
            types {
                $error_name, $error_kind_name,
                $future_ext_name;
            }
            $( $rest )*
        }
        /// Convenient wrapper around `futures::BoxFuture`.
        #[allow(unused)]
        pub type $future_name<T> = ::futures::BoxFuture<T, $error_name>;

        #[allow(unused)]
        pub use futures::Future;
    };
    // Without `futures::BoxFuture` wrapper.
    (
        types {
            $error_name:ident, $error_kind_name:ident,
            $future_ext_name:ident;
        }

    ) => {
        pub trait $future_ext_name<T> {
            fn chain_err<F, E>(self, callback: F) -> ::futures::BoxFuture<T, $error_name>
                where F: FnOnce() -> E + 'static + Send,
                      E: Into<$error_kind_name>;
        }

        impl<F> $future_ext_name<F::Item> for F
            where F: ::futures::future::Future + 'static + Send,
                  F::Error: ::std::error::Error + Send + 'static,
                  F::Item: Send,
        {
            fn chain_err<C, E>(self, callback: C) -> ::futures::BoxFuture<F::Item, $error_name>
                where C: FnOnce() -> E + 'static + Send,
                      E: Into<$error_kind_name>,
            {
                Box::new(self.then(|r| r.chain_err(callback)))
            }
        }
    };
}

/// Internal macro used for reordering of the fields.
#[doc(hidden)]
macro_rules! future_chain_processing {
    (
        ({})
        types $content:tt
        $( $tail:tt )*
    ) => {
        future_chain_processing! {
            ($content)
            $($tail)*
        }
    };
    ( ($a:tt) ) => {
        future_chain_processed! {
            types $a
        }
    };
}

/// This macro is used for handling of duplicated and out-of-order fields. For
/// the exact rules, see `future_chain_processed`.
#[macro_export]
macro_rules! future_chain {
    ( $( $block_name:ident { $( $block_content:tt )* } )* ) => {
        future_chain_processing! {
            ({})
            $($block_name { $( $block_content )* })*
        }
    };
}
