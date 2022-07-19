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
        Datatype
        , Dimension
        , Header
        , intent::{Intent, Packet, Parameters}
        , slice::Code
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

pub (super) fn datatype<W: Write>(i: Datatype) -> impl SerializeFn<W> {
    be_i16(i as i16)
}

pub (super) fn bitpix<W: Write>(i: i16) -> impl SerializeFn<W> {
    be_i16(i)
}

pub (super) fn slice_start<W: Write>(i: i16) -> impl SerializeFn<W> {
    be_i16(i)
}

pub (super) fn pixdim<W: Write>(xs: [f32; 8]) -> impl SerializeFn<W> {
    many_ref(xs, be_f32)
}

pub (super) fn vox_offset<W: Write>(i: f32) -> impl SerializeFn<W> {
    be_f32(i)
}

pub (super) fn scl_slope<W: Write>(i: f32) -> impl SerializeFn<W> {
    be_f32(i)
}

pub (super) fn scl_inter<W: Write>(i: f32) -> impl SerializeFn<W> {
    be_f32(i)
}

pub (super) fn slice_end<W: Write>(i: i16) -> impl SerializeFn<W> {
    be_i16(i)
}

pub (super) fn slice_code<W: Write>(i: Code) -> impl SerializeFn<W> {
    be_i8(i as i8)
}

pub (super) fn xyzt_units<W: Write>(i: i8) -> impl SerializeFn<W> {
    be_i8(i)
}

pub (super) fn cal_max<W: Write>(i: f32) -> impl SerializeFn<W> {
    be_f32(i)
}

pub (super) fn cal_min<W: Write>(i: f32) -> impl SerializeFn<W> {
    be_f32(i)
}

pub (super) fn slice_duration<W: Write>(i: f32) -> impl SerializeFn<W> {
    be_f32(i)
}

pub fn header<W: Write>(header: Header) -> impl SerializeFn<W> {
    let slice = header.slice;
    tuple(
        ( sizeof_hdr(header.size)
        , data_type()
        , db_name()
        , extents()
        , session_error()
        , regular()
        , dimension(header.dimension)
        , packet(header.intent)
        , datatype(header.datatype)
        , bitpix(header.bitpix)
        , slice_start(slice.start)
        , pixdim(header.pixdim)
        , vox_offset(header.offset)
        , scl_slope(0.0f32)
        , scl_inter(0.0f32)
        , slice_end(slice.end)
        , slice_code(slice.code)
        , xyzt_units(0i8)
        , cal_max(0.0f32)
        , cal_min(0.0f32)
        , slice_duration(slice.duration)
        )
    )
}
