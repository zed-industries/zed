/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#ifndef GRPC_RB_CHANNEL_ARGS_H_
#define GRPC_RB_CHANNEL_ARGS_H_

#include <ruby/ruby.h>

#include <grpc/grpc.h>

/* Converts a hash object containing channel args to a channel args instance.
 *
 * This func ALLOCs args->args.  The caller is responsible for freeing it.  If
 * a ruby error is raised during processing of the hash values, the func takes
 * care to deallocate any memory allocated so far, and propagate the error.
 *
 * @param src_hash A ruby hash
 * @param dst the grpc_channel_args that the hash entries will be added to.
 */
void grpc_rb_hash_convert_to_channel_args(VALUE src_hash,
                                          grpc_channel_args* dst);

/* Destroys inner fields of args (does not deallocate the args pointer itself)
 */
void grpc_rb_channel_args_destroy(grpc_channel_args* args);

#endif /* GRPC_RB_CHANNEL_ARGS_H_ */
