pub trait OptionTuple<T>: Sized {
    fn f(self) -> Option<T>;
    fn s(self) -> Option<T>;
    fn t(self) -> Option<T>;
    fn fth(self) -> Option<T>;
}

impl<T> OptionTuple<T> for Option<(T, T)> {
    fn f(self) -> Option<T> {
        self.map(|x| x.0)
    }

    fn s(self) -> Option<T> {
        self.map(|x| x.1)
    }

    fn t(self) -> Option<T> {
        None
    }

    fn fth(self) -> Option<T> {
        None
    }
}

impl<T> OptionTuple<T> for Option<(T, T, T)> {
    fn f(self) -> Option<T> {
        self.map(|x| x.0)
    }

    fn s(self) -> Option<T> {
        self.map(|x| x.1)
    }

    fn t(self) -> Option<T> {
        self.map(|x| x.2)
    }

    fn fth(self) -> Option<T> {
        None
    }
}

macro_rules! replace_expr {
    ($_t:tt $sub:ty) => {
        $sub
    };
}

macro_rules! tuple_impls_homo {
    ( $( $name:ident )+ ) => {
        impl<T> OptionTuple<T> for Option<($(replace_expr!(($name) T),)+)>
        {
            fn f(self) -> Option<T> {
                self.map(|x| x.0)
            }

            fn s(self) -> Option<T> {
                self.map(|x| x.1)
            }

            fn t(self) -> Option<T> {
                self.map(|x| x.2)
            }

            fn fth(self) -> Option<T> {
                self.map(|x| x.3)
            }
        }
    };
}

tuple_impls_homo! { A B C D }
tuple_impls_homo! { A B C D E }
tuple_impls_homo! { A B C D E F }
tuple_impls_homo! { A B C D E F G }
tuple_impls_homo! { A B C D E F G H }
tuple_impls_homo! { A B C D E F G H I }
tuple_impls_homo! { A B C D E F G H I J }
tuple_impls_homo! { A B C D E F G H I J K }
tuple_impls_homo! { A B C D E F G H I J K L }

pub trait OptionTupleHetero<A, B, C, D>: Sized {
    fn f(self) -> Option<A>;
    fn s(self) -> Option<B>;
    fn t(self) -> Option<C>;
    fn fth(self) -> Option<D>;
}

impl<A, B> OptionTupleHetero<A, B, (), ()> for Option<(A, B)> {
    fn f(self) -> Option<A> {
        self.map(|x| x.0)
    }

    fn s(self) -> Option<B> {
        self.map(|x| x.1)
    }

    fn t(self) -> Option<()> {
        None
    }

    fn fth(self) -> Option<()> {
        None
    }
}

impl<A, B, C> OptionTupleHetero<A, B, C, ()> for Option<(A, B, C)> {
    fn f(self) -> Option<A> {
        self.map(|x| x.0)
    }

    fn s(self) -> Option<B> {
        self.map(|x| x.1)
    }

    fn t(self) -> Option<C> {
        self.map(|x| x.2)
    }

    fn fth(self) -> Option<()> {
        None
    }
}

macro_rules! tuple_impls_hetero {
    ( $( $name:ident )+ ) => {
        impl<$($name),+> OptionTupleHetero<A, B, C, D> for Option<($($name,)+)>
        {
            fn f(self) -> Option<A> {
                self.map(|x| x.0)
            }

            fn s(self) -> Option<B> {
                self.map(|x| x.1)
            }

            fn t(self) -> Option<C> {
                self.map(|x| x.2)
            }

            fn fth(self) -> Option<D> {
                self.map(|x| x.3)
            }
        }
    };
}

tuple_impls_hetero! { A B C D }
tuple_impls_hetero! { A B C D E }
tuple_impls_hetero! { A B C D E F }
tuple_impls_hetero! { A B C D E F G }
tuple_impls_hetero! { A B C D E F G H }
tuple_impls_hetero! { A B C D E F G H I }
tuple_impls_hetero! { A B C D E F G H I J }
tuple_impls_hetero! { A B C D E F G H I J K }
tuple_impls_hetero! { A B C D E F G H I J K L }
