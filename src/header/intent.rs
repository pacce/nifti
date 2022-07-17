#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Packet {
    pub (super) parameters  : Parameters,
    pub (super) intent      : Intent,
}

pub (super) type Parameters = [f32; 3];

#[allow(non_camel_case_types)]
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

#[cfg(test)]
impl quickcheck::Arbitrary for Packet {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        let parameters  = i32::arbitrary(g);
        let parameters  = [parameters as f32; 3];

        let code        = i32::arbitrary(g);
        let intent      = match code {
            2       => Intent::CORREL,
            3       => Intent::TTEST,
            4       => Intent::FTEST,
            5       => Intent::ZSCORE,
            6       => Intent::CHISQ,
            7       => Intent::BETA,
            8       => Intent::BINOM,
            9       => Intent::GAMMA,
            10      => Intent::POISSON,
            11      => Intent::NORMAL,
            12      => Intent::FTEST_NONC,
            13      => Intent::CHISQ_NONC,
            14      => Intent::LOGISTIC,
            15      => Intent::LAPLACE,
            16      => Intent::UNIFORM,
            17      => Intent::TTEST_NONC,
            18      => Intent::WEIBULL,
            19      => Intent::CHI,
            20      => Intent::INVGAUSS,
            21      => Intent::EXTVAL,
            22      => Intent::PVAL,
            23      => Intent::LOGPVAL,
            24      => Intent::LOG10PVAL,
            1001    => Intent::ESTIMATE,
            1002    => Intent::LABEL,
            1003    => Intent::NEURONAME,
            1004    => Intent::GENMATRIX,
            1005    => Intent::SYMMATRIX,
            1006    => Intent::DISPVECT,
            1007    => Intent::VECTOR,
            1008    => Intent::POINTSET,
            1009    => Intent::TRIANGLE,
            1010    => Intent::QUATERNION,
            1011    => Intent::DIMLESS,
            2001    => Intent::TIME_SERIES,
            2002    => Intent::NODE_INDEX,
            2003    => Intent::RGB_VECTOR,
            2004    => Intent::RGBA_VECTOR,
            2005    => Intent::SHAPE,
            2006    => Intent::FSL_FNIRT_DISPLACEMENT_FIELD,
            2007    => Intent::FSL_CUBIC_SPLINE_COEFFICIENTS,
            2008    => Intent::FSL_DCT_COEFFICIENTS,
            2009    => Intent::FSL_QUADRATIC_SPLINE_COEFFICIENTS,
            2016    => Intent::FSL_TOPUP_CUBIC_SPLINE_COEFFICIENTS,
            2017    => Intent::FSL_TOPUP_QUADRATIC_SPLINE_COEFFICIENTS,
            2018    => Intent::FSL_TOPUP_FIELD,
            _       => Intent::NONE,
        };

        Self{parameters, intent}
    }
}
