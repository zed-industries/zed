! Platform-specific declarations of SIMD math functions for Fortran. -*- f90 -*-
!   Copyright (C) 2019-2020 Free Software Foundation, Inc.
!   This file is part of the GNU C Library.
!
!   The GNU C Library is free software; you can redistribute it and/or
!   modify it under the terms of the GNU Lesser General Public
!   License as published by the Free Software Foundation; either
!   version 2.1 of the License, or (at your option) any later version.
!
!   The GNU C Library is distributed in the hope that it will be useful,
!   but WITHOUT ANY WARRANTY; without even the implied warranty of
!   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
!   Lesser General Public License for more details.
!
!   You should have received a copy of the GNU Lesser General Public
!   License along with the GNU C Library; if not, see
!   <https://www.gnu.org/licenses/>.

!GCC$ builtin (cos) attributes simd (notinbranch) if('x86_64')
!GCC$ builtin (cosf) attributes simd (notinbranch) if('x86_64')
!GCC$ builtin (sin) attributes simd (notinbranch) if('x86_64')
!GCC$ builtin (sinf) attributes simd (notinbranch) if('x86_64')
!GCC$ builtin (sincos) attributes simd (notinbranch) if('x86_64')
!GCC$ builtin (sincosf) attributes simd (notinbranch) if('x86_64')
!GCC$ builtin (log) attributes simd (notinbranch) if('x86_64')
!GCC$ builtin (logf) attributes simd (notinbranch) if('x86_64')
!GCC$ builtin (exp) attributes simd (notinbranch) if('x86_64')
!GCC$ builtin (expf) attributes simd (notinbranch) if('x86_64')
!GCC$ builtin (pow) attributes simd (notinbranch) if('x86_64')
!GCC$ builtin (powf) attributes simd (notinbranch) if('x86_64')

!GCC$ builtin (cos) attributes simd (notinbranch) if('x32')
!GCC$ builtin (cosf) attributes simd (notinbranch) if('x32')
!GCC$ builtin (sin) attributes simd (notinbranch) if('x32')
!GCC$ builtin (sinf) attributes simd (notinbranch) if('x32')
!GCC$ builtin (sincos) attributes simd (notinbranch) if('x32')
!GCC$ builtin (sincosf) attributes simd (notinbranch) if('x32')
!GCC$ builtin (log) attributes simd (notinbranch) if('x32')
!GCC$ builtin (logf) attributes simd (notinbranch) if('x32')
!GCC$ builtin (exp) attributes simd (notinbranch) if('x32')
!GCC$ builtin (expf) attributes simd (notinbranch) if('x32')
!GCC$ builtin (pow) attributes simd (notinbranch) if('x32')
!GCC$ builtin (powf) attributes simd (notinbranch) if('x32')
