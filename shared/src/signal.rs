macro_rules! connector {
    ($client:ident, $signal:expr, $function : ident) => {
        ::paste::paste! {
            $client.connect($signal, [<$function ____net____>]);
        }
    };
}

pub fn connect()

pub(crate) use connector;
