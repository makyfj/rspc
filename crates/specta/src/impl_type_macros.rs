#[macro_export]
macro_rules! impl_primitives {
    ($($i:ident)+) => {$(
        impl Type for $i {
            const NAME: &'static str = stringify!($i);

            fn inline(_: DefOpts, _: &[DataType]) -> DataType {
                DataType::Primitive(PrimitiveType::$i)
            }

            fn reference(_: DefOpts, _: &[DataType]) -> DataType {
                DataType::Primitive(PrimitiveType::$i)
            }

            fn definition(_: DefOpts) -> DataType {
                unreachable!()
            }
        }
    )+};
}

#[macro_export]
macro_rules! impl_tuple {
    (($($i:ident),*)) => {
        #[allow(non_snake_case)]
        impl<$($i: Type + 'static),*> Type for ($($i),*) {
            const NAME: &'static str = stringify!(($($i::NAME),*));

            fn inline(_opts: DefOpts, _generics: &[DataType]) -> DataType {
                $(let $i = $i::reference(
                    DefOpts {
                        parent_inline: _opts.parent_inline,
                        type_map: _opts.type_map
                    }, &[]
                );)*

                DataType::Tuple(TupleType {
                    name: stringify!(($($i),*)).to_string(),
                    fields: vec![$($i),*],
                    generics: vec![]
                })
            }

            fn reference(_opts: DefOpts, generics: &[DataType]) -> DataType {
                Self::inline(_opts, generics)
            }

            fn definition(_opts: DefOpts) -> DataType {
                unreachable!()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_containers {
    ($($container:ident)+) => {$(
        impl<T: Type> Type for $container<T> {
            const NAME: &'static str = stringify!($container);

            fn inline(opts: DefOpts, generics: &[DataType]) -> DataType {
                generics.get(0).cloned().unwrap_or(T::inline(
                    DefOpts {
                        parent_inline: false,
                        type_map: opts.type_map,
                    },
                    generics,
                ))
            }

            fn reference(opts: DefOpts, generics: &[DataType]) -> DataType {
                generics.get(0).cloned().unwrap_or(T::reference(
                    DefOpts {
                        parent_inline: false,
                        type_map: opts.type_map,
                    },
                    generics,
                ))
            }

            fn definition(_: DefOpts) -> DataType {
                unreachable!()
            }
        }
    )+}
}

#[macro_export]
macro_rules! impl_as {
    ($($ty:path as $tty:ident)+) => {$(
        impl Type for $ty {
            const NAME: &'static str = stringify!($ty);

            fn inline(opts: DefOpts, generics: &[DataType]) -> DataType {
                <$tty as Type>::inline(opts, generics)
            }

            fn reference(opts: DefOpts, generics: &[DataType]) -> DataType {
                <$tty as Type>::reference(opts, generics)
            }

            fn definition(opts: DefOpts) -> DataType {
                <$tty as Type>::definition(opts)
            }
        }
    )+};
}

#[macro_export]
macro_rules! impl_for_list {
    ($($ty:path as $name:expr)+) => {$(
        impl<T: Type> Type for $ty {
            const NAME: &'static str = $name;

            fn inline(opts: DefOpts, generics: &[DataType]) -> DataType {
                DataType::List(Box::new(generics.get(0).cloned().unwrap_or(T::inline(
                    DefOpts {
                        parent_inline: false,
                        type_map: opts.type_map,
                    },
                    generics,
                ))))
            }

            fn reference(opts: DefOpts, generics: &[DataType]) -> DataType {
                DataType::List(Box::new(generics.get(0).cloned().unwrap_or(T::reference(
                    DefOpts {
                        parent_inline: false,
                        type_map: opts.type_map,
                    },
                    generics,
                ))))
            }

            fn definition(_: DefOpts) -> DataType {
                unreachable!()
            }
        }
    )+};
}

#[macro_export]
macro_rules! impl_for_map {
    ($ty:path as $name:expr) => {
        impl<K: Type, V: Type> Type for $ty {
            const NAME: &'static str = $name;

            fn inline(defs: DefOpts, generics: &[DataType]) -> DataType {
                DataType::Record(Box::new((
                    generics.get(0).cloned().unwrap_or(<K as Type>::inline(
                        DefOpts {
                            parent_inline: false,
                            type_map: defs.type_map,
                        },
                        &[],
                    )),
                    generics.get(1).cloned().unwrap_or(<V as Type>::inline(
                        DefOpts {
                            parent_inline: false,
                            type_map: defs.type_map,
                        },
                        &[],
                    )),
                )))
            }

            fn reference(opts: DefOpts, generics: &[DataType]) -> DataType {
                DataType::Record(Box::new((
                    generics.get(0).cloned().unwrap_or(K::reference(
                        DefOpts {
                            parent_inline: false,
                            type_map: opts.type_map,
                        },
                        generics,
                    )),
                    generics.get(1).cloned().unwrap_or(V::reference(
                        DefOpts {
                            parent_inline: false,
                            type_map: opts.type_map,
                        },
                        generics,
                    )),
                )))
            }

            fn definition(_: DefOpts) -> DataType {
                unreachable!()
            }
        }
    };
}
