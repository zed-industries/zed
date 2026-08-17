use std::fmt;

use crate::stats::Distribution;

#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd, Deserialize, Serialize, Debug)]
pub enum Statistic {
    Mean,
    Median,
    MedianAbsDev,
    Slope,
    StdDev,
    Typical,
}

impl fmt::Display for Statistic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Statistic::Mean => f.pad("mean"),
            Statistic::Median => f.pad("median"),
            Statistic::MedianAbsDev => f.pad("MAD"),
            Statistic::Slope => f.pad("slope"),
            Statistic::StdDev => f.pad("SD"),
            Statistic::Typical => f.pad("typical"),
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct ConfidenceInterval {
    pub confidence_level: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Estimate {
    /// The confidence interval for this estimate
    pub confidence_interval: ConfidenceInterval,
    ///
    pub point_estimate: f64,
    /// The standard error of this estimate
    pub standard_error: f64,
}

pub fn build_estimates(
    distributions: &Distributions,
    points: &PointEstimates,
    cl: f64,
) -> Estimates {
    let to_estimate = |point_estimate, distribution: &Distribution<f64>| {
        let (lb, ub) = distribution.confidence_interval(cl);

        Estimate {
            confidence_interval: ConfidenceInterval {
                confidence_level: cl,
                lower_bound: lb,
                upper_bound: ub,
            },
            point_estimate,
            standard_error: distribution.std_dev(None),
        }
    };

    Estimates {
        mean: to_estimate(points.mean, &distributions.mean),
        median: to_estimate(points.median, &distributions.median),
        median_abs_dev: to_estimate(points.median_abs_dev, &distributions.median_abs_dev),
        slope: None,
        std_dev: to_estimate(points.std_dev, &distributions.std_dev),
    }
}

pub fn build_change_estimates(
    distributions: &ChangeDistributions,
    points: &ChangePointEstimates,
    cl: f64,
) -> ChangeEstimates {
    let to_estimate = |point_estimate, distribution: &Distribution<f64>| {
        let (lb, ub) = distribution.confidence_interval(cl);

        Estimate {
            confidence_interval: ConfidenceInterval {
                confidence_level: cl,
                lower_bound: lb,
                upper_bound: ub,
            },
            point_estimate,
            standard_error: distribution.std_dev(None),
        }
    };

    ChangeEstimates {
        mean: to_estimate(points.mean, &distributions.mean),
        median: to_estimate(points.median, &distributions.median),
    }
}

pub struct PointEstimates {
    pub mean: f64,
    pub median: f64,
    pub median_abs_dev: f64,
    pub std_dev: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Estimates {
    pub mean: Estimate,
    pub median: Estimate,
    pub median_abs_dev: Estimate,
    pub slope: Option<Estimate>,
    pub std_dev: Estimate,
}
impl Estimates {
    pub fn typical(&self) -> &Estimate {
        self.slope.as_ref().unwrap_or(&self.mean)
    }
    pub fn get(&self, stat: Statistic) -> Option<&Estimate> {
        match stat {
            Statistic::Mean => Some(&self.mean),
            Statistic::Median => Some(&self.median),
            Statistic::MedianAbsDev => Some(&self.median_abs_dev),
            Statistic::Slope => self.slope.as_ref(),
            Statistic::StdDev => Some(&self.std_dev),
            Statistic::Typical => Some(self.typical()),
        }
    }
}

pub struct Distributions {
    pub mean: Distribution<f64>,
    pub median: Distribution<f64>,
    pub median_abs_dev: Distribution<f64>,
    pub slope: Option<Distribution<f64>>,
    pub std_dev: Distribution<f64>,
}
impl Distributions {
    pub fn typical(&self) -> &Distribution<f64> {
        self.slope.as_ref().unwrap_or(&self.mean)
    }
    pub fn get(&self, stat: Statistic) -> Option<&Distribution<f64>> {
        match stat {
            Statistic::Mean => Some(&self.mean),
            Statistic::Median => Some(&self.median),
            Statistic::MedianAbsDev => Some(&self.median_abs_dev),
            Statistic::Slope => self.slope.as_ref(),
            Statistic::StdDev => Some(&self.std_dev),
            Statistic::Typical => Some(self.typical()),
        }
    }
}

pub struct ChangePointEstimates {
    pub mean: f64,
    pub median: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChangeEstimates {
    pub mean: Estimate,
    pub median: Estimate,
}
impl ChangeEstimates {
    pub fn get(&self, stat: Statistic) -> &Estimate {
        match stat {
            Statistic::Mean => &self.mean,
            Statistic::Median => &self.median,
            _ => panic!("Unexpected statistic"),
        }
    }
}

pub struct ChangeDistributions {
    pub mean: Distribution<f64>,
    pub median: Distribution<f64>,
}
impl ChangeDistributions {
    pub fn get(&self, stat: Statistic) -> &Distribution<f64> {
        match stat {
            Statistic::Mean => &self.mean,
            Statistic::Median => &self.median,
            _ => panic!("Unexpected statistic"),
        }
    }
}
