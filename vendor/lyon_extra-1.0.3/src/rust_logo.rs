#![allow(clippy::approx_constant)]
use path::builder::SvgPathBuilder;
use path::math::{point, vector};

pub fn build_logo_path<Builder: SvgPathBuilder>(path: &mut Builder) {
    path.move_to(point(122.631, 69.716));
    path.relative_line_to(vector(-4.394, -2.72));
    path.relative_cubic_bezier_to(
        vector(-0.037, -0.428),
        vector(-0.079, -0.855),
        vector(-0.125, -1.28),
    );
    path.relative_line_to(vector(3.776, -3.522));
    path.relative_cubic_bezier_to(
        vector(0.384, -0.358),
        vector(0.556, -0.888),
        vector(0.452, -1.401),
    );
    path.relative_cubic_bezier_to(
        vector(-0.101, -0.515),
        vector(-0.462, -0.939),
        vector(-0.953, -1.122),
    );
    path.relative_line_to(vector(-4.827, -1.805));
    path.relative_cubic_bezier_to(
        vector(-0.121, -0.418),
        vector(-0.248, -0.833),
        vector(-0.378, -1.246),
    );
    path.relative_line_to(vector(3.011, -4.182));
    path.relative_cubic_bezier_to(
        vector(0.307, -0.425),
        vector(0.37, -0.978),
        vector(0.17, -1.463),
    );
    path.relative_cubic_bezier_to(
        vector(-0.2, -0.483),
        vector(-0.637, -0.829),
        vector(-1.154, -0.914),
    );
    path.relative_line_to(vector(-5.09, -0.828));
    path.relative_cubic_bezier_to(
        vector(-0.198, -0.386),
        vector(-0.404, -0.766),
        vector(-0.612, -1.143),
    );
    path.relative_line_to(vector(2.139, -4.695));
    path.relative_cubic_bezier_to(
        vector(0.219, -0.478),
        vector(0.174, -1.034),
        vector(-0.118, -1.468),
    );
    path.relative_cubic_bezier_to(
        vector(-0.291, -0.436),
        vector(-0.784, -0.691),
        vector(-1.31, -0.671),
    );
    path.relative_line_to(vector(-5.166, 0.18));
    path.relative_cubic_bezier_to(
        vector(-0.267, -0.334),
        vector(-0.539, -0.665),
        vector(-0.816, -0.99),
    );
    path.relative_line_to(vector(1.187, -5.032));
    path.relative_cubic_bezier_to(
        vector(0.12, -0.511),
        vector(-0.031, -1.046),
        vector(-0.403, -1.417),
    );
    path.relative_cubic_bezier_to(
        vector(-0.369, -0.37),
        vector(-0.905, -0.523),
        vector(-1.416, -0.403),
    );
    path.relative_line_to(vector(-5.031, 1.186));
    path.relative_cubic_bezier_to(
        vector(-0.326, -0.276),
        vector(-0.657, -0.549),
        vector(-0.992, -0.816),
    );
    path.relative_line_to(vector(0.181, -5.166));
    path.relative_cubic_bezier_to(
        vector(0.02, -0.523),
        vector(-0.235, -1.02),
        vector(-0.671, -1.31),
    );
    path.relative_cubic_bezier_to(
        vector(-0.437, -0.292),
        vector(-0.99, -0.336),
        vector(-1.467, -0.119),
    );
    path.relative_line_to(vector(-4.694, 2.14));
    path.relative_cubic_bezier_to(
        vector(-0.379, -0.208),
        vector(-0.759, -0.414),
        vector(-1.143, -0.613),
    );
    path.relative_line_to(vector(-0.83, -5.091));
    path.relative_cubic_bezier_to(
        vector(-0.084, -0.516),
        vector(-0.43, -0.954),
        vector(-0.914, -1.154),
    );
    path.relative_cubic_bezier_to(
        vector(-0.483, -0.201),
        vector(-1.037, -0.136),
        vector(-1.462, 0.17),
    );
    path.relative_line_to(vector(-4.185, 3.011));
    path.relative_cubic_bezier_to(
        vector(-0.412, -0.131),
        vector(-0.826, -0.257),
        vector(-1.244, -0.377),
    );
    path.relative_line_to(vector(-1.805, -4.828));
    path.relative_cubic_bezier_to(
        vector(-0.183, -0.492),
        vector(-0.607, -0.853),
        vector(-1.122, -0.955),
    );
    path.relative_cubic_bezier_to(
        vector(-0.514, -0.101),
        vector(-1.043, 0.07),
        vector(-1.4, 0.452),
    );
    path.relative_line_to(vector(-3.522, 3.779));
    path.relative_cubic_bezier_to(
        vector(-0.425, -0.047),
        vector(-0.853, -0.09),
        vector(-1.28, -0.125),
    );
    path.relative_line_to(vector(-2.72, -4.395));
    path.relative_cubic_bezier_to(
        vector(-0.275, -0.445),
        vector(-0.762, -0.716),
        vector(-1.286, -0.716),
    );
    path.smooth_relative_cubic_bezier_to(vector(-1.011, 0.271), vector(-1.285, 0.716));
    path.relative_line_to(vector(-2.72, 4.395));
    path.relative_cubic_bezier_to(
        vector(-0.428, 0.035),
        vector(-0.856, 0.078),
        vector(-1.281, 0.125),
    );
    path.relative_line_to(vector(-3.523, -3.779));
    path.relative_cubic_bezier_to(
        vector(-0.357, -0.382),
        vector(-0.887, -0.553),
        vector(-1.4, -0.452),
    );
    path.relative_cubic_bezier_to(
        vector(-0.515, 0.103),
        vector(-0.939, 0.463),
        vector(-1.122, 0.955),
    );
    path.relative_line_to(vector(-1.805, 4.828));
    path.relative_cubic_bezier_to(
        vector(-0.418, 0.12),
        vector(-0.832, 0.247),
        vector(-1.245, 0.377),
    );
    path.relative_line_to(vector(-4.184, -3.011));
    path.relative_cubic_bezier_to(
        vector(-0.425, -0.307),
        vector(-0.979, -0.372),
        vector(-1.463, -0.17),
    );
    path.relative_cubic_bezier_to(
        vector(-0.483, 0.2),
        vector(-0.83, 0.638),
        vector(-0.914, 1.154),
    );
    path.relative_line_to(vector(-0.83, 5.091));
    path.relative_cubic_bezier_to(
        vector(-0.384, 0.199),
        vector(-0.764, 0.404),
        vector(-1.143, 0.613),
    );
    path.relative_line_to(vector(-4.694, -2.14));
    path.relative_cubic_bezier_to(
        vector(-0.477, -0.218),
        vector(-1.033, -0.173),
        vector(-1.467, 0.119),
    );
    path.relative_cubic_bezier_to(
        vector(-0.436, 0.29),
        vector(-0.691, 0.787),
        vector(-0.671, 1.31),
    );
    path.relative_line_to(vector(0.18, 5.166));
    path.relative_cubic_bezier_to(
        vector(-0.334, 0.267),
        vector(-0.665, 0.54),
        vector(-0.992, 0.816),
    );
    path.relative_line_to(vector(-5.031, -1.186));
    path.relative_cubic_bezier_to(
        vector(-0.511, -0.119),
        vector(-1.047, 0.033),
        vector(-1.417, 0.403),
    );
    path.relative_cubic_bezier_to(
        vector(-0.372, 0.371),
        vector(-0.523, 0.906),
        vector(-0.403, 1.417),
    );
    path.relative_line_to(vector(1.185, 5.032));
    path.relative_cubic_bezier_to(
        vector(-0.275, 0.326),
        vector(-0.547, 0.656),
        vector(-0.814, 0.99),
    );
    path.relative_line_to(vector(-5.166, -0.18));
    path.relative_cubic_bezier_to(
        vector(-0.521, -0.015),
        vector(-1.019, 0.235),
        vector(-1.31, 0.671),
    );
    path.relative_cubic_bezier_to(
        vector(-0.292, 0.434),
        vector(-0.336, 0.99),
        vector(-0.119, 1.468),
    );
    path.relative_line_to(vector(2.14, 4.695));
    path.relative_cubic_bezier_to(
        vector(-0.208, 0.377),
        vector(-0.414, 0.757),
        vector(-0.613, 1.143),
    );
    path.relative_line_to(vector(-5.09, 0.828));
    path.relative_cubic_bezier_to(
        vector(-0.517, 0.084),
        vector(-0.953, 0.43),
        vector(-1.154, 0.914),
    );
    path.relative_cubic_bezier_to(
        vector(-0.2, 0.485),
        vector(-0.135, 1.038),
        vector(0.17, 1.463),
    );
    path.relative_line_to(vector(3.011, 4.182));
    path.relative_cubic_bezier_to(
        vector(-0.131, 0.413),
        vector(-0.258, 0.828),
        vector(-0.378, 1.246),
    );
    path.relative_line_to(vector(-4.828, 1.805));
    path.relative_cubic_bezier_to(
        vector(-0.49, 0.183),
        vector(-0.851, 0.607),
        vector(-0.953, 1.122),
    );
    path.relative_cubic_bezier_to(
        vector(-0.102, 0.514),
        vector(0.069, 1.043),
        vector(0.452, 1.401),
    );
    path.relative_line_to(vector(3.777, 3.522));
    path.relative_cubic_bezier_to(
        vector(-0.047, 0.425),
        vector(-0.089, 0.853),
        vector(-0.125, 1.28),
    );
    path.relative_line_to(vector(-4.394, 2.72));
    path.relative_cubic_bezier_to(
        vector(-0.445, 0.275),
        vector(-0.716, 0.761),
        vector(-0.716, 1.286),
    );
    path.smooth_relative_cubic_bezier_to(vector(0.271, 1.011), vector(0.716, 1.285));
    path.relative_line_to(vector(4.394, 2.72));
    path.relative_cubic_bezier_to(
        vector(0.036, 0.428),
        vector(0.078, 0.855),
        vector(0.125, 1.28),
    );
    path.relative_line_to(vector(-3.777, 3.523));
    path.relative_cubic_bezier_to(
        vector(-0.383, 0.357),
        vector(-0.554, 0.887),
        vector(-0.452, 1.4),
    );
    path.relative_cubic_bezier_to(
        vector(0.102, 0.515),
        vector(0.463, 0.938),
        vector(0.953, 1.122),
    );
    path.relative_line_to(vector(4.828, 1.805));
    path.relative_cubic_bezier_to(
        vector(0.12, 0.418),
        vector(0.247, 0.833),
        vector(0.378, 1.246),
    );
    path.relative_line_to(vector(-3.011, 4.183));
    path.relative_cubic_bezier_to(
        vector(-0.306, 0.426),
        vector(-0.371, 0.979),
        vector(-0.17, 1.462),
    );
    path.relative_cubic_bezier_to(
        vector(0.201, 0.485),
        vector(0.638, 0.831),
        vector(1.155, 0.914),
    );
    path.relative_line_to(vector(5.089, 0.828));
    path.relative_cubic_bezier_to(
        vector(0.199, 0.386),
        vector(0.403, 0.766),
        vector(0.613, 1.145),
    );
    path.relative_line_to(vector(-2.14, 4.693));
    path.relative_cubic_bezier_to(
        vector(-0.218, 0.477),
        vector(-0.173, 1.032),
        vector(0.119, 1.468),
    );
    path.relative_cubic_bezier_to(
        vector(0.292, 0.437),
        vector(0.789, 0.692),
        vector(1.31, 0.671),
    );
    path.relative_line_to(vector(5.164, -0.181));
    path.relative_cubic_bezier_to(
        vector(0.269, 0.336),
        vector(0.54, 0.665),
        vector(0.816, 0.992),
    );
    path.relative_line_to(vector(-1.185, 5.033));
    path.relative_cubic_bezier_to(
        vector(-0.12, 0.51),
        vector(0.031, 1.043),
        vector(0.403, 1.414),
    );
    path.relative_cubic_bezier_to(
        vector(0.369, 0.373),
        vector(0.906, 0.522),
        vector(1.417, 0.402),
    );
    path.relative_line_to(vector(5.031, -1.185));
    path.relative_cubic_bezier_to(
        vector(0.327, 0.278),
        vector(0.658, 0.548),
        vector(0.992, 0.814),
    );
    path.relative_line_to(vector(-0.18, 5.167));
    path.relative_cubic_bezier_to(
        vector(-0.02, 0.523),
        vector(0.235, 1.019),
        vector(0.671, 1.311),
    );
    path.relative_cubic_bezier_to(
        vector(0.434, 0.291),
        vector(0.99, 0.335),
        vector(1.467, 0.117),
    );
    path.relative_line_to(vector(4.694, -2.139));
    path.relative_cubic_bezier_to(
        vector(0.378, 0.21),
        vector(0.758, 0.414),
        vector(1.143, 0.613),
    );
    path.relative_line_to(vector(0.83, 5.088));
    path.relative_cubic_bezier_to(
        vector(0.084, 0.518),
        vector(0.43, 0.956),
        vector(0.914, 1.155),
    );
    path.relative_cubic_bezier_to(
        vector(0.483, 0.201),
        vector(1.038, 0.136),
        vector(1.463, -0.169),
    );
    path.relative_line_to(vector(4.182, -3.013));
    path.relative_cubic_bezier_to(
        vector(0.413, 0.131),
        vector(0.828, 0.259),
        vector(1.246, 0.379),
    );
    path.relative_line_to(vector(1.805, 4.826));
    path.relative_cubic_bezier_to(
        vector(0.183, 0.49),
        vector(0.607, 0.853),
        vector(1.122, 0.953),
    );
    path.relative_cubic_bezier_to(
        vector(0.514, 0.104),
        vector(1.043, -0.068),
        vector(1.4, -0.452),
    );
    path.relative_line_to(vector(3.523, -3.777));
    path.relative_cubic_bezier_to(
        vector(0.425, 0.049),
        vector(0.853, 0.09),
        vector(1.281, 0.128),
    );
    path.relative_line_to(vector(2.72, 4.394));
    path.relative_cubic_bezier_to(
        vector(0.274, 0.443),
        vector(0.761, 0.716),
        vector(1.285, 0.716),
    );
    path.smooth_relative_cubic_bezier_to(vector(1.011, -0.272), vector(1.286, -0.716));
    path.relative_line_to(vector(2.72, -4.394));
    path.relative_cubic_bezier_to(
        vector(0.428, -0.038),
        vector(0.855, -0.079),
        vector(1.28, -0.128),
    );
    path.relative_line_to(vector(3.522, 3.777));
    path.relative_cubic_bezier_to(
        vector(0.357, 0.384),
        vector(0.887, 0.556),
        vector(1.4, 0.452),
    );
    path.relative_cubic_bezier_to(
        vector(0.515, -0.101),
        vector(0.939, -0.463),
        vector(1.122, -0.953),
    );
    path.relative_line_to(vector(1.805, -4.826));
    path.relative_cubic_bezier_to(
        vector(0.418, -0.12),
        vector(0.833, -0.248),
        vector(1.246, -0.379),
    );
    path.relative_line_to(vector(4.183, 3.013));
    path.relative_cubic_bezier_to(
        vector(0.425, 0.305),
        vector(0.979, 0.37),
        vector(1.462, 0.169),
    );
    path.relative_cubic_bezier_to(
        vector(0.484, -0.199),
        vector(0.83, -0.638),
        vector(0.914, -1.155),
    );
    path.relative_line_to(vector(0.83, -5.088));
    path.relative_cubic_bezier_to(
        vector(0.384, -0.199),
        vector(0.764, -0.406),
        vector(1.143, -0.613),
    );
    path.relative_line_to(vector(4.694, 2.139));
    path.relative_cubic_bezier_to(
        vector(0.477, 0.218),
        vector(1.032, 0.174),
        vector(1.467, -0.117),
    );
    path.relative_cubic_bezier_to(
        vector(0.436, -0.292),
        vector(0.69, -0.787),
        vector(0.671, -1.311),
    );
    path.relative_line_to(vector(-0.18, -5.167));
    path.relative_cubic_bezier_to(
        vector(0.334, -0.267),
        vector(0.665, -0.536),
        vector(0.991, -0.814),
    );
    path.relative_line_to(vector(5.031, 1.185));
    path.relative_cubic_bezier_to(
        vector(0.511, 0.12),
        vector(1.047, -0.029),
        vector(1.416, -0.402),
    );
    path.relative_cubic_bezier_to(
        vector(0.372, -0.371),
        vector(0.523, -0.904),
        vector(0.403, -1.414),
    );
    path.relative_line_to(vector(-1.185, -5.033));
    path.relative_cubic_bezier_to(
        vector(0.276, -0.327),
        vector(0.548, -0.656),
        vector(0.814, -0.992),
    );
    path.relative_line_to(vector(5.166, 0.181));
    path.relative_cubic_bezier_to(
        vector(0.521, 0.021),
        vector(1.019, -0.234),
        vector(1.31, -0.671),
    );
    path.relative_cubic_bezier_to(
        vector(0.292, -0.436),
        vector(0.337, -0.991),
        vector(0.118, -1.468),
    );
    path.relative_line_to(vector(-2.139, -4.693));
    path.relative_cubic_bezier_to(
        vector(0.209, -0.379),
        vector(0.414, -0.759),
        vector(0.612, -1.145),
    );
    path.relative_line_to(vector(5.09, -0.828));
    path.relative_cubic_bezier_to(
        vector(0.518, -0.083),
        vector(0.954, -0.429),
        vector(1.154, -0.914),
    );
    path.relative_cubic_bezier_to(
        vector(0.2, -0.483),
        vector(0.137, -1.036),
        vector(-0.17, -1.462),
    );
    path.relative_line_to(vector(-3.011, -4.183));
    path.relative_cubic_bezier_to(
        vector(0.13, -0.413),
        vector(0.257, -0.828),
        vector(0.378, -1.246),
    );
    path.relative_line_to(vector(4.827, -1.805));
    path.relative_cubic_bezier_to(
        vector(0.491, -0.184),
        vector(0.853, -0.607),
        vector(0.953, -1.122),
    );
    path.relative_cubic_bezier_to(
        vector(0.104, -0.514),
        vector(-0.068, -1.043),
        vector(-0.452, -1.4),
    );
    path.relative_line_to(vector(-3.776, -3.523));
    path.relative_cubic_bezier_to(
        vector(0.046, -0.425),
        vector(0.088, -0.853),
        vector(0.125, -1.28),
    );
    path.relative_line_to(vector(4.394, -2.72));
    path.relative_cubic_bezier_to(
        vector(0.445, -0.274),
        vector(0.716, -0.761),
        vector(0.716, -1.285),
    );
    path.smooth_cubic_bezier_to(point(123.076, 69.991), point(122.631, 69.716));
    path.close();

    path.move_to(point(93.222, 106.167));
    path.relative_cubic_bezier_to(
        vector(-1.678, -0.362),
        vector(-2.745, -2.016),
        vector(-2.385, -3.699),
    );
    path.relative_cubic_bezier_to(
        vector(0.359, -1.681),
        vector(2.012, -2.751),
        vector(3.689, -2.389),
    );
    path.relative_cubic_bezier_to(
        vector(1.678, 0.359),
        vector(2.747, 2.016),
        vector(2.387, 3.696),
    );
    path.smooth_cubic_bezier_to(point(94.899, 106.526), point(93.222, 106.167));
    path.close();

    path.move_to(point(91.729, 96.069));
    path.relative_cubic_bezier_to(
        vector(-1.531, -0.328),
        vector(-3.037, 0.646),
        vector(-3.365, 2.18),
    );
    path.relative_line_to(vector(-1.56, 7.28));
    path.relative_cubic_bezier_to(
        vector(-4.814, 2.185),
        vector(-10.16, 3.399),
        vector(-15.79, 3.399),
    );
    path.relative_cubic_bezier_to(
        vector(-5.759, 0.0),
        vector(-11.221, -1.274),
        vector(-16.121, -3.552),
    );
    path.relative_line_to(vector(-1.559, -7.28));
    path.relative_cubic_bezier_to(
        vector(-0.328, -1.532),
        vector(-1.834, -2.508),
        vector(-3.364, -2.179),
    );
    path.relative_line_to(vector(-6.427, 1.38));
    path.relative_cubic_bezier_to(
        vector(-1.193, -1.228),
        vector(-2.303, -2.536),
        vector(-3.323, -3.917),
    );
    path.relative_horizontal_line_to(31.272);
    path.relative_cubic_bezier_to(
        vector(0.354, 0.0),
        vector(0.59, -0.064),
        vector(0.59, -0.386),
    );
    path.vertical_line_to(81.932);
    path.relative_cubic_bezier_to(
        vector(0.0, -0.322),
        vector(-0.236, -0.386),
        vector(-0.59, -0.386),
    );
    path.relative_horizontal_line_to(-9.146);
    path.relative_vertical_line_to(-7.012);
    path.relative_horizontal_line_to(9.892);
    path.relative_cubic_bezier_to(
        vector(0.903, 0.0),
        vector(4.828, 0.258),
        vector(6.083, 5.275),
    );
    path.relative_cubic_bezier_to(
        vector(0.393, 1.543),
        vector(1.256, 6.562),
        vector(1.846, 8.169),
    );
    path.relative_cubic_bezier_to(
        vector(0.588, 1.802),
        vector(2.982, 5.402),
        vector(5.533, 5.402),
    );
    path.relative_horizontal_line_to(15.583);
    path.relative_cubic_bezier_to(
        vector(0.177, 0.0),
        vector(0.366, -0.02),
        vector(0.565, -0.056),
    );
    path.relative_cubic_bezier_to(
        vector(-1.081, 1.469),
        vector(-2.267, 2.859),
        vector(-3.544, 4.158),
    );
    path.line_to(point(91.729, 96.069));
    path.close();

    path.move_to(point(48.477, 106.015));
    path.relative_cubic_bezier_to(
        vector(-1.678, 0.362),
        vector(-3.33, -0.708),
        vector(-3.691, -2.389),
    );
    path.relative_cubic_bezier_to(
        vector(-0.359, -1.684),
        vector(0.708, -3.337),
        vector(2.386, -3.699),
    );
    path.relative_cubic_bezier_to(
        vector(1.678, -0.359),
        vector(3.331, 0.711),
        vector(3.691, 2.392),
    );
    path.cubic_bezier_to(
        point(51.222, 103.999),
        point(50.154, 105.655),
        point(48.477, 106.015),
    );
    path.close();

    path.move_to(point(36.614, 57.91));
    path.relative_cubic_bezier_to(
        vector(0.696, 1.571),
        vector(-0.012, 3.412),
        vector(-1.581, 4.107),
    );
    path.relative_cubic_bezier_to(
        vector(-1.569, 0.697),
        vector(-3.405, -0.012),
        vector(-4.101, -1.584),
    );
    path.relative_cubic_bezier_to(
        vector(-0.696, -1.572),
        vector(0.012, -3.41),
        vector(1.581, -4.107),
    );
    path.cubic_bezier_to(
        point(34.083, 55.63),
        point(35.918, 56.338),
        point(36.614, 57.91),
    );
    path.close();

    path.move_to(point(32.968, 66.553));
    path.relative_line_to(vector(6.695, -2.975));
    path.relative_cubic_bezier_to(
        vector(1.43, -0.635),
        vector(2.076, -2.311),
        vector(1.441, -3.744),
    );
    path.relative_line_to(vector(-1.379, -3.118));
    path.relative_horizontal_line_to(5.423);
    path.vertical_line_to(81.16);
    path.horizontal_line_to(34.207);
    path.relative_cubic_bezier_to(
        vector(-0.949, -3.336),
        vector(-1.458, -6.857),
        vector(-1.458, -10.496),
    );
    path.cubic_bezier_to(
        point(32.749, 69.275),
        point(32.824, 67.902),
        point(32.968, 66.553),
    );
    path.close();

    path.move_to(point(62.348, 64.179));
    path.relative_vertical_line_to(-7.205);
    path.relative_horizontal_line_to(12.914);
    path.relative_cubic_bezier_to(vector(0.667, 0.0), vector(4.71, 0.771), vector(4.71, 3.794));
    path.relative_cubic_bezier_to(
        vector(0.0, 2.51),
        vector(-3.101, 3.41),
        vector(-5.651, 3.41),
    );
    path.horizontal_line_to(62.348);
    path.close();

    path.move_to(point(109.28, 70.664));
    path.relative_cubic_bezier_to(
        vector(0.0, 0.956),
        vector(-0.035, 1.902),
        vector(-0.105, 2.841),
    );
    path.relative_horizontal_line_to(-3.926);
    path.relative_cubic_bezier_to(
        vector(-0.393, 0.0),
        vector(-0.551, 0.258),
        vector(-0.551, 0.643),
    );
    path.relative_vertical_line_to(1.803);
    path.relative_cubic_bezier_to(
        vector(0.0, 4.244),
        vector(-2.393, 5.167),
        vector(-4.49, 5.402),
    );
    path.relative_cubic_bezier_to(
        vector(-1.997, 0.225),
        vector(-4.211, -0.836),
        vector(-4.484, -2.058),
    );
    path.relative_cubic_bezier_to(
        vector(-1.178, -6.626),
        vector(-3.141, -8.041),
        vector(-6.241, -10.486),
    );
    path.relative_cubic_bezier_to(
        vector(3.847, -2.443),
        vector(7.85, -6.047),
        vector(7.85, -10.871),
    );
    path.relative_cubic_bezier_to(
        vector(0.0, -5.209),
        vector(-3.571, -8.49),
        vector(-6.005, -10.099),
    );
    path.relative_cubic_bezier_to(
        vector(-3.415, -2.251),
        vector(-7.196, -2.702),
        vector(-8.216, -2.702),
    );
    path.horizontal_line_to(42.509);
    path.relative_cubic_bezier_to(
        vector(5.506, -6.145),
        vector(12.968, -10.498),
        vector(21.408, -12.082),
    );
    path.relative_line_to(vector(4.786, 5.021));
    path.relative_cubic_bezier_to(
        vector(1.082, 1.133),
        vector(2.874, 1.175),
        vector(4.006, 0.092),
    );
    path.relative_line_to(vector(5.355, -5.122));
    path.relative_cubic_bezier_to(
        vector(11.221, 2.089),
        vector(20.721, 9.074),
        vector(26.196, 18.657),
    );
    path.relative_line_to(vector(-3.666, 8.28));
    path.relative_cubic_bezier_to(
        vector(-0.633, 1.433),
        vector(0.013, 3.109),
        vector(1.442, 3.744),
    );
    path.relative_line_to(vector(7.058, 3.135));
    path.cubic_bezier_to(
        point(109.216, 68.115),
        point(109.28, 69.381),
        point(109.28, 70.664),
    );
    path.close();

    path.move_to(point(68.705, 28.784));
    path.relative_cubic_bezier_to(
        vector(1.24, -1.188),
        vector(3.207, -1.141),
        vector(4.394, 0.101),
    );
    path.relative_cubic_bezier_to(
        vector(1.185, 1.245),
        vector(1.14, 3.214),
        vector(-0.103, 4.401),
    );
    path.relative_cubic_bezier_to(
        vector(-1.24, 1.188),
        vector(-3.207, 1.142),
        vector(-4.394, -0.102),
    );
    path.cubic_bezier_to(
        point(67.418, 31.941),
        point(67.463, 29.972),
        point(68.705, 28.784),
    );
    path.close();

    path.move_to(point(105.085, 58.061));
    path.relative_cubic_bezier_to(
        vector(0.695, -1.571),
        vector(2.531, -2.28),
        vector(4.1, -1.583),
    );
    path.relative_cubic_bezier_to(
        vector(1.569, 0.696),
        vector(2.277, 2.536),
        vector(1.581, 4.107),
    );
    path.relative_cubic_bezier_to(
        vector(-0.695, 1.572),
        vector(-2.531, 2.281),
        vector(-4.101, 1.584),
    );
    path.cubic_bezier_to(
        point(105.098, 61.473),
        point(104.39, 59.634),
        point(105.085, 58.061),
    );
    path.close();
}
