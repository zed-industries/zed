// Copyright 2016 Google Inc.
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
////////////////////////////////////////////////////////////////////////////////

#ifndef UTIL_VARSETTER_H_
#define UTIL_VARSETTER_H_

//
// Use a VarSetter object to temporarily set an object of some sort to
// a particular value.  When the VarSetter object is destructed, the
// underlying object will revert to its former value.
//
// Sample code:
//
#if 0
{
  bool b = true;
  {
    VarSetter<bool> bool_setter(&b, false);
    // Now b == false.
  }
  // Now b == true again.
}
#endif

template <class C>
class VarSetter {
public:

  // Constructor that just sets the object to a fixed value
  VarSetter(C* object, const C& value) : object_(object), old_value_(*object) {
    *object = value;
  }

  ~VarSetter() { *object_ = old_value_; }

private:

  C*const object_;
  C old_value_;

  // Disallow
  VarSetter(const VarSetter&);
  VarSetter& operator=(const VarSetter&);

  // VarSetters always live on the stack
  static void* operator new (size_t);
  static void* operator new[](size_t);  // Redundant, no default ctor

  static void operator delete (void*);
  static void operator delete[](void*);
};

#endif  // UTIL_VARSETTER_H_
