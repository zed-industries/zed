/*
  Interface for the f2c translation of fftpack as found on http://www.netlib.org/fftpack/
  
   FFTPACK license:

   http://www.cisl.ucar.edu/css/software/fftpack5/ftpk.html

   Copyright (c) 2004 the University Corporation for Atmospheric
   Research ("UCAR"). All rights reserved. Developed by NCAR's
   Computational and Information Systems Laboratory, UCAR,
   www.cisl.ucar.edu.

   Redistribution and use of the Software in source and binary forms,
   with or without modification, is permitted provided that the
   following conditions are met:

   - Neither the names of NCAR's Computational and Information Systems
   Laboratory, the University Corporation for Atmospheric Research,
   nor the names of its sponsors or contributors may be used to
   endorse or promote products derived from this Software without
   specific prior written permission.  

   - Redistributions of source code must retain the above copyright
   notices, this list of conditions, and the disclaimer below.

   - Redistributions in binary form must reproduce the above copyright
   notice, this list of conditions, and the disclaimer below in the
   documentation and/or other materials provided with the
   distribution.

   THIS SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
   EXPRESS OR IMPLIED, INCLUDING, BUT NOT LIMITED TO THE WARRANTIES OF
   MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
   NONINFRINGEMENT. IN NO EVENT SHALL THE CONTRIBUTORS OR COPYRIGHT
   HOLDERS BE LIABLE FOR ANY CLAIM, INDIRECT, INCIDENTAL, SPECIAL,
   EXEMPLARY, OR CONSEQUENTIAL DAMAGES OR OTHER LIABILITY, WHETHER IN AN
   ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
   CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS WITH THE
   SOFTWARE.

   ChangeLog:
   2011/10/02: this is my first release of this file.
*/

#ifndef FFTPACK_H
#define FFTPACK_H

#ifdef __cplusplus
extern "C" {
#endif

// just define FFTPACK_DOUBLE_PRECISION if you want to build it as a double precision fft

#ifndef FFTPACK_DOUBLE_PRECISION
  typedef float fftpack_real;
  typedef int   fftpack_int;
#else
  typedef double fftpack_real;
  typedef int    fftpack_int;
#endif

  void cffti(fftpack_int n, fftpack_real *wsave);

  void cfftf(fftpack_int n, fftpack_real *c, fftpack_real *wsave);

  void cfftb(fftpack_int n, fftpack_real *c, fftpack_real *wsave);

  void rffti(fftpack_int n, fftpack_real *wsave);
  void rfftf(fftpack_int n, fftpack_real *r, fftpack_real *wsave);
  void rfftb(fftpack_int n, fftpack_real *r, fftpack_real *wsave);

  void cosqi(fftpack_int n, fftpack_real *wsave);
  void cosqf(fftpack_int n, fftpack_real *x, fftpack_real *wsave);
  void cosqb(fftpack_int n, fftpack_real *x, fftpack_real *wsave);

  void costi(fftpack_int n, fftpack_real *wsave);
  void cost(fftpack_int n, fftpack_real *x, fftpack_real *wsave);

  void sinqi(fftpack_int n, fftpack_real *wsave);
  void sinqb(fftpack_int n, fftpack_real *x, fftpack_real *wsave);
  void sinqf(fftpack_int n, fftpack_real *x, fftpack_real *wsave);

  void sinti(fftpack_int n, fftpack_real *wsave);
  void sint(fftpack_int n, fftpack_real *x, fftpack_real *wsave);

#ifdef __cplusplus
}
#endif

#endif /* FFTPACK_H */

/*

                      FFTPACK

* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *

                  version 4  april 1985

     a package of fortran subprograms for the fast fourier
      transform of periodic and other symmetric sequences

                         by

                  paul n swarztrauber

  national center for atmospheric research  boulder,colorado 80307

   which is sponsored by the national science foundation

* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *


this package consists of programs which perform fast fourier
transforms for both complex and real periodic sequences and
certain other symmetric sequences that are listed below.

1.   rffti     initialize  rfftf and rfftb
2.   rfftf     forward transform of a real periodic sequence
3.   rfftb     backward transform of a real coefficient array

4.   ezffti    initialize ezfftf and ezfftb
5.   ezfftf    a simplified real periodic forward transform
6.   ezfftb    a simplified real periodic backward transform

7.   sinti     initialize sint
8.   sint      sine transform of a real odd sequence

9.   costi     initialize cost
10.  cost      cosine transform of a real even sequence

11.  sinqi     initialize sinqf and sinqb
12.  sinqf     forward sine transform with odd wave numbers
13.  sinqb     unnormalized inverse of sinqf

14.  cosqi     initialize cosqf and cosqb
15.  cosqf     forward cosine transform with odd wave numbers
16.  cosqb     unnormalized inverse of cosqf

17.  cffti     initialize cfftf and cfftb
18.  cfftf     forward transform of a complex periodic sequence
19.  cfftb     unnormalized inverse of cfftf


******************************************************************

subroutine rffti(n,wsave)

  ****************************************************************

subroutine rffti initializes the array wsave which is used in
both rfftf and rfftb. the prime factorization of n together with
a tabulation of the trigonometric functions are computed and
stored in wsave.

input parameter

n       the length of the sequence to be transformed.

output parameter

wsave   a work array which must be dimensioned at least 2*n+15.
        the same work array can be used for both rfftf and rfftb
        as long as n remains unchanged. different wsave arrays
        are required for different values of n. the contents of
        wsave must not be changed between calls of rfftf or rfftb.

******************************************************************

subroutine rfftf(n,r,wsave)

******************************************************************

subroutine rfftf computes the fourier coefficients of a real
perodic sequence (fourier analysis). the transform is defined
below at output parameter r.

input parameters

n       the length of the array r to be transformed.  the method
        is most efficient when n is a product of small primes.
        n may change so long as different work arrays are provided

r       a real array of length n which contains the sequence
        to be transformed

wsave   a work array which must be dimensioned at least 2*n+15.
        in the program that calls rfftf. the wsave array must be
        initialized by calling subroutine rffti(n,wsave) and a
        different wsave array must be used for each different
        value of n. this initialization does not have to be
        repeated so long as n remains unchanged thus subsequent
        transforms can be obtained faster than the first.
        the same wsave array can be used by rfftf and rfftb.


output parameters

r       r(1) = the sum from i=1 to i=n of r(i)

        if n is even set l =n/2   , if n is odd set l = (n+1)/2

          then for k = 2,...,l

             r(2*k-2) = the sum from i = 1 to i = n of

                  r(i)*cos((k-1)*(i-1)*2*pi/n)

             r(2*k-1) = the sum from i = 1 to i = n of

                 -r(i)*sin((k-1)*(i-1)*2*pi/n)

        if n is even

             r(n) = the sum from i = 1 to i = n of

                  (-1)**(i-1)*r(i)

 *****  note
             this transform is unnormalized since a call of rfftf
             followed by a call of rfftb will multiply the input
             sequence by n.

wsave   contains results which must not be destroyed between
        calls of rfftf or rfftb.


******************************************************************

subroutine rfftb(n,r,wsave)

******************************************************************

subroutine rfftb computes the real perodic sequence from its
fourier coefficients (fourier synthesis). the transform is defined
below at output parameter r.

input parameters

n       the length of the array r to be transformed.  the method
        is most efficient when n is a product of small primes.
        n may change so long as different work arrays are provided

r       a real array of length n which contains the sequence
        to be transformed

wsave   a work array which must be dimensioned at least 2*n+15.
        in the program that calls rfftb. the wsave array must be
        initialized by calling subroutine rffti(n,wsave) and a
        different wsave array must be used for each different
        value of n. this initialization does not have to be
        repeated so long as n remains unchanged thus subsequent
        transforms can be obtained faster than the first.
        the same wsave array can be used by rfftf and rfftb.


output parameters

r       for n even and for i = 1,...,n

             r(i) = r(1)+(-1)**(i-1)*r(n)

                  plus the sum from k=2 to k=n/2 of

                   2.*r(2*k-2)*cos((k-1)*(i-1)*2*pi/n)

                  -2.*r(2*k-1)*sin((k-1)*(i-1)*2*pi/n)

        for n odd and for i = 1,...,n

             r(i) = r(1) plus the sum from k=2 to k=(n+1)/2 of

                  2.*r(2*k-2)*cos((k-1)*(i-1)*2*pi/n)

                 -2.*r(2*k-1)*sin((k-1)*(i-1)*2*pi/n)

 *****  note
             this transform is unnormalized since a call of rfftf
             followed by a call of rfftb will multiply the input
             sequence by n.

wsave   contains results which must not be destroyed between
        calls of rfftb or rfftf.

******************************************************************

subroutine sinti(n,wsave)

******************************************************************

subroutine sinti initializes the array wsave which is used in
subroutine sint. the prime factorization of n together with
a tabulation of the trigonometric functions are computed and
stored in wsave.

input parameter

n       the length of the sequence to be transformed.  the method
        is most efficient when n+1 is a product of small primes.

output parameter

wsave   a work array with at least int(2.5*n+15) locations.
        different wsave arrays are required for different values
        of n. the contents of wsave must not be changed between
        calls of sint.

******************************************************************

subroutine sint(n,x,wsave)

******************************************************************

subroutine sint computes the discrete fourier sine transform
of an odd sequence x(i). the transform is defined below at
output parameter x.

sint is the unnormalized inverse of itself since a call of sint
followed by another call of sint will multiply the input sequence
x by 2*(n+1).

the array wsave which is used by subroutine sint must be
initialized by calling subroutine sinti(n,wsave).

input parameters

n       the length of the sequence to be transformed.  the method
        is most efficient when n+1 is the product of small primes.

x       an array which contains the sequence to be transformed


wsave   a work array with dimension at least int(2.5*n+15)
        in the program that calls sint. the wsave array must be
        initialized by calling subroutine sinti(n,wsave) and a
        different wsave array must be used for each different
        value of n. this initialization does not have to be
        repeated so long as n remains unchanged thus subsequent
        transforms can be obtained faster than the first.

output parameters

x       for i=1,...,n

             x(i)= the sum from k=1 to k=n

                  2*x(k)*sin(k*i*pi/(n+1))

             a call of sint followed by another call of
             sint will multiply the sequence x by 2*(n+1).
             hence sint is the unnormalized inverse
             of itself.

wsave   contains initialization calculations which must not be
        destroyed between calls of sint.

******************************************************************

subroutine costi(n,wsave)

******************************************************************

subroutine costi initializes the array wsave which is used in
subroutine cost. the prime factorization of n together with
a tabulation of the trigonometric functions are computed and
stored in wsave.

input parameter

n       the length of the sequence to be transformed.  the method
        is most efficient when n-1 is a product of small primes.

output parameter

wsave   a work array which must be dimensioned at least 3*n+15.
        different wsave arrays are required for different values
        of n. the contents of wsave must not be changed between
        calls of cost.

******************************************************************

subroutine cost(n,x,wsave)

******************************************************************

subroutine cost computes the discrete fourier cosine transform
of an even sequence x(i). the transform is defined below at output
parameter x.

cost is the unnormalized inverse of itself since a call of cost
followed by another call of cost will multiply the input sequence
x by 2*(n-1). the transform is defined below at output parameter x

the array wsave which is used by subroutine cost must be
initialized by calling subroutine costi(n,wsave).

input parameters

n       the length of the sequence x. n must be greater than 1.
        the method is most efficient when n-1 is a product of
        small primes.

x       an array which contains the sequence to be transformed

wsave   a work array which must be dimensioned at least 3*n+15
        in the program that calls cost. the wsave array must be
        initialized by calling subroutine costi(n,wsave) and a
        different wsave array must be used for each different
        value of n. this initialization does not have to be
        repeated so long as n remains unchanged thus subsequent
        transforms can be obtained faster than the first.

output parameters

x       for i=1,...,n

            x(i) = x(1)+(-1)**(i-1)*x(n)

             + the sum from k=2 to k=n-1

                 2*x(k)*cos((k-1)*(i-1)*pi/(n-1))

             a call of cost followed by another call of
             cost will multiply the sequence x by 2*(n-1)
             hence cost is the unnormalized inverse
             of itself.

wsave   contains initialization calculations which must not be
        destroyed between calls of cost.

******************************************************************

subroutine sinqi(n,wsave)

******************************************************************

subroutine sinqi initializes the array wsave which is used in
both sinqf and sinqb. the prime factorization of n together with
a tabulation of the trigonometric functions are computed and
stored in wsave.

input parameter

n       the length of the sequence to be transformed. the method
        is most efficient when n is a product of small primes.

output parameter

wsave   a work array which must be dimensioned at least 3*n+15.
        the same work array can be used for both sinqf and sinqb
        as long as n remains unchanged. different wsave arrays
        are required for different values of n. the contents of
        wsave must not be changed between calls of sinqf or sinqb.

******************************************************************

subroutine sinqf(n,x,wsave)

******************************************************************

subroutine sinqf computes the fast fourier transform of quarter
wave data. that is , sinqf computes the coefficients in a sine
series representation with only odd wave numbers. the transform
is defined below at output parameter x.

sinqb is the unnormalized inverse of sinqf since a call of sinqf
followed by a call of sinqb will multiply the input sequence x
by 4*n.

the array wsave which is used by subroutine sinqf must be
initialized by calling subroutine sinqi(n,wsave).


input parameters

n       the length of the array x to be transformed.  the method
        is most efficient when n is a product of small primes.

x       an array which contains the sequence to be transformed

wsave   a work array which must be dimensioned at least 3*n+15.
        in the program that calls sinqf. the wsave array must be
        initialized by calling subroutine sinqi(n,wsave) and a
        different wsave array must be used for each different
        value of n. this initialization does not have to be
        repeated so long as n remains unchanged thus subsequent
        transforms can be obtained faster than the first.

output parameters

x       for i=1,...,n

             x(i) = (-1)**(i-1)*x(n)

                + the sum from k=1 to k=n-1 of

                2*x(k)*sin((2*i-1)*k*pi/(2*n))

             a call of sinqf followed by a call of
             sinqb will multiply the sequence x by 4*n.
             therefore sinqb is the unnormalized inverse
             of sinqf.

wsave   contains initialization calculations which must not
        be destroyed between calls of sinqf or sinqb.

******************************************************************

subroutine sinqb(n,x,wsave)

******************************************************************

subroutine sinqb computes the fast fourier transform of quarter
wave data. that is , sinqb computes a sequence from its
representation in terms of a sine series with odd wave numbers.
the transform is defined below at output parameter x.

sinqf is the unnormalized inverse of sinqb since a call of sinqb
followed by a call of sinqf will multiply the input sequence x
by 4*n.

the array wsave which is used by subroutine sinqb must be
initialized by calling subroutine sinqi(n,wsave).


input parameters

n       the length of the array x to be transformed.  the method
        is most efficient when n is a product of small primes.

x       an array which contains the sequence to be transformed

wsave   a work array which must be dimensioned at least 3*n+15.
        in the program that calls sinqb. the wsave array must be
        initialized by calling subroutine sinqi(n,wsave) and a
        different wsave array must be used for each different
        value of n. this initialization does not have to be
        repeated so long as n remains unchanged thus subsequent
        transforms can be obtained faster than the first.

output parameters

x       for i=1,...,n

             x(i)= the sum from k=1 to k=n of

               4*x(k)*sin((2k-1)*i*pi/(2*n))

             a call of sinqb followed by a call of
             sinqf will multiply the sequence x by 4*n.
             therefore sinqf is the unnormalized inverse
             of sinqb.

wsave   contains initialization calculations which must not
        be destroyed between calls of sinqb or sinqf.

******************************************************************

subroutine cosqi(n,wsave)

******************************************************************

subroutine cosqi initializes the array wsave which is used in
both cosqf and cosqb. the prime factorization of n together with
a tabulation of the trigonometric functions are computed and
stored in wsave.

input parameter

n       the length of the array to be transformed.  the method
        is most efficient when n is a product of small primes.

output parameter

wsave   a work array which must be dimensioned at least 3*n+15.
        the same work array can be used for both cosqf and cosqb
        as long as n remains unchanged. different wsave arrays
        are required for different values of n. the contents of
        wsave must not be changed between calls of cosqf or cosqb.

******************************************************************

subroutine cosqf(n,x,wsave)

******************************************************************

subroutine cosqf computes the fast fourier transform of quarter
wave data. that is , cosqf computes the coefficients in a cosine
series representation with only odd wave numbers. the transform
is defined below at output parameter x

cosqf is the unnormalized inverse of cosqb since a call of cosqf
followed by a call of cosqb will multiply the input sequence x
by 4*n.

the array wsave which is used by subroutine cosqf must be
initialized by calling subroutine cosqi(n,wsave).


input parameters

n       the length of the array x to be transformed.  the method
        is most efficient when n is a product of small primes.

x       an array which contains the sequence to be transformed

wsave   a work array which must be dimensioned at least 3*n+15
        in the program that calls cosqf. the wsave array must be
        initialized by calling subroutine cosqi(n,wsave) and a
        different wsave array must be used for each different
        value of n. this initialization does not have to be
        repeated so long as n remains unchanged thus subsequent
        transforms can be obtained faster than the first.

output parameters

x       for i=1,...,n

             x(i) = x(1) plus the sum from k=2 to k=n of

                2*x(k)*cos((2*i-1)*(k-1)*pi/(2*n))

             a call of cosqf followed by a call of
             cosqb will multiply the sequence x by 4*n.
             therefore cosqb is the unnormalized inverse
             of cosqf.

wsave   contains initialization calculations which must not
        be destroyed between calls of cosqf or cosqb.

******************************************************************

subroutine cosqb(n,x,wsave)

******************************************************************

subroutine cosqb computes the fast fourier transform of quarter
wave data. that is , cosqb computes a sequence from its
representation in terms of a cosine series with odd wave numbers.
the transform is defined below at output parameter x.

cosqb is the unnormalized inverse of cosqf since a call of cosqb
followed by a call of cosqf will multiply the input sequence x
by 4*n.

the array wsave which is used by subroutine cosqb must be
initialized by calling subroutine cosqi(n,wsave).


input parameters

n       the length of the array x to be transformed.  the method
        is most efficient when n is a product of small primes.

x       an array which contains the sequence to be transformed

wsave   a work array that must be dimensioned at least 3*n+15
        in the program that calls cosqb. the wsave array must be
        initialized by calling subroutine cosqi(n,wsave) and a
        different wsave array must be used for each different
        value of n. this initialization does not have to be
        repeated so long as n remains unchanged thus subsequent
        transforms can be obtained faster than the first.

output parameters

x       for i=1,...,n

             x(i)= the sum from k=1 to k=n of

               4*x(k)*cos((2*k-1)*(i-1)*pi/(2*n))

             a call of cosqb followed by a call of
             cosqf will multiply the sequence x by 4*n.
             therefore cosqf is the unnormalized inverse
             of cosqb.

wsave   contains initialization calculations which must not
        be destroyed between calls of cosqb or cosqf.

******************************************************************

subroutine cffti(n,wsave)

******************************************************************

subroutine cffti initializes the array wsave which is used in
both cfftf and cfftb. the prime factorization of n together with
a tabulation of the trigonometric functions are computed and
stored in wsave.

input parameter

n       the length of the sequence to be transformed

output parameter

wsave   a work array which must be dimensioned at least 4*n+15
        the same work array can be used for both cfftf and cfftb
        as long as n remains unchanged. different wsave arrays
        are required for different values of n. the contents of
        wsave must not be changed between calls of cfftf or cfftb.

******************************************************************

subroutine cfftf(n,c,wsave)

******************************************************************

subroutine cfftf computes the forward complex discrete fourier
transform (the fourier analysis). equivalently , cfftf computes
the fourier coefficients of a complex periodic sequence.
the transform is defined below at output parameter c.

the transform is not normalized. to obtain a normalized transform
the output must be divided by n. otherwise a call of cfftf
followed by a call of cfftb will multiply the sequence by n.

the array wsave which is used by subroutine cfftf must be
initialized by calling subroutine cffti(n,wsave).

input parameters


n      the length of the complex sequence c. the method is
       more efficient when n is the product of small primes. n

c      a complex array of length n which contains the sequence

wsave   a real work array which must be dimensioned at least 4n+15
        in the program that calls cfftf. the wsave array must be
        initialized by calling subroutine cffti(n,wsave) and a
        different wsave array must be used for each different
        value of n. this initialization does not have to be
        repeated so long as n remains unchanged thus subsequent
        transforms can be obtained faster than the first.
        the same wsave array can be used by cfftf and cfftb.

output parameters

c      for j=1,...,n

           c(j)=the sum from k=1,...,n of

                 c(k)*exp(-i*(j-1)*(k-1)*2*pi/n)

                       where i=sqrt(-1)

wsave   contains initialization calculations which must not be
        destroyed between calls of subroutine cfftf or cfftb

******************************************************************

subroutine cfftb(n,c,wsave)

******************************************************************

subroutine cfftb computes the backward complex discrete fourier
transform (the fourier synthesis). equivalently , cfftb computes
a complex periodic sequence from its fourier coefficients.
the transform is defined below at output parameter c.

a call of cfftf followed by a call of cfftb will multiply the
sequence by n.

the array wsave which is used by subroutine cfftb must be
initialized by calling subroutine cffti(n,wsave).

input parameters


n      the length of the complex sequence c. the method is
       more efficient when n is the product of small primes.

c      a complex array of length n which contains the sequence

wsave   a real work array which must be dimensioned at least 4n+15
        in the program that calls cfftb. the wsave array must be
        initialized by calling subroutine cffti(n,wsave) and a
        different wsave array must be used for each different
        value of n. this initialization does not have to be
        repeated so long as n remains unchanged thus subsequent
        transforms can be obtained faster than the first.
        the same wsave array can be used by cfftf and cfftb.

output parameters

c      for j=1,...,n

           c(j)=the sum from k=1,...,n of

                 c(k)*exp(i*(j-1)*(k-1)*2*pi/n)

                       where i=sqrt(-1)

wsave   contains initialization calculations which must not be
        destroyed between calls of subroutine cfftf or cfftb

*/
