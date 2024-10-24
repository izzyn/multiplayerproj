macro_rules! connector {
    ($client:ident, $signal:expr, $function : ident) => {
        ::paste::paste! {
            $client._connect($signal, [<$function ____net____>]);
        }
    };
}

pub(crate) use connector;
