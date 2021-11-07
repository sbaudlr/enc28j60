macro_rules! register {
    ($REGISTER:ident, $reset_value:expr, $uxx:ty, {
        $(#[$($attr:tt)*] $bitfield:ident @ $range:expr,)+
    }) => {
        #[derive(Clone, Copy)]
        pub(crate) struct $REGISTER<MODE> {
            bits: $uxx,
            _mode: ::core::marker::PhantomData<MODE>,
        }

        impl $REGISTER<crate::traits::Mask> {
            #[allow(dead_code)]
            pub(crate) fn mask() -> $REGISTER<crate::traits::Mask> {
                $REGISTER { bits: 0, _mode: ::core::marker::PhantomData }
            }

            $(
                #[allow(dead_code)]
                pub(crate) fn $bitfield(&self) -> $uxx {
                    use crate::traits::OffsetSize;

                    let size = $range.size();
                    let offset = $range.offset();
                    ((1 << size) - 1) << offset
                }
            )+
        }

        impl ::core::default::Default for $REGISTER<crate::traits::W> {
            fn default() -> Self {
                $REGISTER { bits: $reset_value, _mode: ::core::marker::PhantomData }
            }
        }

        #[allow(non_snake_case)]
        #[allow(dead_code)]
        pub(crate) fn $REGISTER(bits: $uxx) -> $REGISTER<crate::traits::R> {
            $REGISTER { bits, _mode: ::core::marker::PhantomData }
        }

        impl $REGISTER<crate::traits::R> {
            #[allow(dead_code)]
            pub(crate) fn modify(self) -> $REGISTER<crate::traits::W> {
                $REGISTER { bits: self.bits, _mode: ::core::marker::PhantomData }
            }

            $(
                #[$($attr)*]
                #[allow(dead_code)]
                pub(crate) fn $bitfield(&self) -> $uxx {
                    use crate::traits::OffsetSize;

                    let offset = $range.offset();
                    let size = $range.size();
                    let mask = (1 << size) - 1;

                    (self.bits >> offset) & mask
                }
            )+
        }

        impl $REGISTER<crate::traits::W> {
            #[allow(dead_code)]
            pub(crate) fn bits(self) -> $uxx {
                self.bits
            }

            $(
                #[$($attr)*]
                #[allow(dead_code)]
                pub(crate) fn $bitfield(&mut self, mut bits: $uxx) -> &mut Self {
                    use crate::traits::OffsetSize;

                    let offset = $range.offset();
                    let size = $range.size();
                    let mask = (1 << size) - 1;

                    debug_assert!(bits <= mask);
                    bits &= mask;

                    self.bits &= !(mask << offset);
                    self.bits |= bits << offset;

                    self
                }
            )+
        }
    }
}

/// Poor man's specialization
macro_rules! typeid {
    ($type_parameter:ident == $concrete_type:ident) => {
        ::core::any::TypeId::of::<$type_parameter>() == ::core::any::TypeId::of::<$concrete_type>()
    };
    ($type_parameter:ident != $concrete_type:ident) => {
        !typeid!($type_parameter == $concrete_type)
    };
}
