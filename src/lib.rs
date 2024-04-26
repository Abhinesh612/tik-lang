#[macro_export]
macro_rules! define_ast {
    ($type:ident/$visitor:ident, $( $choice:ident/$struct:ident/$visit:ident { $( $prop_name:ident: $prop_type:ty ),* $(,)? } )*  ) => {
        pub enum $type {
            $(
                $choice($struct),
                )*
        }

        impl $type {
            pub fn accept<T>(&self, visitor: &dyn $visitor<T>) -> Result<T, TikError> {
                match self {
                    $(
                        $type::$choice(x) => x.accept(visitor),
                        )*
                }
            }
        }

        $(
            pub struct $struct {
                $(
                    pub $prop_name: $prop_type,
                    )*
            }
         )*

            pub trait $visitor<T> {
                $(
                    fn $visit(&self, expr: &$struct) -> Result<T, TikError>;
                 )*
            }

        $(
            impl $struct {
                pub fn accept<T>(&self, visitor: &dyn $visitor<T>) -> Result<T, TikError> {
                    visitor.$visit(self)
                }
            }
         )*
    }
}
