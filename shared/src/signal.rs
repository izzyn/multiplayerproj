#[macro_export]
macro_rules! connect {
    ($signal:expr,$client:ident, $function : ident) => {
        $crate::paste::paste! {
            $client._connect($signal, [<$function ____net____>]);
        }
    };
}
