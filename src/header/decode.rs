use {
    nom::{
        error::ParseError
        , multi::{count, fill}
        , number::complete::*
        , IResult
    }
    , super::*
    , super::intent::{Intent, Packet, Parameters}
};

pub (crate) type Bytes = [u8];

pub (super) fn sizeof_hdr<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, i32, E> {
    be_i32(i)
}

fn data_type<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, (), E> {
    let (i, _) = count(be_i8, 10)(i)?;

    Ok((i, ()))
}

fn db_name<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, (), E> {
    let (i, _) = count(be_i8, 18)(i)?;

    Ok((i, ()))
}

fn extents<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, (), E> {
    let (i, _) = be_i32(i)?;

    Ok((i, ()))
}

fn session_error<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, (), E> {
    let (i, _) = be_i16(i)?;

    Ok((i, ()))
}

fn regular<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, (), E> {
    let (i, _) = be_i8(i)?;

    Ok((i, ()))
}

pub (super) fn dim_info<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, i8, E> {
    be_i8(i)
}

pub (super) fn dim<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, [i16; 8], E> {
    let mut xs = [0i16; 8];
    let (i, _) = fill(be_i16, &mut xs)(i)?;

    Ok((i, xs))
}

pub (super) fn dimension<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Dimension, E> {
    let (i, information)    = dim_info(i)?;
    let (i, values)         = dim(i)?;

    Ok((i, Dimension{information, values}))
}

fn parameters<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Parameters, E> {
    let mut xs = [0f32; 3];
    let (i, _) = fill(be_f32, &mut xs)(i)?;

    Ok((i, xs))
}

pub (super) fn intent<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Intent, E> {
    let (i, code) = be_i16(i)?;

    match code {
        2       => Ok((i, Intent::CORREL)),
        3       => Ok((i, Intent::TTEST)),
        4       => Ok((i, Intent::FTEST)),
        5       => Ok((i, Intent::ZSCORE)),
        6       => Ok((i, Intent::CHISQ)),
        7       => Ok((i, Intent::BETA)),
        8       => Ok((i, Intent::BINOM)),
        9       => Ok((i, Intent::GAMMA)),
        10      => Ok((i, Intent::POISSON)),
        11      => Ok((i, Intent::NORMAL)),
        12      => Ok((i, Intent::FTEST_NONC)),
        13      => Ok((i, Intent::CHISQ_NONC)),
        14      => Ok((i, Intent::LOGISTIC)),
        15      => Ok((i, Intent::LAPLACE)),
        16      => Ok((i, Intent::UNIFORM)),
        17      => Ok((i, Intent::TTEST_NONC)),
        18      => Ok((i, Intent::WEIBULL)),
        19      => Ok((i, Intent::CHI)),
        20      => Ok((i, Intent::INVGAUSS)),
        21      => Ok((i, Intent::EXTVAL)),
        22      => Ok((i, Intent::PVAL)),
        23      => Ok((i, Intent::LOGPVAL)),
        24      => Ok((i, Intent::LOG10PVAL)),
        1001    => Ok((i, Intent::ESTIMATE)),
        1002    => Ok((i, Intent::LABEL)),
        1003    => Ok((i, Intent::NEURONAME)),
        1004    => Ok((i, Intent::GENMATRIX)),
        1005    => Ok((i, Intent::SYMMATRIX)),
        1006    => Ok((i, Intent::DISPVECT)),
        1007    => Ok((i, Intent::VECTOR)),
        1008    => Ok((i, Intent::POINTSET)),
        1009    => Ok((i, Intent::TRIANGLE)),
        1010    => Ok((i, Intent::QUATERNION)),
        1011    => Ok((i, Intent::DIMLESS)),
        2001    => Ok((i, Intent::TIME_SERIES)),
        2002    => Ok((i, Intent::NODE_INDEX)),
        2003    => Ok((i, Intent::RGB_VECTOR)),
        2004    => Ok((i, Intent::RGBA_VECTOR)),
        2005    => Ok((i, Intent::SHAPE)),
        2006    => Ok((i, Intent::FSL_FNIRT_DISPLACEMENT_FIELD)),
        2007    => Ok((i, Intent::FSL_CUBIC_SPLINE_COEFFICIENTS)),
        2008    => Ok((i, Intent::FSL_DCT_COEFFICIENTS)),
        2009    => Ok((i, Intent::FSL_QUADRATIC_SPLINE_COEFFICIENTS)),
        2016    => Ok((i, Intent::FSL_TOPUP_CUBIC_SPLINE_COEFFICIENTS)),
        2017    => Ok((i, Intent::FSL_TOPUP_QUADRATIC_SPLINE_COEFFICIENTS)),
        2018    => Ok((i, Intent::FSL_TOPUP_FIELD)),
        _       => Ok((i, Intent::NONE)),
    }
}

pub (super) fn packet<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Packet, E> {
    let (i, parameters) = parameters(i)?;
    let (i, intent)     = intent(i)?;

    Ok((i, Packet{parameters, intent}))
}

pub (super) fn datatype<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Datatype, E> {
    let (i, code) = be_i16(i)?;
    match code {
        1       => Ok((i, Datatype::BINARY)),
        2       => Ok((i, Datatype::UINT8)),
        4       => Ok((i, Datatype::INT16)),
        8       => Ok((i, Datatype::INT32)),
        16      => Ok((i, Datatype::FLOAT32)),
        32      => Ok((i, Datatype::COMPLEX64)),
        64      => Ok((i, Datatype::FLOAT64)),
        128     => Ok((i, Datatype::RGB24)),
        255     => Ok((i, Datatype::ALL)),
        256     => Ok((i, Datatype::INT8)),
        512     => Ok((i, Datatype::UINT16)),
        768     => Ok((i, Datatype::UINT32)),
        1024    => Ok((i, Datatype::INT64)),
        1280    => Ok((i, Datatype::UINT64)),
        1536    => Ok((i, Datatype::FLOAT128)),
        1792    => Ok((i, Datatype::COMPLEX128)),
        2048    => Ok((i, Datatype::COMPLEX256)),
        2304    => Ok((i, Datatype::RGBA32)),
        _       => Ok((i, Datatype::NONE)),
    }
}

pub (super) fn bitpix<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, i16, E> {
    be_i16(i)
}

pub (super) fn slice_start<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, i16, E> {
    be_i16(i)
}

pub fn header<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Header, E> {
    let (i, size)       = sizeof_hdr(i)?;
    let (i, _)          = data_type(i)?;
    let (i, _)          = db_name(i)?;
    let (i, _)          = extents(i)?;
    let (i, _)          = session_error(i)?;
    let (i, _)          = regular(i)?;

    let (i, dimension)  = dimension(i)?;
    let (i, intent)     = packet(i)?;

    let (i, datatype)   = datatype(i)?;
    let (i, bitpix)     = bitpix(i)?;

    let header  = Header {
        size
        , dimension
        , intent
        , datatype
        , bitpix
    };
    Ok((i, header))
}
