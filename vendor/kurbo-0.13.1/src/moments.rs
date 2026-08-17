// Copyright 2025 the Kurbo Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::{PathEl, Point};

#[cfg(not(feature = "std"))]
use crate::common::FloatFuncs;

/// A collect of moment integrals for a path, used for computing
/// shape properties like area, centroid, and inertia.
///
/// The moments are computed using Green's theorem, which relates the
/// integrals of a function over a closed curve to the integrals of its
/// partial derivatives over the region enclosed by the curve.
#[derive(Debug, Default, Copy, Clone)]
pub struct Moments {
    /// The first moment of area about the x-axis.
    pub moment_x: f64,
    /// The first moment of area about the y-axis.
    pub moment_y: f64,
    /// The second moment of area about the x-axis (also known as the
    /// area moment of inertia about the x-axis).
    pub moment_xx: f64,
    /// The second moment of area about the xy-axis (also known as the
    /// product of inertia).
    pub moment_xy: f64,
    /// The second moment of area about the y-axis (also known as the
    /// area moment of inertia about the y-axis).
    pub moment_yy: f64,
}

impl core::ops::Add<Self> for Moments {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            moment_x: self.moment_x + rhs.moment_x,
            moment_y: self.moment_y + rhs.moment_y,
            moment_xx: self.moment_xx + rhs.moment_xx,
            moment_xy: self.moment_xy + rhs.moment_xy,
            moment_yy: self.moment_yy + rhs.moment_yy,
        }
    }
}

impl Moments {
    // The code below is derived from fontTools's fontTools.pens.momentsPen, which is in turn generated
    // by running sympy on the Green's theorem integral expressions.
    fn handle_line(&mut self, p0: Point, p1: Point) {
        let (x0, y0) = (p0.x, p0.y);
        let (x1, y1) = (p1.x, p1.y);
        let r0 = x1 * y0;
        let r1 = x1 * y1;
        let r2 = x1.powi(2);
        let r3 = r2 * y1;
        let r4 = y0 - y1;
        let r5 = r4 * x0;
        let r6 = x0.powi(2);
        let r7 = 2.0 * y0;
        let r8 = y0.powi(2);
        let r9 = y1.powi(2);
        let r10 = x1.powi(3);
        let r11 = y0.powi(3);
        let r12 = y1.powi(3);
        self.moment_x += -r2 * y0 / 6.0 - r3 / 3.0 - r5 * x1 / 6.0 + r6 * (r7 + y1) / 6.0;
        self.moment_y +=
            -r0 * y1 / 6.0 - r8 * x1 / 6.0 - r9 * x1 / 6.0 + x0 * (r8 + r9 + y0 * y1) / 6.0;
        self.moment_xx += -r10 * y0 / 12.0 - r10 * y1 / 4.0 - r2 * r5 / 12.0 - r4 * r6 * x1 / 12.0
            + x0.powi(3) * (3.0 * y0 + y1) / 12.0;
        self.moment_xy += -r2 * r8 / 24.0 - r2 * r9 / 8.0 - r3 * r7 / 24.0
            + r6 * (r7 * y1 + 3.0 * r8 + r9) / 24.0
            - x0 * x1 * (r8 - r9) / 12.0;
        self.moment_yy += -r0 * r9 / 12.0 - r1 * r8 / 12.0 - r11 * x1 / 12.0 - r12 * x1 / 12.0
            + x0 * (r11 + r12 + r8 * y1 + r9 * y0) / 12.0;
    }

    fn handle_quad(&mut self, p0: Point, p1: Point, p2: Point) {
        let (x0, y0) = (p0.x, p0.y);
        let x1 = p1.x;
        let y1 = p1.y;
        let x2 = p2.x;
        let y2 = p2.y;

        let r0 = 2.0 * y1;
        let r1 = r0 * x2;
        let r2 = x2 * y2;
        let r3 = 3.0 * r2;
        let r4 = 2.0 * x1;
        let r5 = 3.0 * y0;
        let r6 = x1.powi(2);
        let r7 = x2.powi(2);
        let r8 = 4.0 * y1;
        let r9 = 10.0 * y2;
        let r10 = 2.0 * y2;
        let r11 = r4 * x2;
        let r12 = x0.powi(2);
        let r13 = 10.0 * y0;
        let r14 = r4 * y2;
        let r15 = x2 * y0;
        let r16 = 4.0 * x1;
        let r17 = r0 * x1 + r2;
        let r18 = r2 * r8;
        let r19 = y1.powi(2);
        let r20 = 2.0 * r19;
        let r21 = y2.powi(2);
        let r22 = r21 * x2;
        let r23 = 5.0 * r22;
        let r24 = y0.powi(2);
        let r25 = y0 * y2;
        let r26 = 5.0 * r24;
        let r27 = x1.powi(3);
        let r28 = x2.powi(3);
        let r29 = 30.0 * y1;
        let r30 = 6.0 * y1;
        let r31 = 10.0 * r7 * x1;
        let r32 = 5.0 * y2;
        let r33 = 12.0 * r6;
        let r34 = 30.0 * x1;
        let r35 = x1 * y1;
        let r36 = r3 + 20.0 * r35;
        let r37 = 12.0 * x1;
        let r38 = 20.0 * r6;
        let r39 = 8.0 * r6 * y1;
        let r40 = r32 * r7;
        let r41 = 60.0 * y1;
        let r42 = 20.0 * r19;
        let r43 = 4.0 * r19;
        let r44 = 15.0 * r21;
        let r45 = 12.0 * x2;
        let r46 = 12.0 * y2;
        let r47 = 6.0 * x1;
        let r48 = 8.0 * r19 * x1 + r23;
        let r49 = 8.0 * y1.powi(3);
        let r50 = y2.powi(3);
        let r51 = y0.powi(3);
        let r52 = 10.0 * y1;
        let r53 = 12.0 * y1;
        self.moment_x += -r11 * (-r10 + y1) / 30.0 + r12 * (r13 + r8 + y2) / 30.0 + r6 * y2 / 15.0
            - r7 * r8 / 30.0
            - r7 * r9 / 30.0
            + x0 * (r14 - r15 - r16 * y0 + r17) / 30.0
            - y0 * (r11 + 2.0 * r6 + r7) / 30.0;
        self.moment_y += -r18 / 30.0 - r20 * x2 / 30.0 - r23 / 30.0 - r24 * (r16 + x2) / 30.0
            + x0 * (r0 * y2 + r20 + r21 + r25 + r26 + r8 * y0) / 30.0
            + x1 * y2 * (r10 + y1) / 15.0
            - y0 * (r1 + r17) / 30.0;
        self.moment_xx += r12 * (r1 - 5.0 * r15 - r34 * y0 + r36 + r9 * x1) / 420.0
            + 2.0 * r27 * y2 / 105.0
            - r28 * r29 / 420.0
            - r28 * y2 / 4.0
            - r31 * (r0 - 3.0 * y2) / 420.0
            - r6 * x2 * (r0 - r32) / 105.0
            + x0.powi(3) * (r30 + 21.0 * y0 + y2) / 84.0
            - x0 * (r0 * r7 + r15 * r37 - r2 * r37 - r33 * y2 + r38 * y0 - r39 - r40 + r5 * r7)
                / 420.0
            - y0 * (8.0 * r27 + 5.0 * r28 + r31 + r33 * x2) / 420.0;
        self.moment_xy += r12 * (r13 * y2 + 3.0 * r21 + 105.0 * r24 + r41 * y0 + r42 + r46 * y1)
            / 840.0
            - r16 * x2 * (r43 - r44) / 840.0
            - r21 * r7 / 8.0
            - r24 * (r38 + r45 * x1 + 3.0 * r7) / 840.0
            - r41 * r7 * y2 / 840.0
            - r42 * r7 / 840.0
            + r6 * y2 * (r32 + r8) / 210.0
            + x0 * (-r15 * r8 + r16 * r25 + r18 + r21 * r47 - r24 * r34 - r26 * x2
                + r35 * r46
                + r48)
                / 420.0
            - y0 * (r16 * r2 + r30 * r7 + r35 * r45 + r39 + r40) / 420.0;

        self.moment_yy += -r2 * r42 / 420.0
            - r22 * r29 / 420.0
            - r24 * (r14 + r36 + r52 * x2) / 420.0
            - r49 * x2 / 420.0
            - r50 * x2 / 12.0
            - r51 * (r47 + x2) / 84.0
            + x0 * (r19 * r46
                + r21 * r5
                + r21 * r52
                + r24 * r29
                + r25 * r53
                + r26 * y2
                + r42 * y0
                + r49
                + 5.0 * r50
                + 35.0 * r51)
                / 420.0
            + x1 * y2 * (r43 + r44 + r9 * y1) / 210.0
            - y0 * (r19 * r45 + r2 * r53 - r21 * r4 + r48) / 420.0;
    }

    fn handle_cubic(&mut self, p0: Point, p1: Point, p2: Point, p3: Point) {
        let x0 = p0.x;
        let y0 = p0.y;
        let x1 = p1.x;
        let y1 = p1.y;
        let x2 = p2.x;
        let y2 = p2.y;
        let x3 = p3.x;
        let y3 = p3.y;

        let r0 = 6.0 * y2;
        let r1 = r0 * x3;
        let r2 = 10.0 * y3;
        let r3 = r2 * x3;
        let r4 = 3.0 * y1;
        let r5 = 6.0 * x1;
        let r6 = 3.0 * x2;
        let r7 = 6.0 * y1;
        let r8 = 3.0 * y2;
        let r9 = x2.powi(2);
        let r10 = 45.0 * r9;
        let r11 = r10 * y3;
        let r12 = x3.powi(2);
        let r13 = r12 * y2;
        let r14 = r12 * y3;
        let r15 = 7.0 * y3;
        let r16 = 15.0 * x3;
        let r17 = r16 * x2;
        let r18 = x1.powi(2);
        let r19 = 9.0 * r18;
        let r20 = x0.powi(2);
        let r21 = 21.0 * y1;
        let r22 = 9.0 * r9;
        let r23 = r7 * x3;
        let r24 = 9.0 * y2;
        let r25 = r24 * x2 + r3;
        let r26 = 9.0 * x2;
        let r27 = x2 * y3;
        let r28 = -r26 * y1 + 15.0 * r27;
        let r29 = 3.0 * x1;
        let r30 = 45.0 * x1;
        let r31 = 12.0 * x3;
        let r32 = 45.0 * r18;
        let r33 = 5.0 * r12;
        let r34 = r8 * x3;
        let r35 = 105.0 * y0;
        let r36 = 30.0 * y0;
        let r37 = r36 * x2;
        let r38 = 5.0 * x3;
        let r39 = 15.0 * y3;
        let r40 = 5.0 * y3;
        let r41 = r40 * x3;
        let r42 = x2 * y2;
        let r43 = 18.0 * r42;
        let r44 = 45.0 * y1;
        let r45 = r41 + r43 + r44 * x1;
        let r46 = y2 * y3;
        let r47 = r46 * x3;
        let r48 = y2.powi(2);
        let r49 = 45.0 * r48;
        let r50 = r49 * x3;
        let r51 = y3.powi(2);
        let r52 = r51 * x3;
        let r53 = y1.powi(2);
        let r54 = 9.0 * r53;
        let r55 = y0.powi(2);
        let r56 = 21.0 * x1;
        let r57 = 6.0 * x2;
        let r58 = r16 * y2;
        let r59 = r39 * y2;
        let r60 = 9.0 * r48;
        let r61 = r6 * y3;
        let r62 = 3.0 * y3;
        let r63 = r36 * y2;
        let r64 = y1 * y3;
        let r65 = 45.0 * r53;
        let r66 = 5.0 * r51;
        let r67 = x2.powi(3);
        let r68 = x3.powi(3);
        let r69 = 630.0 * y2;
        let r70 = 126.0 * x3;
        let r71 = x1.powi(3);
        let r72 = 126.0 * x2;
        let r73 = 63.0 * r9;
        let r74 = r73 * x3;
        let r75 = r15 * x3 + 15.0 * r42;
        let r76 = 630.0 * x1;
        let r77 = 14.0 * x3;
        let r78 = 21.0 * r27;
        let r79 = 42.0 * x1;
        let r80 = 42.0 * x2;
        let r81 = x1 * y2;
        let r82 = 63.0 * r42;
        let r83 = x1 * y1;
        let r84 = r41 + r82 + 378.0 * r83;
        let r85 = x2 * x3;
        let r86 = r85 * y1;
        let r87 = r27 * x3;
        let r88 = 27.0 * r9;
        let r89 = r88 * y2;
        let r90 = 42.0 * r14;
        let r91 = 90.0 * x1;
        let r92 = 189.0 * r18;
        let r93 = 378.0 * r18;
        let r94 = r12 * y1;
        let r95 = 252.0 * x1 * x2;
        let r96 = r79 * x3;
        let r97 = 30.0 * r85;
        let r98 = r83 * x3;
        let r99 = 30.0 * x3;
        let r100 = 42.0 * x3;
        let r101 = r42 * x1;
        let r102 = r10 * y2 + 14.0 * r14 + 126.0 * r18 * y1 + r81 * r99;
        let r103 = 378.0 * r48;
        let r104 = 18.0 * y1;
        let r105 = r104 * y2;
        let r106 = y0 * y1;
        let r107 = 252.0 * y2;
        let r108 = r107 * y0;
        let r109 = y0 * y3;
        let r110 = 42.0 * r64;
        let r111 = 378.0 * r53;
        let r112 = 63.0 * r48;
        let r113 = 27.0 * x2;
        let r114 = r27 * y2;
        let r115 = r113 * r48 + 42.0 * r52;
        let r116 = x3 * y3;
        let r117 = 54.0 * r42;
        let r118 = r51 * x1;
        let r119 = r51 * x2;
        let r120 = r48 * x1;
        let r121 = 21.0 * x3;
        let r122 = r64 * x1;
        let r123 = r81 * y3;
        let r124 = 30.0 * r27 * y1 + r49 * x2 + 14.0 * r52 + 126.0 * r53 * x1;
        let r125 = y2.powi(3);
        let r126 = y3.powi(3);
        let r127 = y1.powi(3);
        let r128 = y0.powi(3);
        let r129 = r51 * y2;
        let r130 = r112 * y3 + r21 * r51;
        let r131 = 189.0 * r53;
        let r132 = 90.0 * y2;
        self.moment_x += r11 / 840.0 - r13 / 8.0 - r14 / 3.0 - r17 * (-r15 + r8) / 840.0
            + r19 * (r8 + 2.0 * y3) / 840.0
            + r20 * (r0 + r21 + 56.0 * y0 + y3) / 168.0
            + r29 * (-r23 + r25 + r28) / 840.0
            - r4 * (10.0 * r12 + r17 + r22) / 840.0
            + x0 * (12.0 * r27 + r30 * y2 + r34 - r35 * x1 - r37 - r38 * y0 + r39 * x1 - r4 * x3
                + r45)
                / 840.0
            - y0 * (r17 + r30 * x2 + r31 * x1 + r32 + r33 + 18.0 * r9) / 840.0;
        self.moment_y += -r4 * (r25 + r58) / 840.0
            - r47 / 8.0
            - r50 / 840.0
            - r52 / 6.0
            - r54 * (r6 + 2.0 * x3) / 840.0
            - r55 * (r56 + r57 + x3) / 168.0
            + x0 * (r35 * y1
                + r40 * y0
                + r44 * y2
                + 18.0 * r48
                + 140.0 * r55
                + r59
                + r63
                + 12.0 * r64
                + r65
                + r66)
                / 840.0
            + x1 * (r24 * y1 + 10.0 * r51 + r59 + r60 + r7 * y3) / 280.0
            + x2 * y3 * (r15 + r8) / 56.0
            - y0 * (r16 * y1 + r31 * y2 + r44 * x2 + r45 + r61 - r62 * x1) / 840.0;
        self.moment_xx += -r12 * r72 * (-r40 + r8) / 9240.0
            + 3.0 * r18 * (r28 + r34 - r38 * y1 + r75) / 3080.0
            + r20
                * (r24 * x3 - r72 * y0 - r76 * y0 - r77 * y0
                    + r78
                    + r79 * y3
                    + r80 * y1
                    + 210.0 * r81
                    + r84)
                / 9240.0
            - r29
                * (r12 * r21 + 14.0 * r13 + r44 * r9 - r73 * y3 + 54.0 * r86
                    - 84.0 * r87
                    - r89
                    - r90)
                / 9240.0
            - r4 * (70.0 * r12 * x2 + 27.0 * r67 + 42.0 * r68 + r74) / 9240.0
            + 3.0 * r67 * y3 / 220.0
            - r68 * r69 / 9240.0
            - r68 * y3 / 4.0
            - r70 * r9 * (-r62 + y2) / 9240.0
            + 3.0 * r71 * (r24 + r40) / 3080.0
            + x0.powi(3) * (r24 + r44 + 165.0 * y0 + y3) / 660.0
            + x0 * (r100 * r27 + 162.0 * r101 + r102 + r11 + 63.0 * r18 * y3 + r27 * r91
                - r33 * y0
                - r37 * x3
                + r43 * x3
                - r73 * y0
                - r88 * y1
                + r92 * y2
                - r93 * y0
                - 9.0 * r94
                - r95 * y0
                - r96 * y0
                - r97 * y1
                - 18.0 * r98
                + r99 * x1 * y3)
                / 9240.0
            - y0 * (r12 * r56
                + r12 * r80
                + r32 * x3
                + 45.0 * r67
                + 14.0 * r68
                + 126.0 * r71
                + r74
                + r85 * r91
                + 135.0 * r9 * x1
                + r92 * x2)
                / 9240.0;
        self.moment_xy += -r103 * r12 / 18480.0 - r12 * r51 / 8.0 - 3.0 * r14 * y2 / 44.0
            + 3.0 * r18 * (r105 + r2 * y1 + 18.0 * r46 + 15.0 * r48 + 7.0 * r51) / 6160.0
            + r20
                * (1260.0 * r106
                    + r107 * y1
                    + r108
                    + 28.0 * r109
                    + r110
                    + r111
                    + r112
                    + 30.0 * r46
                    + 2310.0 * r55
                    + r66)
                / 18480.0
            - r54 * (7.0 * r12 + 18.0 * r85 + 15.0 * r9) / 18480.0
            - r55 * (r33 + r73 + r93 + r95 + r96 + r97) / 18480.0
            - r7 * (42.0 * r13 + r82 * x3 + 28.0 * r87 + r89 + r90) / 18480.0
            - 3.0 * r85 * (r48 - r66) / 220.0
            + 3.0 * r9 * y3 * (r62 + 2.0 * y2) / 440.0
            + x0 * (-r1 * y0 - 84.0 * r106 * x2
                + r109 * r56
                + 54.0 * r114
                + r117 * y1
                + 15.0 * r118
                + 21.0 * r119
                + 81.0 * r120
                + r121 * r46
                + 54.0 * r122
                + 60.0 * r123
                + r124
                - r21 * x3 * y0
                + r23 * y3
                - r54 * x3
                - r55 * r72
                - r55 * r76
                - r55 * r77
                + r57 * y0 * y3
                + r60 * x3
                + 84.0 * r81 * y0
                + 189.0 * r81 * y1)
                / 9240.0
            + x1 * (r104 * r27 - r105 * x3 - r113 * r53 + 63.0 * r114 + r115 - r16 * r53
                + 28.0 * r47
                + r51 * r80)
                / 3080.0
            - y0 * (54.0 * r101 + r102 + r116 * r5 + r117 * x3 + 21.0 * r13 - r19 * y3
                + r22 * y3
                + r78 * x3
                + 189.0 * r83 * x2
                + 60.0 * r86
                + 81.0 * r9 * y1
                + 15.0 * r94
                + 54.0 * r98)
                / 9240.0;
        self.moment_yy += -r103 * r116 / 9240.0
            - r125 * r70 / 9240.0
            - r126 * x3 / 12.0
            - 3.0 * r127 * (r26 + r38) / 3080.0
            - r128 * (r26 + r30 + x3) / 660.0
            - r4 * (r112 * x3 + r115 - 14.0 * r119 + 84.0 * r47) / 9240.0
            - r52 * r69 / 9240.0
            - r54 * (r58 + r61 + r75) / 9240.0
            - r55 * (r100 * y1 + r121 * y2 + r26 * y3 + r79 * y2 + r84 + 210.0 * x2 * y1) / 9240.0
            + x0 * (r108 * y1
                + r110 * y0
                + r111 * y0
                + r112 * y0
                + 45.0 * r125
                + 14.0 * r126
                + 126.0 * r127
                + 770.0 * r128
                + 42.0 * r129
                + r130
                + r131 * y2
                + r132 * r64
                + 135.0 * r48 * y1
                + 630.0 * r55 * y1
                + 126.0 * r55 * y2
                + 14.0 * r55 * y3
                + r63 * y3
                + r65 * y3
                + r66 * y0)
                / 9240.0
            + x1 * (27.0 * r125
                + 42.0 * r126
                + 70.0 * r129
                + r130
                + r39 * r53
                + r44 * r48
                + 27.0 * r53 * y2
                + 54.0 * r64 * y2)
                / 3080.0
            + 3.0 * x2 * y3 * (r48 + r66 + r8 * y3) / 220.0
            - y0 * (r100 * r46 + 18.0 * r114
                - 9.0 * r118
                - 27.0 * r120
                - 18.0 * r122
                - 30.0 * r123
                + r124
                + r131 * x2
                + r132 * x3 * y1
                + 162.0 * r42 * y1
                + r50
                + 63.0 * r53 * x3
                + r64 * r99)
                / 9240.0;
    }
}

/// A trait for types that can provide moment integrals for a path using Green's theorem.
pub trait ParamCurveMoments<'a> {
    /// Returns the moment integrals for the path using Green's theorem.
    fn moments(&'a self) -> Moments;
}

impl<'a, T: 'a> ParamCurveMoments<'a> for T
where
    &'a T: IntoIterator<Item = PathEl>,
{
    fn moments(&'a self) -> Moments {
        let mut moments = Moments::default();
        let mut start_pt: Point = Point::ZERO;
        let mut cur: Point = Point::ZERO;
        for el in self {
            match el {
                PathEl::MoveTo(p) => {
                    start_pt = p;
                    cur = p;
                }
                PathEl::LineTo(p) => {
                    moments.handle_line(cur, p);
                    cur = p;
                }
                PathEl::QuadTo(p0, p1) => {
                    moments.handle_quad(cur, p0, p1);
                    cur = p1;
                }
                PathEl::CurveTo(p1, p2, p3) => {
                    moments.handle_cubic(cur, p1, p2, p3);
                    cur = p3;
                }
                PathEl::ClosePath => {
                    if cur != start_pt {
                        moments.handle_line(cur, start_pt);
                        cur = start_pt;
                    }
                }
            }
        }
        moments
    }
}

#[cfg(test)]
mod tests {
    use crate::BezPath;

    use super::*;

    macro_rules! assert_approx_eq {
        ($x: expr, $y: expr) => {
            assert!(($x - $y).abs() < 1e-8, "{} != {}", $x, $y);
        };
    }

    #[test]
    fn test_moments() {
        let path = BezPath::from_vec(vec![
            PathEl::MoveTo(Point::new(0.0, 0.0)),
            PathEl::LineTo(Point::new(1.0, 0.0)),
            PathEl::LineTo(Point::new(1.0, 1.0)),
            PathEl::LineTo(Point::new(0.0, 1.0)),
            PathEl::ClosePath,
        ]);
        let moments = path.moments();
        assert_approx_eq!(moments.moment_x, 0.5);
        assert_approx_eq!(moments.moment_y, 0.5);
        assert_approx_eq!(moments.moment_xx, 0.3333333333333333);
        assert_approx_eq!(moments.moment_xy, 0.25);
        assert_approx_eq!(moments.moment_yy, 0.3333333333333333);
    }
}
