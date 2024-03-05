; Dockerfile instructions set taken from:
; https://docs.docker.com/engine/reference/builder/#overview
; https://github.com/helix-editor/helix/blob/78c34194b5c83beb26ca04f12bf9d53fd5aba801/runtime/queries/dockerfile/highlights.scm
[
	"ADD"
	"ARG"
	"CMD"
	"COPY"
	"ENTRYPOINT"
	"ENV"
	"EXPOSE"
	"FROM"
	"HEALTHCHECK"
	"LABEL"
	"MAINTAINER"
	"ONBUILD"
	"RUN"
	"SHELL"
	"STOPSIGNAL"
	"USER"
	"VOLUME"
	"WORKDIR"

	; "as" for multi-stage builds
	"AS"
] @keyword

[
	":"
	"@"
] @operator

(comment) @comment

(image_spec
	(image_tag
		":" @punctuation.special)
	(image_digest
		"@" @punctuation.special))

[
  (double_quoted_string)
  (single_quoted_string)
  (json_string)
] @string

[
  (env_pair)
  (label_pair)
] @constant

[
  (param)
  (mount_param)
] @function

(expansion
    [
        "$"
        "{"
        "}"
    ] @punctuation.special
) @constant
