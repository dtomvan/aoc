use std::iter::Sum;

pub trait OptionTuple<A, B, C, D>: Sized {
    fn f(self) -> Option<A>;
    fn s(self) -> Option<B>;
    fn t(self) -> Option<C>;
    fn fth(self) -> Option<D>;
}

impl<A, B> OptionTuple<A, B, (), ()> for Option<(A, B)> {
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

impl<A, B, C> OptionTuple<A, B, C, ()> for Option<(A, B, C)> {
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

macro_rules! tuple_impls{
    ( $( $name:ident )+ ) => {
        impl<$($name),+> OptionTuple<A, B, C, D> for Option<($($name,)+)>
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

tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }

pub trait TupleSum<T>: Sized {
    fn sum(self) -> T;
}

macro_rules! tuple_impls_sum {
    ( $($name:ty, $bind:ident),+ ) => {
        impl<T: Sum> TupleSum<T> for ($($name,)+)
        {
            fn sum(self) -> T {
                let ($($bind,)+) = self;
                return [$($bind,)+].into_iter().sum();
            }
        }
    };
}

tuple_impls_sum! { T,a }
tuple_impls_sum! { T,a, T,b }
tuple_impls_sum! { T,a, T,b, T,c }
tuple_impls_sum! { T,a, T,b, T,c, T,d }
tuple_impls_sum! { T,a, T,b, T,c, T,d, T,e }
tuple_impls_sum! { T,a, T,b, T,c, T,d, T,e, T,f }
tuple_impls_sum! { T,a, T,b, T,c, T,d, T,e, T,f, T,g }
tuple_impls_sum! { T,a, T,b, T,c, T,d, T,e, T,f, T,g, T,h }
tuple_impls_sum! { T,a, T,b, T,c, T,d, T,e, T,f, T,g, T,h, T,i }
tuple_impls_sum! { T,a, T,b, T,c, T,d, T,e, T,f, T,g, T,h, T,i, T,j }
tuple_impls_sum! { T,a, T,b, T,c, T,d, T,e, T,f, T,g, T,h, T,i, T,j, T,k }
tuple_impls_sum! { T,a, T,b, T,c, T,d, T,e, T,f, T,g, T,h, T,i, T,j, T,k, T,l }
