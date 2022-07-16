pub (crate) use decode::packet;

#[derive(Clone, Copy, Debug)]
pub struct Packet {
    parameters  : Parameters,
    intent      : Intent,
}

type Parameters = [f32; 3];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Intent {
    NONE                                    =    0,
    CORREL                                  =    2,
    TTEST                                   =    3,
    FTEST                                   =    4,
    ZSCORE                                  =    5,
    CHISQ                                   =    6,
    BETA                                    =    7,
    BINOM                                   =    8,
    GAMMA                                   =    9,
    POISSON                                 =   10,
    NORMAL                                  =   11,
    FTEST_NONC                              =   12,
    CHISQ_NONC                              =   13,
    LOGISTIC                                =   14,
    LAPLACE                                 =   15,
    UNIFORM                                 =   16,
    TTEST_NONC                              =   17,
    WEIBULL                                 =   18,
    CHI                                     =   19,
    INVGAUSS                                =   20,
    EXTVAL                                  =   21,
    PVAL                                    =   22,
    LOGPVAL                                 =   23,
    LOG10PVAL                               =   24,
    ESTIMATE                                = 1001,
    LABEL                                   = 1002,
    NEURONAME                               = 1003,
    GENMATRIX                               = 1004,
    SYMMATRIX                               = 1005,
    DISPVECT                                = 1006,
    VECTOR                                  = 1007,
    POINTSET                                = 1008,
    TRIANGLE                                = 1009,
    QUATERNION                              = 1010,
    DIMLESS                                 = 1011,
    TIME_SERIES                             = 2001,
    NODE_INDEX                              = 2002,
    RGB_VECTOR                              = 2003,
    RGBA_VECTOR                             = 2004,
    SHAPE                                   = 2005,
    FSL_FNIRT_DISPLACEMENT_FIELD            = 2006,
    FSL_CUBIC_SPLINE_COEFFICIENTS           = 2007,
    FSL_DCT_COEFFICIENTS                    = 2008,
    FSL_QUADRATIC_SPLINE_COEFFICIENTS       = 2009,
    FSL_TOPUP_CUBIC_SPLINE_COEFFICIENTS     = 2016,
    FSL_TOPUP_QUADRATIC_SPLINE_COEFFICIENTS = 2017,
    FSL_TOPUP_FIELD                         = 2018,
}


mod decode {
    use {
        nom::{
            error::ParseError
            , multi::fill
            , number::complete::{be_f32, be_i16}
            , IResult
        }
        , super::{Intent, Packet, Parameters}
    };

    pub (crate) type Bytes = [u8];

    fn parameters<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Parameters, E> {
        let mut xs = [0f32; 3];
        let (i, _) = fill(be_f32, &mut xs)(i)?;

        Ok((i, xs))
    }

    fn intent<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Intent, E> {
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

    pub fn packet<'a, E: ParseError<&'a Bytes>>(i: &'a Bytes) -> IResult<&'a Bytes, Packet, E> {
        let (i, parameters) = parameters(i)?;
        let (i, intent)     = intent(i)?;

        Ok((i, Packet{parameters, intent}))
    }
}
