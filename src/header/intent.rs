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