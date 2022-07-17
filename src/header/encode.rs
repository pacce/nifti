use {
    cookie_factory::{
        bytes::{
            be_f32
            , be_i8
            , be_i16
            , be_i32
        }
        , multi::many_ref
        , sequence::tuple
        , SerializeFn
    }
    , std::io::Write
    , super::{
        Dimension
        , Header
        , intent::{Intent, Packet, Parameters}
    }
};

pub (super) fn sizeof_hdr<W: Write>(i: i32) -> impl SerializeFn<W> {
    be_i32(i)
}

fn data_type<W: Write>() -> impl SerializeFn<W> {
    many_ref([0i8; 10], be_i8)
}

fn db_name<W: Write>() -> impl SerializeFn<W> {
    many_ref([0i8; 18], be_i8)
}

fn extents<W: Write>() -> impl SerializeFn<W> {
    be_i32(0i32)
}

fn session_error<W: Write>() -> impl SerializeFn<W> {
    be_i16(0i16)
}

fn regular<W: Write>() -> impl SerializeFn<W> {
    be_i8(0i8)
}

pub (super) fn dim_info<W: Write>(i: i8) -> impl SerializeFn<W> {
    be_i8(i)
}

pub (super) fn dim<W: Write>(xs: [i16; 8]) -> impl SerializeFn<W> {
    many_ref(xs, be_i16)
}

pub (super) fn dimension<W: Write>(i: Dimension) -> impl SerializeFn<W> {
    tuple(
        ( dim_info(i.information)
        , dim(i.values)
        )
    )
}

fn parameters<W: Write>(xs: Parameters) -> impl SerializeFn<W> {
    many_ref(xs, be_f32)
}

fn intent<W: Write>(i: Intent) -> impl SerializeFn<W> {
    be_i16(i as i16)
}

pub (super) fn packet<W: Write>(i: Packet) -> impl SerializeFn<W> {
    tuple(
        ( parameters(i.parameters)
        , intent(i.intent)
        )
    )
}

pub fn header<W: Write>(i: Header) -> impl SerializeFn<W> {
    tuple(
        ( sizeof_hdr(i.size)
        , data_type()
        , db_name()
        , extents()
        , session_error()
        , regular()
        , dimension(i.dimension)
        , packet(i.intent)
        )
    )
}
