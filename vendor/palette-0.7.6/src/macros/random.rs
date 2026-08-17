#[cfg(feature = "random")]
#[cfg(test)]
macro_rules! assert_uniform_distribution {
    ($bins:expr) => {{
        let bins = &$bins;

        for (i, &bin) in bins.iter().enumerate() {
            if bin < 5 {
                panic!("{}[{}] < 5: {:?}", stringify!($bins), i, bins);
            }
        }
        const P_LIMIT: f64 = 0.01; // Keeping it low to account for the RNG noise
        let p_value = crate::random_sampling::test_utils::uniform_distribution_test(bins);
        if p_value < P_LIMIT {
            panic!(
                "distribution of {} is not uniform enough (p-value {} < {}): {:?}",
                stringify!($bins),
                p_value,
                P_LIMIT,
                bins
            );
        }
    }};
}

#[cfg(test)]
macro_rules! test_uniform_distribution {
    (
        $ty:path $(as $base_ty:path)? {
            $($component:ident: ($component_min:expr, $component_max:expr)),+$(,)?
        },
        min: $min:expr,
        max: $max:expr$(,)?
    ) => {
        #[cfg(feature = "random")]
        #[test]
        fn uniform_distribution_rng_gen() {
            use rand::Rng;

            const BINS: usize = crate::random_sampling::test_utils::BINS;
            const SAMPLES: usize = crate::random_sampling::test_utils::SAMPLES;

            $(let mut $component = [0; BINS];)+

            let mut rng = rand_mt::Mt::new(1234); // We want the same seed on every run to avoid random fails

            for _ in 0..SAMPLES {
                let color: $ty = rng.gen();
                $(let color: $base_ty = crate::convert::IntoColorUnclamped::into_color_unclamped(color);)?

                if $(color.$component < $component_min || color.$component > $component_max)||+ {
                    continue;
                }

                $({
                    let min: f32 = $component_min;
                    let max: f32 = $component_max;
                    let range = max - min;
                    let normalized = (color.$component - min) / range;
                    $component[((normalized * BINS as f32) as usize).min(BINS - 1)] += 1;
                })+
            }

            $(assert_uniform_distribution!($component);)+
        }

        #[cfg(feature = "random")]
        #[test]
        fn uniform_distribution_uniform_sample() {
            use rand::distributions::uniform::Uniform;
            use rand::Rng;

            const BINS: usize = crate::random_sampling::test_utils::BINS;
            const SAMPLES: usize = crate::random_sampling::test_utils::SAMPLES;

            $(let mut $component = [0; BINS];)+

            let mut rng = rand_mt::Mt::new(1234); // We want the same seed on every run to avoid random fails
            let uniform_sampler = Uniform::new($min, $max);

            for _ in 0..SAMPLES {
                let color: $ty = rng.sample(&uniform_sampler);
                $(let color: $base_ty = crate::convert::IntoColorUnclamped::into_color_unclamped(color);)?

                if $(color.$component < $component_min || color.$component > $component_max)||+ {
                    continue;
                }

                $({
                    let min: f32 = $component_min;
                    let max: f32 = $component_max;
                    let range = max - min;
                    let normalized = (color.$component - min) / range;
                    $component[((normalized * BINS as f32) as usize).min(BINS - 1)] += 1;
                })+
            }

            $(assert_uniform_distribution!($component);)+
        }

        #[cfg(feature = "random")]
        #[test]
        fn uniform_distribution_uniform_sample_inclusive() {
            use rand::distributions::uniform::Uniform;
            use rand::Rng;

            const BINS: usize = crate::random_sampling::test_utils::BINS;
            const SAMPLES: usize = crate::random_sampling::test_utils::SAMPLES;

            $(let mut $component = [0; BINS];)+

            let mut rng = rand_mt::Mt::new(1234); // We want the same seed on every run to avoid random fails
            let uniform_sampler = Uniform::new_inclusive($min, $max);

            for _ in 0..SAMPLES {
                let color: $ty = rng.sample(&uniform_sampler);
                $(let color: $base_ty = crate::convert::IntoColorUnclamped::into_color_unclamped(color);)?

                if $(color.$component < $component_min || color.$component > $component_max)||+ {
                    continue;
                }

                $({
                    let min: f32 = $component_min;
                    let max: f32 = $component_max;
                    let range = max - min;
                    let normalized = (color.$component - min) / range;
                    $component[((normalized * BINS as f32) as usize).min(BINS - 1)] += 1;
                })+
            }

            $(assert_uniform_distribution!($component);)+
        }
    };
}

macro_rules! __apply_map_fn {
    ($value: expr) => {
        $value
    };
    ($value: expr, $map_fn: expr) => {
        $map_fn($value)
    };
}

macro_rules! impl_rand_traits_cartesian {
    (
        $uniform_ty: ident,
        $ty: ident
        {$($component: ident $(=> [$map_fn: expr])?),+}
        $(phantom: $phantom: ident : PhantomData<$phantom_ty: ident>)?
        $(where $($where: tt)+)?
    ) => {
        impl_rand_traits_cartesian!(
            $uniform_ty,
            $ty<>
            {$($component $( => [$map_fn])?),+}
            $(phantom: $phantom : PhantomData<$phantom_ty>)?
            $(where $($where)+)?);
    };
    (
        $uniform_ty: ident,
        $ty: ident <$($ty_param: ident),*>
        {$($component: ident $(=> [$map_fn: expr])?),+}
        $(phantom: $phantom: ident : PhantomData<$phantom_ty: ident>)?
        $(where $($where: tt)+)?
    ) => {
        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::Distribution<$ty<$($ty_param,)* T>> for rand::distributions::Standard
        where
            rand::distributions::Standard: rand::distributions::Distribution<T>,
            $($($where)+)?
        {
            #[allow(clippy::redundant_closure_call)]
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty<$($ty_param,)* T> {
                $ty {
                    $($component: __apply_map_fn!(rng.gen::<T>() $(, $map_fn)?),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        /// Samples colors uniformly.
        #[cfg(feature = "random")]
        pub struct $uniform_ty<$($ty_param,)* T>
        where
            T: rand::distributions::uniform::SampleUniform,
        {
            $($component: rand::distributions::uniform::Uniform<T>,)+
            $($phantom: core::marker::PhantomData<$phantom_ty>,)?
        }

        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::uniform::SampleUniform for $ty<$($ty_param,)* T>
        where
            T: rand::distributions::uniform::SampleUniform + Clone,
        {
            type Sampler = $uniform_ty<$($ty_param,)* T>;
        }

        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::uniform::UniformSampler for $uniform_ty<$($ty_param,)* T>
        where
            T: rand::distributions::uniform::SampleUniform + Clone,
        {
            type X = $ty<$($ty_param,)* T>;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
                B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
            {
                let low = low_b.borrow();
                let high = high_b.borrow();

                Self {
                    $($component: rand::distributions::uniform::Uniform::new::<_, T>(low.$component.clone(), high.$component.clone()),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
                B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
            {
                let low = low_b.borrow();
                let high = high_b.borrow();

                Self {
                    $($component: rand::distributions::uniform::Uniform::new_inclusive::<_, T>(low.$component.clone(), high.$component.clone()),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty<$($ty_param,)* T> {
                use rand::distributions::Distribution;

                $ty {
                    $($component: self.$component.sample(rng),)+
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }
    };
}

macro_rules! impl_rand_traits_cylinder {
    (
        $uniform_ty: ident,
        $ty: ident
        {
            hue: $hue_uniform_ty: ident => $hue_ty: ident,
            height: $height: ident $(=> [$height_map_fn: expr])?,
            radius: $radius: ident $(=> [$radius_map_fn: expr])?
        }
        $(phantom: $phantom: ident : PhantomData<$phantom_ty: ident>)?
        $(where $($where: tt)+)?
    ) => {
        impl_rand_traits_cylinder!(
            $uniform_ty,
            $ty<>
            {
                hue: $hue_uniform_ty => $hue_ty,
                height: $height $(=> [$height_map_fn])?,
                radius: $radius $(=> [$radius_map_fn])?
            }
            $(phantom: $phantom : PhantomData<$phantom_ty>)?
            $(where $($where)+)?);
    };
    (
        $uniform_ty: ident,
        $ty: ident <$($ty_param: ident),*>
        {
            hue: $hue_uniform_ty: ident => $hue_ty: ident,
            height: $height: ident $(=> [$height_map_fn: expr])?,
            radius: $radius: ident $(=> [$radius_map_fn: expr])?
        }
        $(phantom: $phantom: ident : PhantomData<$phantom_ty: ident>)?
        $(where $($where: tt)+)?
    ) => {
        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::Distribution<$ty<$($ty_param,)* T>> for rand::distributions::Standard
        where
            T: crate::num::Sqrt,
            rand::distributions::Standard: rand::distributions::Distribution<T> + rand::distributions::Distribution<$hue_ty<T>>,
            $($($where)+)?
        {
            #[allow(clippy::redundant_closure_call)]
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty<$($ty_param,)* T> {
                $ty {
                    hue: rng.gen::<$hue_ty<T>>(),
                    $height: __apply_map_fn!(rng.gen::<T>() $(, $height_map_fn)?),
                    $radius: __apply_map_fn!(rng.gen::<T>().sqrt() $(, $radius_map_fn)?),
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        /// Samples colors uniformly.
        #[cfg(feature = "random")]
        pub struct $uniform_ty<$($ty_param,)* T>
        where
            T: rand::distributions::uniform::SampleUniform,
        {
            hue: crate::hues::$hue_uniform_ty<T>,
            $height: rand::distributions::uniform::Uniform<T>,
            $radius: rand::distributions::uniform::Uniform<T>,
            $($phantom: core::marker::PhantomData<$phantom_ty>,)?
        }

        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::uniform::SampleUniform for $ty<$($ty_param,)* T>
        where
            T: crate::num::Sqrt + core::ops::Mul<Output = T> + Clone + rand::distributions::uniform::SampleUniform,
            $hue_ty<T>: rand::distributions::uniform::SampleBorrow<$hue_ty<T>>,
            crate::hues::$hue_uniform_ty<T>: rand::distributions::uniform::UniformSampler<X = $hue_ty<T>>,
        {
            type Sampler = $uniform_ty<$($ty_param,)* T>;
        }

        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::uniform::UniformSampler for $uniform_ty<$($ty_param,)* T>
        where
            T: crate::num::Sqrt + core::ops::Mul<Output = T> + Clone + rand::distributions::uniform::SampleUniform,
            $hue_ty<T>: rand::distributions::uniform::SampleBorrow<$hue_ty<T>>,
            crate::hues::$hue_uniform_ty<T>: rand::distributions::uniform::UniformSampler<X = $hue_ty<T>>,
        {
            type X = $ty<$($ty_param,)* T>;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
                B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
            {
                let low = low_b.borrow().clone();
                let high = high_b.borrow().clone();

                $uniform_ty {
                    $height: rand::distributions::uniform::Uniform::new::<_, T>(low.$height, high.$height),
                    $radius: rand::distributions::uniform::Uniform::new::<_, T>(
                        low.$radius.clone() * low.$radius,
                        high.$radius.clone() * high.$radius,
                    ),
                    hue: crate::hues::$hue_uniform_ty::new(low.hue, high.hue),
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
                B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
            {
                let low = low_b.borrow().clone();
                let high = high_b.borrow().clone();

                $uniform_ty {
                    $height: rand::distributions::uniform::Uniform::new_inclusive::<_, T>(low.$height, high.$height),
                    $radius: rand::distributions::uniform::Uniform::new_inclusive::<_, T>(
                        low.$radius.clone() * low.$radius,
                        high.$radius.clone() * high.$radius,
                    ),
                    hue: crate::hues::$hue_uniform_ty::new_inclusive(low.hue, high.hue),
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty<$($ty_param,)* T> {
                use rand::distributions::Distribution;

                $ty {
                    $height: self.$height.sample(rng),
                    $radius: self.$radius.sample(rng).sqrt(),
                    hue: self.hue.sample(rng),
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }
    };
}

macro_rules! impl_rand_traits_hsv_cone {
    (
        $uniform_ty: ident,
        $ty: ident
        {
            hue: $hue_uniform_ty: ident => $hue_ty: ident,
            height: $height: ident,
            radius: $radius: ident
        }
        $(phantom: $phantom: ident : PhantomData<$phantom_ty: ident>)?
        $(where $($where: tt)+)?
    ) => {
        impl_rand_traits_hsv_cone!(
            $uniform_ty,
            $ty<>
            {
                hue: $hue_uniform_ty => $hue_ty,
                height: $height,
                radius: $radius
            }
            $(phantom: $phantom : PhantomData<$phantom_ty>)?
            $(where $($where)+)?);
    };
    (
        $uniform_ty: ident,
        $ty: ident <$($ty_param: ident),*>
        {
            hue: $hue_uniform_ty: ident => $hue_ty: ident,
            height: $height: ident,
            radius: $radius: ident
        }
        $(phantom: $phantom: ident : PhantomData<$phantom_ty: ident>)?
        $(where $($where: tt)+)?
    ) => {
        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::Distribution<$ty<$($ty_param,)* T>> for rand::distributions::Standard
        where
            T: crate::num::Cbrt + crate::num::Sqrt,
            rand::distributions::Standard: rand::distributions::Distribution<T> + rand::distributions::Distribution<$hue_ty<T>>,
        {
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty<$($ty_param,)* T> {
                let hue = rng.gen::<$hue_ty<T>>();
                let crate::random_sampling::HsvSample { saturation: $radius, value: $height } =
                    crate::random_sampling::sample_hsv(rng.gen(), rng.gen());

                $ty {
                    hue,
                    $radius,
                    $height,
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        /// Samples colors uniformly.
        #[cfg(feature = "random")]
        pub struct $uniform_ty<$($ty_param,)* T>
        where
            T: rand::distributions::uniform::SampleUniform,
        {
            hue: crate::hues::$hue_uniform_ty<T>,
            u1: rand::distributions::uniform::Uniform<T>,
            u2: rand::distributions::uniform::Uniform<T>,
            $($phantom: core::marker::PhantomData<$phantom_ty>,)?
        }

        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::uniform::SampleUniform for $ty<$($ty_param,)* T>
        where
            T: crate::num::Cbrt + crate::num::Sqrt + crate::num::Powi + Clone + rand::distributions::uniform::SampleUniform,
            $hue_ty<T>: rand::distributions::uniform::SampleBorrow<$hue_ty<T>>,
            crate::hues::$hue_uniform_ty<T>: rand::distributions::uniform::UniformSampler<X = $hue_ty<T>>,
        {
            type Sampler = $uniform_ty<$($ty_param,)* T>;
        }

        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::uniform::UniformSampler for $uniform_ty<$($ty_param,)* T>
        where
            T: crate::num::Cbrt + crate::num::Sqrt + crate::num::Powi + Clone + rand::distributions::uniform::SampleUniform,
            $hue_ty<T>: rand::distributions::uniform::SampleBorrow<$hue_ty<T>>,
            crate::hues::$hue_uniform_ty<T>: rand::distributions::uniform::UniformSampler<X = $hue_ty<T>>,
        {
            type X = $ty<$($ty_param,)* T>;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
                B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
            {
                let low = low_b.borrow().clone();
                let high = high_b.borrow().clone();

                let (r1_min, r2_min) =
                    crate::random_sampling::invert_hsv_sample(crate::random_sampling::HsvSample {
                        value: low.$height,
                        saturation: low.$radius,
                    });
                let (r1_max, r2_max) =
                    crate::random_sampling::invert_hsv_sample(crate::random_sampling::HsvSample {
                        value: high.$height,
                        saturation: high.$radius,
                    });

                $uniform_ty {
                    hue: crate::hues::$hue_uniform_ty::new(low.hue, high.hue),
                    u1: rand::distributions::uniform::Uniform::new::<_, T>(r1_min, r1_max),
                    u2: rand::distributions::uniform::Uniform::new::<_, T>(r2_min, r2_max),
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
                B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
            {
                let low = low_b.borrow().clone();
                let high = high_b.borrow().clone();

                let (r1_min, r2_min) =
                    crate::random_sampling::invert_hsv_sample(crate::random_sampling::HsvSample {
                        value: low.$height,
                        saturation: low.$radius,
                    });
                let (r1_max, r2_max) =
                    crate::random_sampling::invert_hsv_sample(crate::random_sampling::HsvSample {
                        value: high.$height,
                        saturation: high.$radius,
                    });

                $uniform_ty {
                    hue: crate::hues::$hue_uniform_ty::new_inclusive(low.hue, high.hue),
                    u1: rand::distributions::uniform::Uniform::new_inclusive::<_, T>(r1_min, r1_max),
                    u2: rand::distributions::uniform::Uniform::new_inclusive::<_, T>(r2_min, r2_max),
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty<$($ty_param,)* T> {
                use rand::distributions::Distribution;

                let hue = self.hue.sample(rng);
                let crate::random_sampling::HsvSample { saturation: $radius, value: $height } =
                    crate::random_sampling::sample_hsv(self.u1.sample(rng), self.u2.sample(rng));

                $ty {
                    hue,
                    $radius,
                    $height,
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }
    }
}

macro_rules! impl_rand_traits_hsl_bicone {
    (
        $uniform_ty: ident,
        $ty: ident
        {
            hue: $hue_uniform_ty: ident => $hue_ty: ident,
            height: $height: ident $(=> [$height_map_fn: expr, $height_unmap_fn: expr])?,
            radius: $radius: ident $(=> [$radius_map_fn: expr, $radius_unmap_fn: expr])?
        }
        $(phantom: $phantom: ident : PhantomData<$phantom_ty: ident>)?
        $(where $($where: tt)+)?
    ) => {
        impl_rand_traits_hsl_bicone!(
            $uniform_ty,
            $ty<>
            {
                hue: $hue_uniform_ty => $hue_ty,
                height: $height $(=> [$height_map_fn, $height_unmap_fn])?,
                radius: $radius $(=> [$radius_map_fn, $radius_unmap_fn])?
            }
            $(phantom: $phantom : PhantomData<$phantom_ty>)?
            $(where $($where)+)?);
    };
    (
        $uniform_ty: ident,
        $ty: ident <$($ty_param: ident),*>
        {
            hue: $hue_uniform_ty: ident => $hue_ty: ident,
            height: $height: ident $(=> [$height_map_fn: expr, $height_unmap_fn: expr])?,
            radius: $radius: ident $(=> [$radius_map_fn: expr, $radius_unmap_fn: expr])?
        }
        $(phantom: $phantom: ident : PhantomData<$phantom_ty: ident>)?
        $(where $($where: tt)+)?
    ) => {
        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::Distribution<$ty<$($ty_param,)* T>> for rand::distributions::Standard
        where
            T: crate::num::Real + crate::num::One + crate::num::Cbrt + crate::num::Sqrt + crate::num::Arithmetics + crate::num::PartialCmp + Clone,
            T::Mask: crate::bool_mask::LazySelect<T> + Clone,
            rand::distributions::Standard: rand::distributions::Distribution<T> + rand::distributions::Distribution<$hue_ty<T>>,
        {
            #[allow(clippy::redundant_closure_call)]
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty<$($ty_param,)* T> {
                let hue = rng.gen::<$hue_ty<T>>();
                let crate::random_sampling::HslSample { saturation, lightness } =
                    crate::random_sampling::sample_hsl(rng.gen(), rng.gen());

                $ty {
                    hue,
                    $radius: __apply_map_fn!(saturation $(, $radius_map_fn)?),
                    $height: __apply_map_fn!(lightness $(, $height_map_fn)?),
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }

        /// Samples colors uniformly.
        #[cfg(feature = "random")]
        pub struct $uniform_ty<$($ty_param,)* T>
        where
            T: rand::distributions::uniform::SampleUniform,
        {
            hue: crate::hues::$hue_uniform_ty<T>,
            u1: rand::distributions::uniform::Uniform<T>,
            u2: rand::distributions::uniform::Uniform<T>,
            $($phantom: core::marker::PhantomData<$phantom_ty>,)?
        }

        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::uniform::SampleUniform for $ty<$($ty_param,)* T>
        where
            T: crate::num::Real + crate::num::One + crate::num::Cbrt + crate::num::Sqrt + crate::num::Powi + crate::num::Arithmetics + crate::num::PartialCmp + Clone + rand::distributions::uniform::SampleUniform,
            T::Mask: crate::bool_mask::LazySelect<T> + Clone,
            $hue_ty<T>: rand::distributions::uniform::SampleBorrow<$hue_ty<T>>,
            crate::hues::$hue_uniform_ty<T>: rand::distributions::uniform::UniformSampler<X = $hue_ty<T>>,
        {
            type Sampler = $uniform_ty<$($ty_param,)* T>;
        }

        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::uniform::UniformSampler for $uniform_ty<$($ty_param,)* T>
        where
            T: crate::num::Real + crate::num::One + crate::num::Cbrt + crate::num::Sqrt + crate::num::Powi + crate::num::Arithmetics + crate::num::PartialCmp + Clone + rand::distributions::uniform::SampleUniform,
            T::Mask: crate::bool_mask::LazySelect<T> + Clone,
            $hue_ty<T>: rand::distributions::uniform::SampleBorrow<$hue_ty<T>>,
            crate::hues::$hue_uniform_ty<T>: rand::distributions::uniform::UniformSampler<X = $hue_ty<T>>,
        {
            type X = $ty<$($ty_param,)* T>;

            #[allow(clippy::redundant_closure_call)]
            fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
                B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
            {
                let low = low_b.borrow().clone();
                let high = high_b.borrow().clone();

                let (r1_min, r2_min) =
                    crate::random_sampling::invert_hsl_sample(crate::random_sampling::HslSample {
                        lightness: __apply_map_fn!(low.$height $(, $radius_unmap_fn)?),
                        saturation: __apply_map_fn!(low.$radius $(, $height_unmap_fn)?),
                    });
                let (r1_max, r2_max) =
                    crate::random_sampling::invert_hsl_sample(crate::random_sampling::HslSample {
                        lightness: __apply_map_fn!(high.$height $(, $radius_unmap_fn)?),
                        saturation: __apply_map_fn!(high.$radius $(, $height_unmap_fn)?),
                    });

                $uniform_ty {
                    hue: crate::hues::$hue_uniform_ty::new(low.hue, high.hue),
                    u1: rand::distributions::uniform::Uniform::new::<_, T>(r1_min, r1_max),
                    u2: rand::distributions::uniform::Uniform::new::<_, T>(r2_min, r2_max),
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            #[allow(clippy::redundant_closure_call)]
            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
                B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
            {
                let low = low_b.borrow().clone();
                let high = high_b.borrow().clone();

                let (r1_min, r2_min) =
                    crate::random_sampling::invert_hsl_sample(crate::random_sampling::HslSample {
                        lightness: __apply_map_fn!(low.$height $(, $radius_unmap_fn)?),
                        saturation: __apply_map_fn!(low.$radius $(, $height_unmap_fn)?),
                    });
                let (r1_max, r2_max) =
                    crate::random_sampling::invert_hsl_sample(crate::random_sampling::HslSample {
                        lightness: __apply_map_fn!(high.$height $(, $radius_unmap_fn)?),
                        saturation: __apply_map_fn!(high.$radius $(, $height_unmap_fn)?),
                    });

                $uniform_ty {
                    hue: crate::hues::$hue_uniform_ty::new_inclusive(low.hue, high.hue),
                    u1: rand::distributions::uniform::Uniform::new_inclusive::<_, T>(r1_min, r1_max),
                    u2: rand::distributions::uniform::Uniform::new_inclusive::<_, T>(r2_min, r2_max),
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            #[allow(clippy::redundant_closure_call)]
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty<$($ty_param,)* T> {
                use rand::distributions::Distribution;

                let hue = self.hue.sample(rng);
                let crate::random_sampling::HslSample { saturation, lightness } =
                    crate::random_sampling::sample_hsl(self.u1.sample(rng), self.u2.sample(rng));

                $ty {
                    hue,
                    $radius: __apply_map_fn!(saturation $(, $radius_map_fn)?),
                    $height: __apply_map_fn!(lightness $(, $height_map_fn)?),
                    $($phantom: core::marker::PhantomData,)?
                }
            }
        }
    }
}

macro_rules! impl_rand_traits_hwb_cone {
    (
        $uniform_ty: ident,
        $ty: ident,
        $hsv_uniform_ty: ident,
        $hsv_ty: ident
        {
            height: $height: ident,
            radius: $radius: ident
        }
        $(phantom: $phantom: ident : PhantomData<$phantom_ty: ident>)?
        $(where $($where: tt)+)?
    ) => {
        impl_rand_traits_hwb_cone!(
            $uniform_ty,
            $ty<>,
            $hsv_uniform_ty,
            $hsv_ty
            {
                height: $height,
                radius: $radius
            }
            $(phantom: $phantom : PhantomData<$phantom_ty>)?
            $(where $($where)+)?);
    };
    (
        $uniform_ty: ident,
        $ty: ident <$($ty_param: ident),*>,
        $hsv_uniform_ty: ident,
        $hsv_ty: ident
        {
            height: $height: ident,
            radius: $radius: ident
        }
        $(phantom: $phantom: ident : PhantomData<$phantom_ty: ident>)?
        $(where $($where: tt)+)?
    ) => {
        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::Distribution<$ty<$($ty_param,)* T>> for rand::distributions::Standard
        where
            rand::distributions::Standard: rand::distributions::Distribution<$hsv_ty<$($ty_param,)* T>>,
            $ty<$($ty_param,)* T>: crate::convert::FromColorUnclamped<$hsv_ty<$($ty_param,)* T>>,
        {
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty<$($ty_param,)* T> {
                use crate::convert::FromColorUnclamped;
                $ty::from_color_unclamped(rng.gen::<$hsv_ty<$($ty_param,)* T>>())
            }
        }

        /// Samples colors uniformly.
        #[cfg(feature = "random")]
        pub struct $uniform_ty<$($ty_param,)* T>
        where
            T: rand::distributions::uniform::SampleUniform,
        {
            sampler: $hsv_uniform_ty<$($ty_param,)* T>,
            $($phantom: core::marker::PhantomData<$phantom_ty>,)?
        }

        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::uniform::SampleUniform for $ty<$($ty_param,)* T>
        where
            T: crate::num::MinMax + Clone + rand::distributions::uniform::SampleUniform,
            $hsv_ty<$($ty_param,)* T>: crate::convert::FromColorUnclamped<$ty<$($ty_param,)* T>> + rand::distributions::uniform::SampleBorrow<$hsv_ty<$($ty_param,)* T>>,
            $ty<$($ty_param,)* T>: crate::convert::FromColorUnclamped<$hsv_ty<$($ty_param,)* T>>,
            $hsv_uniform_ty<$($ty_param,)* T>: rand::distributions::uniform::UniformSampler<X = $hsv_ty<$($ty_param,)* T>>,
        {
            type Sampler = $uniform_ty<$($ty_param,)* T>;
        }

        #[cfg(feature = "random")]
        impl<$($ty_param,)* T> rand::distributions::uniform::UniformSampler for $uniform_ty<$($ty_param,)* T>
        where
            T: crate::num::MinMax + Clone + rand::distributions::uniform::SampleUniform,
            $hsv_ty<$($ty_param,)* T>: crate::convert::FromColorUnclamped<$ty<$($ty_param,)* T>> + rand::distributions::uniform::SampleBorrow<$hsv_ty<$($ty_param,)* T>>,
            $ty<$($ty_param,)* T>: crate::convert::FromColorUnclamped<$hsv_ty<$($ty_param,)* T>>,
            $hsv_uniform_ty<$($ty_param,)* T>: rand::distributions::uniform::UniformSampler<X = $hsv_ty<$($ty_param,)* T>>,
        {
            type X = $ty<$($ty_param,)* T>;

            fn new<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
                B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
            {
                use crate::convert::FromColorUnclamped;
                let low_input = $hsv_ty::from_color_unclamped(low_b.borrow().clone());
                let high_input = $hsv_ty::from_color_unclamped(high_b.borrow().clone());

                let (low_saturation, high_saturation) = low_input.saturation.min_max(high_input.saturation);
                let (low_value, high_value) = low_input.value.min_max(high_input.value);

                let low = $hsv_ty{
                    hue: low_input.hue,
                    $radius: low_saturation,
                    $height: low_value,
                    $($phantom: core::marker::PhantomData,)?
                };
                let high = $hsv_ty{
                    hue: high_input.hue,
                    $radius: high_saturation,
                    $height: high_value,
                    $($phantom: core::marker::PhantomData,)?
                };

                let sampler = $hsv_uniform_ty::<$($ty_param,)* T>::new(low, high);

                $uniform_ty {
                    sampler,
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            fn new_inclusive<B1, B2>(low_b: B1, high_b: B2) -> Self
            where
                B1: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
                B2: rand::distributions::uniform::SampleBorrow<Self::X> + Sized,
            {
                use crate::convert::FromColorUnclamped;
                let low_input = $hsv_ty::from_color_unclamped(low_b.borrow().clone());
                let high_input = $hsv_ty::from_color_unclamped(high_b.borrow().clone());

                let (low_saturation, high_saturation) = low_input.saturation.min_max(high_input.saturation);
                let (low_value, high_value) = low_input.value.min_max(high_input.value);

                let low = $hsv_ty{
                    hue: low_input.hue,
                    $radius: low_saturation,
                    $height: low_value,
                    $($phantom: core::marker::PhantomData,)?
                };
                let high = $hsv_ty{
                    hue: high_input.hue,
                    $radius: high_saturation,
                    $height: high_value,
                    $($phantom: core::marker::PhantomData,)?
                };

                let sampler = $hsv_uniform_ty::<$($ty_param,)* T>::new_inclusive(low, high);

                $uniform_ty {
                    sampler,
                    $($phantom: core::marker::PhantomData,)?
                }
            }

            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $ty<$($ty_param,)* T> {
                use crate::convert::FromColorUnclamped;
                $ty::from_color_unclamped(self.sampler.sample(rng))
            }
        }
    };
}
