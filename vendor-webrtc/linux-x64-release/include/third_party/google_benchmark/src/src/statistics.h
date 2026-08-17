// Copyright 2016 Ismael Jimenez Martinez. All rights reserved.
// Copyright 2017 Roman Lebedev. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef STATISTICS_H_
#define STATISTICS_H_

#include <vector>

#include "benchmark/benchmark.h"

namespace benchmark {

// Return a vector containing the mean, median and standard deviation
// information (and any user-specified info) for the specified list of reports.
// If 'reports' contains less than two non-errored runs an empty vector is
// returned
BENCHMARK_EXPORT
std::vector<BenchmarkReporter::Run> ComputeStats(
    const std::vector<BenchmarkReporter::Run>& reports);

BENCHMARK_EXPORT
double StatisticsMean(const std::vector<double>& v);
BENCHMARK_EXPORT
double StatisticsMedian(const std::vector<double>& v);
BENCHMARK_EXPORT
double StatisticsStdDev(const std::vector<double>& v);
BENCHMARK_EXPORT
double StatisticsCV(const std::vector<double>& v);

}  // end namespace benchmark

#endif  // STATISTICS_H_
