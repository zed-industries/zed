// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// This header is used to include the core portions of the calculator
// framework.  The comments that follow describe the main classes within
// the framework and how they interact.
//
// Calculator: A class which clients subclass to do actual work.
// It receives input and produces output which may go to many other
// Calculators connected in a directed acyclic graph.
//
// CalculatorGraph: A class which sets up a CalculatorGraphConfig and
// runs it.  This is the controller class which governs the top level
// behavior of the framework and how things are run.
//
// CalculatorNode: A class which keeps track of a single Calculator and
// framework level details that the client does not need to worry about
// (such as how to advertise that the Calculator is blocked or unblocked).
//
// InputStream: A class which holds the next value in an input stream
// for a Calculator to use and provides access to the stream header.
// It is the superclass of InputStreamImpl which holds implementation
// details for the framework.
//
// InputStreamImpl: All information for the input stream.
// A CalculatorNode and OutputStreamImpl has access to this information,
// but the Calculator does not.
//
// OutputStream: A class which gets the output packets from a Calculator
// and relays them to the next calculators or the framework.
//
// OutputStreamImpl: The framework level information for an OutputStream.
// A CalculatorNode has access to this information but the Calculator
// does not.
//
// CalculatorState: Data class to hold information the Calculator needs
// access to.  This data persists across multiple runs of the graph,
// whereas the Calculators will be destroyed and recreated.

#ifndef MEDIAPIPE_FRAMEWORK_CALCULATOR_FRAMEWORK_H_
#define MEDIAPIPE_FRAMEWORK_CALCULATOR_FRAMEWORK_H_

#include "mediapipe/framework/calculator_base.h"
#include "mediapipe/framework/calculator_context.h"
#include "mediapipe/framework/calculator_contract.h"
#include "mediapipe/framework/calculator_graph.h"
#include "mediapipe/framework/calculator_registry.h"
#include "mediapipe/framework/counter_factory.h"
#include "mediapipe/framework/input_stream.h"
#include "mediapipe/framework/output_side_packet.h"
#include "mediapipe/framework/output_stream.h"
#include "mediapipe/framework/packet.h"
#include "mediapipe/framework/packet_generator.h"
#include "mediapipe/framework/packet_generator_graph.h"
#include "mediapipe/framework/packet_set.h"
#include "mediapipe/framework/packet_type.h"
#include "mediapipe/framework/port.h"
#include "mediapipe/framework/status_handler.h"
#include "mediapipe/framework/subgraph.h"
#include "mediapipe/framework/timestamp.h"
#include "mediapipe/framework/tool/sink.h"
#include "mediapipe/framework/tool/status_util.h"
#include "mediapipe/framework/tool/validate.h"
#include "mediapipe/framework/tool/validate_name.h"
#include "mediapipe/framework/validated_graph_config.h"

#endif  // MEDIAPIPE_FRAMEWORK_CALCULATOR_FRAMEWORK_H_
