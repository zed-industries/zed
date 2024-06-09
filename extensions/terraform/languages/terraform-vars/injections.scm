; https://github.com/nvim-treesitter/nvim-treesitter/blob/ce4adf11cfe36fc5b0e5bcdce0c7c6e8fbc9798a/queries/hcl/injections.scm

(heredoc_template
  (template_literal) @content
  (heredoc_identifier) @language
  (#downcase! @language))

; https://github.com/nvim-treesitter/nvim-treesitter/blob/ce4adf11cfe36fc5b0e5bcdce0c7c6e8fbc9798a/queries/terraform/injections.scm
; inherits: hcl
