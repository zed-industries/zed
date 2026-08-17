/*
 * Generic functions for ULP error estimation.
 *
 * Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
 * See https://llvm.org/LICENSE.txt for license information.
 * SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
 */

/* For each different math function type,
   T(x) should add a different suffix to x.
   RT(x) should add a return type specific suffix to x. */

#ifdef NEW_RT
#undef NEW_RT

# if USE_MPFR
static int RT(ulpscale_mpfr) (mpfr_t x, int t)
{
  /* TODO: pow of 2 cases.  */
  if (mpfr_regular_p (x))
    {
      mpfr_exp_t e = mpfr_get_exp (x) - RT(prec);
      if (e < RT(emin))
	e = RT(emin) - 1;
      if (e > RT(emax) - RT(prec))
	e = RT(emax) - RT(prec);
      return e;
    }
  if (mpfr_zero_p (x))
    return RT(emin) - 1;
  if (mpfr_inf_p (x))
    return RT(emax) - RT(prec);
  /* NaN.  */
  return 0;
}
# endif

/* Difference between exact result and closest real number that
   gets rounded to got, i.e. error before rounding, for a correctly
   rounded result the difference is 0.  */
static double RT(ulperr) (RT(float) got, const struct RT(ret) * p, int r)
{
  RT(float) want = p->y;
  RT(float) d;
  double e;

  if (RT(asuint) (got) == RT(asuint) (want))
    return 0.0;
  if (signbit (got) != signbit (want))
    /* May have false positives with NaN.  */
    //return isnan(got) && isnan(want) ? 0 : INFINITY;
    return INFINITY;
  if (!isfinite (want) || !isfinite (got))
    {
      if (isnan (got) != isnan (want))
	return INFINITY;
      if (isnan (want))
	return 0;
      if (isinf (got))
	{
	  got = RT(copysign) (RT(halfinf), got);
	  want *= 0.5f;
	}
      if (isinf (want))
	{
	  want = RT(copysign) (RT(halfinf), want);
	  got *= 0.5f;
	}
    }
  if (r == FE_TONEAREST)
    {
      // TODO: incorrect when got vs want cross a powof2 boundary
      /* error = got > want
	      ? got - want - tail ulp - 0.5 ulp
	      : got - want - tail ulp + 0.5 ulp;  */
      d = got - want;
      e = d > 0 ? -p->tail - 0.5 : -p->tail + 0.5;
    }
  else
    {
      if ((r == FE_DOWNWARD && got < want) || (r == FE_UPWARD && got > want)
	  || (r == FE_TOWARDZERO && fabs (got) < fabs (want)))
	got = RT(nextafter) (got, want);
      d = got - want;
      e = -p->tail;
    }
  return RT(scalbn) (d, -p->ulpexp) + e;
}

static int RT(isok) (RT(float) ygot, int exgot, RT(float) ywant, int exwant,
		      int exmay)
{
  return RT(asuint) (ygot) == RT(asuint) (ywant)
	 && ((exgot ^ exwant) & ~exmay) == 0;
}

static int RT(isok_nofenv) (RT(float) ygot, RT(float) ywant)
{
  return RT(asuint) (ygot) == RT(asuint) (ywant);
}
#endif

static inline void T(call_fenv) (const struct fun *f, struct T(args) a, int r,
				  RT(float) * y, int *ex)
{
  if (r != FE_TONEAREST)
    fesetround (r);
  feclearexcept (FE_ALL_EXCEPT);
  *y = T(call) (f, a);
  *ex = fetestexcept (FE_ALL_EXCEPT);
  if (r != FE_TONEAREST)
    fesetround (FE_TONEAREST);
}

static inline void T(call_nofenv) (const struct fun *f, struct T(args) a,
				    int r, RT(float) * y, int *ex)
{
  *y = T(call) (f, a);
  *ex = 0;
}

static inline int T(call_long_fenv) (const struct fun *f, struct T(args) a,
				      int r, struct RT(ret) * p,
				      RT(float) ygot, int exgot)
{
  if (r != FE_TONEAREST)
    fesetround (r);
  feclearexcept (FE_ALL_EXCEPT);
  volatile struct T(args) va = a; // TODO: barrier
  a = va;
  RT(double) yl = T(call_long) (f, a);
  p->y = (RT(float)) yl;
  volatile RT(float) vy = p->y; // TODO: barrier
  (void) vy;
  p->ex = fetestexcept (FE_ALL_EXCEPT);
  if (r != FE_TONEAREST)
    fesetround (FE_TONEAREST);
  p->ex_may = FE_INEXACT;
  if (RT(isok) (ygot, exgot, p->y, p->ex, p->ex_may))
    return 1;
  p->ulpexp = RT(ulpscale) (p->y);
  if (isinf (p->y))
    p->tail = RT(lscalbn) (yl - (RT(double)) 2 * RT(halfinf), -p->ulpexp);
  else
    p->tail = RT(lscalbn) (yl - p->y, -p->ulpexp);
  if (RT(fabs) (p->y) < RT(min_normal))
    {
      /* TODO: subnormal result is treated as undeflow even if it's
	 exact since call_long may not raise inexact correctly.  */
      if (p->y != 0 || (p->ex & FE_INEXACT))
	p->ex |= FE_UNDERFLOW | FE_INEXACT;
    }
  return 0;
}
static inline int T(call_long_nofenv) (const struct fun *f, struct T(args) a,
					int r, struct RT(ret) * p,
					RT(float) ygot, int exgot)
{
  RT(double) yl = T(call_long) (f, a);
  p->y = (RT(float)) yl;
  if (RT(isok_nofenv) (ygot, p->y))
    return 1;
  p->ulpexp = RT(ulpscale) (p->y);
  if (isinf (p->y))
    p->tail = RT(lscalbn) (yl - (RT(double)) 2 * RT(halfinf), -p->ulpexp);
  else
    p->tail = RT(lscalbn) (yl - p->y, -p->ulpexp);
  return 0;
}

/* There are nan input args and all quiet.  */
static inline int T(qnanpropagation) (struct T(args) a)
{
  return T(reduce) (a, isnan, ||) && !T(reduce) (a, RT(issignaling), ||);
}
static inline RT(float) T(sum) (struct T(args) a)
{
  return T(reduce) (a, , +);
}

/* returns 1 if the got result is ok.  */
static inline int T(call_mpfr_fix) (const struct fun *f, struct T(args) a,
				     int r_fenv, struct RT(ret) * p,
				     RT(float) ygot, int exgot)
{
#if USE_MPFR
  int t, t2;
  mpfr_rnd_t r = rmap (r_fenv);
  MPFR_DECL_INIT(my, RT(prec_mpfr));
  MPFR_DECL_INIT(mr, RT(prec));
  MPFR_DECL_INIT(me, RT(prec_mpfr));
  mpfr_clear_flags ();
  t = T(call_mpfr) (my, f, a, r);
  /* Double rounding.  */
  t2 = mpfr_set (mr, my, r);
  if (t2)
    t = t2;
  mpfr_set_emin (RT(emin));
  mpfr_set_emax (RT(emax));
  t = mpfr_check_range (mr, t, r);
  t = mpfr_subnormalize (mr, t, r);
  mpfr_set_emax (MPFR_EMAX_DEFAULT);
  mpfr_set_emin (MPFR_EMIN_DEFAULT);
  p->y = mpfr_get_d (mr, r);
  p->ex = t ? FE_INEXACT : 0;
  p->ex_may = FE_INEXACT;
  if (mpfr_underflow_p () && (p->ex & FE_INEXACT))
    /* TODO: handle before and after rounding uflow cases.  */
    p->ex |= FE_UNDERFLOW;
  if (mpfr_overflow_p ())
    p->ex |= FE_OVERFLOW | FE_INEXACT;
  if (mpfr_divby0_p ())
    p->ex |= FE_DIVBYZERO;
  //if (mpfr_erangeflag_p ())
  //  p->ex |= FE_INVALID;
  if (!mpfr_nanflag_p () && RT(isok) (ygot, exgot, p->y, p->ex, p->ex_may))
    return 1;
  if (mpfr_nanflag_p () && !T(qnanpropagation) (a))
    p->ex |= FE_INVALID;
  p->ulpexp = RT(ulpscale_mpfr) (my, t);
  if (!isfinite (p->y))
    {
      p->tail = 0;
      if (isnan (p->y))
	{
	  /* If an input was nan keep its sign.  */
	  p->y = T(sum) (a);
	  if (!isnan (p->y))
	    p->y = (p->y - p->y) / (p->y - p->y);
	  return RT(isok) (ygot, exgot, p->y, p->ex, p->ex_may);
	}
      mpfr_set_si_2exp (mr, signbit (p->y) ? -1 : 1, 1024, MPFR_RNDN);
      if (mpfr_cmpabs (my, mr) >= 0)
	return RT(isok) (ygot, exgot, p->y, p->ex, p->ex_may);
    }
  mpfr_sub (me, my, mr, MPFR_RNDN);
  mpfr_mul_2si (me, me, -p->ulpexp, MPFR_RNDN);
  p->tail = mpfr_get_d (me, MPFR_RNDN);
  return 0;
#else
  abort ();
#endif
}

static int T(cmp) (const struct fun *f, struct gen *gen,
		     const struct conf *conf)
{
  double maxerr = 0;
  uint64_t cnt = 0;
  uint64_t cnt1 = 0;
  uint64_t cnt2 = 0;
  uint64_t cntfail = 0;
  int r = conf->r;
  int use_mpfr = conf->mpfr;
  int fenv = conf->fenv;
  for (;;)
    {
      struct RT(ret) want;
      struct T(args) a = T(next) (gen);
      int exgot;
      int exgot2;
      RT(float) ygot;
      RT(float) ygot2;
      int fail = 0;
      if (fenv)
	T(call_fenv) (f, a, r, &ygot, &exgot);
      else
	T(call_nofenv) (f, a, r, &ygot, &exgot);
      if (f->twice) {
	secondcall = 1;
	if (fenv)
	  T(call_fenv) (f, a, r, &ygot2, &exgot2);
	else
	  T(call_nofenv) (f, a, r, &ygot2, &exgot2);
	secondcall = 0;
	if (RT(asuint) (ygot) != RT(asuint) (ygot2))
	  {
	    fail = 1;
	    cntfail++;
	    T(printcall) (f, a);
	    printf (" got %a then %a for same input\n", ygot, ygot2);
	  }
      }
      cnt++;
      int ok = use_mpfr
		 ? T(call_mpfr_fix) (f, a, r, &want, ygot, exgot)
		 : (fenv ? T(call_long_fenv) (f, a, r, &want, ygot, exgot)
			 : T(call_long_nofenv) (f, a, r, &want, ygot, exgot));
      if (!ok)
	{
	  int print = 0;
	  double err = RT(ulperr) (ygot, &want, r);
	  double abserr = fabs (err);
	  // TODO: count errors below accuracy limit.
	  if (abserr > 0)
	    cnt1++;
	  if (abserr > 1)
	    cnt2++;
	  if (abserr > conf->errlim)
	    {
	      print = 1;
	      if (!fail)
		{
		  fail = 1;
		  cntfail++;
		}
	    }
	  if (abserr > maxerr)
	    {
	      maxerr = abserr;
	      if (!conf->quiet && abserr > conf->softlim)
		print = 1;
	    }
	  if (print)
	    {
	      T(printcall) (f, a);
	      // TODO: inf ulp handling
	      printf (" got %a want %a %+g ulp err %g\n", ygot, want.y,
		      want.tail, err);
	    }
	  int diff = fenv ? exgot ^ want.ex : 0;
	  if (fenv && (diff & ~want.ex_may))
	    {
	      if (!fail)
		{
		  fail = 1;
		  cntfail++;
		}
	      T(printcall) (f, a);
	      printf (" is %a %+g ulp, got except 0x%0x", want.y, want.tail,
		      exgot);
	      if (diff & exgot)
		printf (" wrongly set: 0x%x", diff & exgot);
	      if (diff & ~exgot)
		printf (" wrongly clear: 0x%x", diff & ~exgot);
	      putchar ('\n');
	    }
	}
      if (cnt >= conf->n)
	break;
      if (!conf->quiet && cnt % 0x100000 == 0)
	printf ("progress: %6.3f%% cnt %llu cnt1 %llu cnt2 %llu cntfail %llu "
		"maxerr %g\n",
		100.0 * cnt / conf->n, (unsigned long long) cnt,
		(unsigned long long) cnt1, (unsigned long long) cnt2,
		(unsigned long long) cntfail, maxerr);
    }
  double cc = cnt;
  if (cntfail)
    printf ("FAIL ");
  else
    printf ("PASS ");
  T(printgen) (f, gen);
  printf (" round %c errlim %g maxerr %g %s cnt %llu cnt1 %llu %g%% cnt2 %llu "
	  "%g%% cntfail %llu %g%%\n",
	  conf->rc, conf->errlim,
	  maxerr, conf->r == FE_TONEAREST ? "+0.5" : "+1.0",
	  (unsigned long long) cnt,
	  (unsigned long long) cnt1, 100.0 * cnt1 / cc,
	  (unsigned long long) cnt2, 100.0 * cnt2 / cc,
	  (unsigned long long) cntfail, 100.0 * cntfail / cc);
  return !!cntfail;
}
