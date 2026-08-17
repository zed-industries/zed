use alloc::{borrow::ToOwned, vec};

use pp_rs::token::PreprocessorError;

use super::{
    ast::Profile,
    error::ExpectedToken,
    error::{Error, ErrorKind, ParseErrors},
    token::TokenValue,
    Frontend, Options, Span,
};
use crate::ShaderStage;

#[cfg(test)]
use std::println;

#[test]
fn version() {
    let mut frontend = Frontend::default();

    // invalid versions
    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                "#version 99000\n void main(){}",
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::InvalidVersion(99000),
                meta: Span::new(9, 14)
            }],
        },
    );

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                "#version 449\n void main(){}",
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::InvalidVersion(449),
                meta: Span::new(9, 12)
            }]
        },
    );

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                "#version 450 smart\n void main(){}",
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::InvalidProfile("smart".into()),
                meta: Span::new(13, 18),
            }]
        },
    );

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                "#version 450\nvoid main(){} #version 450",
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![
                Error {
                    kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedHash,),
                    meta: Span::new(27, 28),
                },
                Error {
                    kind: ErrorKind::InvalidToken(
                        TokenValue::Identifier("version".into()),
                        vec![ExpectedToken::Eof]
                    ),
                    meta: Span::new(28, 35)
                }
            ]
        },
    );

    // valid versions
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            "  #  version 450\nvoid main() {}",
        )
        .unwrap();
    assert_eq!(
        (frontend.metadata().version, frontend.metadata().profile),
        (450, Profile::Core)
    );

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            "#version 450\nvoid main() {}",
        )
        .unwrap();
    assert_eq!(
        (frontend.metadata().version, frontend.metadata().profile),
        (450, Profile::Core)
    );

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            "#version 450 core\nvoid main(void) {}",
        )
        .unwrap();
    assert_eq!(
        (frontend.metadata().version, frontend.metadata().profile),
        (450, Profile::Core)
    );
}

#[test]
fn control_flow() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            if (true) {
                return 1;
            } else {
                return 2;
            }
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            if (true) {
                return 1;
            }
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            int x;
            int y = 3;
            switch (5) {
                case 2:
                    x = 2;
                case 5:
                    x = 5;
                    y = 2;
                    break;
                default:
                    x = 0;
            }
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            int x = 0;
            while(x < 5) {
                x = x + 1;
            }
            do {
                x = x - 1;
            } while(x >= 4)
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            int x = 0;
            for(int i = 0; i < 10;) {
                x = x + 2;
            }
            for(;;);
            return x;
        }
        "#,
        )
        .unwrap();
}

#[test]
fn declarations() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(location = 0) in vec2 v_uv;
        layout(location = 0) out vec4 o_color;
        layout(set = 1, binding = 1) uniform texture2D tex;
        layout(set = 1, binding = 2) uniform sampler tex_sampler;

        layout(early_fragment_tests) in;

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(std140, set = 2, binding = 0)
        uniform u_locals {
            vec3 model_offs;
            float load_time;
            ivec4 atlas_offs;
        };

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(push_constant)
        uniform u_locals {
            vec3 model_offs;
            float load_time;
            ivec4 atlas_offs;
        };

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(std430, set = 2, binding = 0)
        uniform u_locals {
            vec3 model_offs;
            float load_time;
            ivec4 atlas_offs;
        };

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(std140, set = 2, binding = 0)
        uniform u_locals {
            vec3 model_offs;
            float load_time;
        } block_var;

        void main() {
            load_time * model_offs;
            block_var.load_time * block_var.model_offs;
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        float vector = vec4(1.0 / 17.0,  9.0 / 17.0,  3.0 / 17.0, 11.0 / 17.0);

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        precision highp float;

        void main() {}
        "#,
        )
        .unwrap();
}

#[test]
fn textures() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #version 450
        layout(location = 0) in vec2 v_uv;
        layout(location = 0) out vec4 o_color;
        layout(set = 1, binding = 1) uniform texture2D tex;
        layout(set = 1, binding = 2) uniform sampler tex_sampler;
        void main() {
            o_color = texture(sampler2D(tex, tex_sampler), v_uv);
            o_color.a = texture(sampler2D(tex, tex_sampler), v_uv, 2.0).a;
        }
        "#,
        )
        .unwrap();
}

#[test]
fn functions() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void test1(float);
        void test1(float) {}

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void test2(float a) {}
        void test3(float a, float b) {}
        void test4(float, float) {}

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        float test(float a) { return a; }

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        float test(vec4 p) {
            return p.x;
        }

        void main() {}
        "#,
        )
        .unwrap();

    // Function overloading
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        float test(vec2 p);
        float test(vec3 p);
        float test(vec4 p);

        float test(vec2 p) {
            return p.x;
        }

        float test(vec3 p) {
            return p.x;
        }

        float test(vec4 p) {
            return p.x;
        }

        void main() {}
        "#,
        )
        .unwrap();

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                r#"
                #  version 450
                int test(vec4 p) {
                    return p.x;
                }

                float test(vec4 p) {
                    return p.x;
                }

                void main() {}
                "#,
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::SemanticError("Function already defined".into()),
                meta: Span::new(134, 152),
            }]
        },
    );

    println!();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        float callee(uint q) {
            return float(q);
        }

        float caller() {
            callee(1u);
        }

        void main() {}
        "#,
        )
        .unwrap();

    // Nested function call
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
            #  version 450
            layout(set = 0, binding = 1) uniform texture2D t_noise;
            layout(set = 0, binding = 2) uniform sampler s_noise;

            void main() {
                textureLod(sampler2D(t_noise, s_noise), vec2(1.0), 0);
            }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void fun(vec2 in_parameter, out float out_parameter) {
            ivec2 _ = ivec2(in_parameter);
        }

        void main() {
            float a;
            fun(vec2(1.0), a);
        }
        "#,
        )
        .unwrap();
}

#[test]
fn constants() {
    use crate::{Constant, Expression, Type, TypeInner};

    let mut frontend = Frontend::default();

    let module = frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        const float a = 1.0;
        float global = a;
        const float b = a;

        void main() {}
        "#,
        )
        .unwrap();

    let mut types = module.types.iter();
    let mut constants = module.constants.iter();
    let mut global_expressions = module.global_expressions.iter();

    let (ty_handle, ty) = types.next().unwrap();
    assert_eq!(
        ty,
        &Type {
            name: None,
            inner: TypeInner::Scalar(crate::Scalar::F32)
        }
    );

    let (init_handle, init) = global_expressions.next().unwrap();
    assert_eq!(init, &Expression::Literal(crate::Literal::F32(1.0)));

    assert_eq!(
        constants.next().unwrap().1,
        &Constant {
            name: Some("a".to_owned()),
            ty: ty_handle,
            init: init_handle
        }
    );

    assert_eq!(
        constants.next().unwrap().1,
        &Constant {
            name: Some("b".to_owned()),
            ty: ty_handle,
            init: init_handle
        }
    );

    assert!(constants.next().is_none());
}

#[test]
fn function_overloading() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450

        float saturate(float v) { return clamp(v, 0.0, 1.0); }
        vec2 saturate(vec2 v) { return clamp(v, vec2(0.0), vec2(1.0)); }
        vec3 saturate(vec3 v) { return clamp(v, vec3(0.0), vec3(1.0)); }
        vec4 saturate(vec4 v) { return clamp(v, vec4(0.0), vec4(1.0)); }

        void main() {
            float v1 = saturate(1.5);
            vec2 v2 = saturate(vec2(0.5, 1.5));
            vec3 v3 = saturate(vec3(0.5, 1.5, 2.5));
            vec3 v4 = saturate(vec4(0.5, 1.5, 2.5, 3.5));
        }
        "#,
        )
        .unwrap();
}

#[test]
fn implicit_conversions() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            mat4 a = mat4(1);
            float b = 1u;
            float c = 1 + 2.0;
        }
        "#,
        )
        .unwrap();

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                r#"
                #  version 450
                void test(int a) {}
                void test(uint a) {}

                void main() {
                    test(1.0);
                }
                "#,
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::SemanticError("Unknown function \'test\'".into()),
                meta: Span::new(156, 165),
            }]
        },
    );

    assert_eq!(
        frontend
            .parse(
                &Options::from(ShaderStage::Vertex),
                r#"
                #  version 450
                void test(float a) {}
                void test(uint a) {}

                void main() {
                    test(1);
                }
                "#,
            )
            .err()
            .unwrap(),
        ParseErrors {
            errors: vec![Error {
                kind: ErrorKind::SemanticError("Ambiguous best function for \'test\'".into()),
                meta: Span::new(158, 165),
            }]
        }
    );
}

#[test]
fn structs() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        Test {
            vec4 pos;
          } xx;

        void main() {}
        "#,
        )
        .unwrap_err();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        struct Test {
            vec4 pos;
        };

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        const int NUM_VECS = 42;
        struct Test {
            vec4 vecs[NUM_VECS];
        };

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        struct Hello {
            vec4 test;
        } test() {
            return Hello( vec4(1.0) );
        }

        void main() {}
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        struct Test {};

        void main() {}
        "#,
        )
        .unwrap_err();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        inout struct Test {
            vec4 x;
        };

        void main() {}
        "#,
        )
        .unwrap_err();
}

#[test]
fn swizzles() {
    let mut frontend = Frontend::default();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            vec4 v = vec4(1);
            v.xyz = vec3(2);
            v.x = 5.0;
            v.xyz.zxy.yx.xy = vec2(5.0, 1.0);
        }
        "#,
        )
        .unwrap();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            vec4 v = vec4(1);
            v.xx = vec2(5.0);
        }
        "#,
        )
        .unwrap_err();

    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            vec3 v = vec3(1);
            v.w = 2.0;
        }
        "#,
        )
        .unwrap_err();
}

#[test]
fn expressions() {
    let mut frontend = Frontend::default();

    // Vector indexing
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        float test(int index) {
            vec4 v = vec4(1.0, 2.0, 3.0, 4.0);
            return v[index] + 1.0;
        }

        void main() {}
        "#,
        )
        .unwrap();

    // Prefix increment/decrement
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            uint index = 0;

            --index;
            ++index;
        }
        "#,
        )
        .unwrap();

    // Dynamic indexing of array
    frontend
        .parse(
            &Options::from(ShaderStage::Vertex),
            r#"
        #  version 450
        void main() {
            const vec4 positions[1] = { vec4(0) };

            gl_Position = positions[gl_VertexIndex];
        }
        "#,
        )
        .unwrap();
}
