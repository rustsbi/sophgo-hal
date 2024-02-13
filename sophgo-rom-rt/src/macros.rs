macro_rules! soc {
    ($($(#[$doc:meta])* pub struct $Ty: ident => $paddr: expr$(, $AsRefTy: ty)+;)+) => {
        $(
$(#[$doc])*
#[allow(non_camel_case_types)]
pub struct $Ty {
    _private: (),
}
$(
impl AsRef<$AsRefTy> for $Ty {
    #[inline(always)]
    fn as_ref(&self) -> &$AsRefTy {
        unsafe { &*($paddr as *const _) }
    }
}
)+
        )+
    };
}
